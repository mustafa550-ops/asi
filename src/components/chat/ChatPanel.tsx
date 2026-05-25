import { useRef, useEffect } from "react";
import { useChatStore } from "../../stores/chatStore";

export default function ChatPanel() {
  const messages = useChatStore((s) => s.messages);
  const input = useChatStore((s) => s.input);
  const loading = useChatStore((s) => s.loading);
  const send = useChatStore((s) => s.send);
  const setInput = useChatStore((s) => s.setInput);

  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  return (
    <div className="chat-panel">
      <div className="chat-messages">
        {messages.map((m, i) => (
          <div key={i} className={`message ${m.role}`}>
            <strong>{m.role === "adler" ? "🔹 ADLER" : "👤 Sen"}</strong>
            <pre style={{ whiteSpace: "pre-wrap", fontFamily: "inherit", margin: "4px 0" }}>{m.content}</pre>
          </div>
        ))}
        {loading && <div className="message adler"><em>ADLER düşünüyor...</em></div>}
        <div ref={endRef} />
      </div>
      <div className="chat-input">
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && send()}
          placeholder="Komut yaz veya soru sor..."
          disabled={loading}
        />
        <button onClick={send} disabled={loading}>
          Gönder
        </button>
      </div>
    </div>
  );
}
