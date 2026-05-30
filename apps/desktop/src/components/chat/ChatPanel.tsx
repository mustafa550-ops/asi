import { useRef, useEffect, useState, useCallback } from "react";
import { useChatStore } from "../../stores/chatStore";
import { MarkdownRenderer } from "../MarkdownRenderer";
import { TypingIndicator } from "./TypingIndicator";
import { SlashCommands } from "./SlashCommands";
import { ChatHistory } from "./ChatHistory";
import { ContextWindow } from "./ContextWindow";
import { FileAttachment } from "./FileAttachment";
import { ProactiveAlert } from "./ProactiveAlert";
import ApprovalPanel from "../approval-panels/ApprovalPanel";
import { invoke } from "../../lib/tauri";

interface RagSource {
  title: string;
  source: string;
  relevance: number;
}

export default function ChatPanel() {
  const messages = useChatStore((s) => s.messages);
  const input = useChatStore((s) => s.input);
  const loading = useChatStore((s) => s.loading);
  const send = useChatStore((s) => s.send);
  const setInput = useChatStore((s) => s.setInput);
  const messageCount = useChatStore((s) => s.messages.length);
  const lastMessage = useChatStore((s) => s.messages[s.messages.length - 1]);

  const [showHistory, setShowHistory] = useState(false);
  const [showSlash, setShowSlash] = useState(false);
  const [sources, setSources] = useState<RagSource[]>([]);
  const [sessions, setSessions] = useState<{ id: string; title: string; date: string }[]>([]);
  const [activeSession, setActiveSession] = useState<string | null>(null);
  const [attachedFiles, setAttachedFiles] = useState<{ name: string; content: string }[]>([]);

  const endRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    invoke<string>("list_chat_sessions", { limit: 50 })
      .then((raw) => {
        const list: { id: string; title: string; created_at: string }[] = JSON.parse(raw);
        setSessions(list.map((s) => ({ id: s.id, title: s.title, date: s.created_at })));
      })
      .catch(() => {});
  }, [messageCount]);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  useEffect(() => {
    if (messageCount > 0 && lastMessage?.role === "user") {
      invoke<string>("hybrid_search", { query: lastMessage.content, limit: 3 })
        .then((raw) => {
          const results: { content: string; source: string; score: number; method: string }[] = JSON.parse(raw);
          setSources(results.map((r) => ({
            title: r.content.length > 40 ? r.content.slice(0, 40) + "..." : r.content,
            source: r.source,
            relevance: r.score,
          })));
        })
        .catch(() => {});
    }
  }, [messageCount, lastMessage]);

  const handleApprovalResult = useCallback((action: "approve" | "reject", summary: string) => {
    const msg = action === "approve"
      ? `✅ Onaylandı: ${summary}`
      : `❌ Reddedildi: ${summary}`;
    useChatStore.getState().addMessage?.("system", msg);
  }, []);

  const handleSlashSelect = useCallback((cmd: string) => {
    setInput(cmd + " ");
    setShowSlash(false);
    inputRef.current?.focus();
  }, [setInput]);

  const handleInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const val = e.target.value;
    setInput(val);
    if (val === "/") {
      setShowSlash(true);
    } else {
      setShowSlash(false);
    }
  }, [setInput]);

  const handleKeyDown = useCallback((e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && !showSlash) {
      send();
    }
  }, [send, showSlash]);

  const handleAttach = useCallback((file: { name: string; size: number; content: string }) => {
    setAttachedFiles((prev) => [...prev, { name: file.name, content: file.content }]);
  }, []);

  return (
    <div className="chat-panel">
      <ProactiveAlert />
      <div className="chat-header">
        <span className="chat-title">Sohbet</span>
        <button className="chat-history-btn" onClick={() => setShowHistory(!showHistory)}>
          {showHistory ? "Gizle" : "Geçmiş"}
        </button>
      </div>
      {showHistory && (
        <ChatHistory
          sessions={sessions}
          activeId={activeSession}
          onSelect={(id) => setActiveSession(id)}
          onDelete={(id) => {
            invoke("delete_chat_session", { sessionId: id }).catch(() => {});
            setSessions((prev) => prev.filter((s) => s.id !== id));
          }}
        />
      )}
      <div className="chat-messages" role="log" aria-label="Sohbet mesajları">
        {messages.map((m, i) => (
          <div key={i} className={`message ${m.role}`}>
            <strong style={{ fontSize: "0.75rem", display: "block", marginBottom: 4 }}>
              {m.role === "adler" ? "ADLER" : m.role === "system" ? "Sistem" : "Sen"}
            </strong>
            {m.role === "system" ? (
              <span style={{ fontSize: "0.8rem", color: "#8b949e", fontStyle: "italic" }}>{m.content}</span>
            ) : m.role === "adler" ? (
              <MarkdownRenderer content={m.content} />
            ) : (
              <pre style={{ whiteSpace: "pre-wrap", fontFamily: "inherit", margin: 0, fontSize: "0.85rem" }}>{m.content}</pre>
            )}
          </div>
        ))}
        {loading && <TypingIndicator />}
        <div ref={endRef} />
      </div>
      <ApprovalPanel onResult={handleApprovalResult} />
      <ContextWindow sources={sources} />
      <div className="chat-input" style={{ position: "relative" }}>
        {showSlash && (
          <SlashCommands
            input={input}
            onSelect={handleSlashSelect}
            onClose={() => setShowSlash(false)}
          />
        )}
        <FileAttachment onAttach={handleAttach} />
        {attachedFiles.length > 0 && (
          <div style={{ fontSize: "0.7rem", color: "#8b949e", padding: "2px 0" }}>
            {attachedFiles.map((f, i) => (
              <span key={i} style={{ marginRight: 8 }}>
                📄 {f.name}
                <button
                  style={{ background: "none", border: "none", color: "#f85149", cursor: "pointer", marginLeft: 4 }}
                  onClick={() => setAttachedFiles((p) => p.filter((_, j) => j !== i))}
                >
                  &times;
                </button>
              </span>
            ))}
          </div>
        )}
        <input
          ref={inputRef}
          value={input}
          onChange={handleInputChange}
          onKeyDown={handleKeyDown}
          placeholder="Komut yaz veya soru sor... (/ ile komutları gör)"
          disabled={loading}
          aria-label="Mesaj girişi"
        />
        <button onClick={send} disabled={loading} aria-label="Gönder">
          Gönder
        </button>
      </div>
    </div>
  );
}
