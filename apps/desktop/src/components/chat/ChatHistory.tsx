interface Session {
  id: string;
  title: string;
  date: string;
}

interface ChatHistoryProps {
  sessions: Session[];
  activeId: string | null;
  onSelect: (id: string) => void;
  onDelete: (id: string) => void;
}

export function ChatHistory({
  sessions,
  activeId,
  onSelect,
  onDelete,
}: ChatHistoryProps) {
  if (sessions.length === 0) return null;

  return (
    <div
      className="chat-session-list"
      role="listbox"
      aria-label="Sohbet geçmişi"
    >
      {sessions.map((s) => (
        <div
          key={s.id}
          className={`chat-session-item ${activeId === s.id ? "active" : ""}`}
          role="option"
          aria-selected={activeId === s.id}
        >
          <div className="chat-session-title" onClick={() => onSelect(s.id)}>
            <span>{s.title}</span>
            <span style={{ fontSize: "0.7rem", color: "#8b949e" }}>
              {s.date}
            </span>
          </div>
          <button
            className="chat-session-delete"
            onClick={(e) => {
              e.stopPropagation();
              onDelete(s.id);
            }}
            aria-label={`${s.title} sil`}
            title="Sil"
          >
            &times;
          </button>
        </div>
      ))}
    </div>
  );
}
