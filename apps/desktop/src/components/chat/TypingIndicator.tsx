export function TypingIndicator() {
  return (
    <div className="message adler chat-typing" role="status" aria-label="ADLER düşünüyor">
      <span className="typing-dots">
        <span className="typing-dot" /><span className="typing-dot" /><span className="typing-dot" />
      </span>
      <span style={{ marginLeft: 8, fontSize: "0.8rem", color: "#8b949e" }}>ADLER düşünüyor...</span>
    </div>
  );
}
