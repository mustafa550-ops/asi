import { useState, useRef, useEffect } from "react";
import { invoke } from "../../lib/tauri";

interface Message {
  role: "user" | "adler";
  content: string;
}

export default function ChatPanel() {
  const [messages, setMessages] = useState<Message[]>([
    { role: "adler", content: "ADLER ASI hazır. Nasıl yardımcı olabilirim?" },
  ]);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);
  const endRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  const send = async () => {
    const text = input.trim();
    if (!text || loading) return;

    setInput("");
    setMessages((p) => [...p, { role: "user", content: text }]);
    setLoading(true);

    try {
      const response: string = await invoke("send_command", { command: text });
      setMessages((p) => [...p, { role: "adler", content: response }]);
    } catch (err) {
      setMessages((p) => [...p, { role: "adler", content: `Hata: ${err}` }]);
    }
    setLoading(false);
  };

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
