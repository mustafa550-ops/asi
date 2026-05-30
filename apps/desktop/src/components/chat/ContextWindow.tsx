interface ContextSource {
  title: string;
  source: string;
  relevance: number;
}

interface ContextWindowProps {
  sources: ContextSource[];
}

export function ContextWindow({ sources }: ContextWindowProps) {
  if (sources.length === 0) return null;

  return (
    <div className="chat-context" role="region" aria-label="Kullanılan kaynaklar">
      <strong style={{ fontSize: "0.7rem" }}>Kaynaklar:</strong>
      {sources.map((s, i) => (
        <span key={i} title={`Kaynak: ${s.source} (ilgi: ${(s.relevance * 100).toFixed(0)}%)`}>
          {s.title}{i < sources.length - 1 ? ", " : ""}
        </span>
      ))}
    </div>
  );
}
