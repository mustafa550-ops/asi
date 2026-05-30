import { Badge } from "../ui/Badge";

interface AgentCardProps {
  name: string;
  status: "ready" | "busy" | "error" | "idle";
  description: string;
  lastAction?: string;
  runCount?: number;
}

const STATUS_MAP: Record<
  string,
  { variant: "success" | "warning" | "error" | "info"; label: string }
> = {
  ready: { variant: "success", label: "Hazır" },
  idle: { variant: "info", label: "Boşta" },
  busy: { variant: "warning", label: "Meşgul" },
  error: { variant: "error", label: "Hata" },
};

export function AgentCard({
  name,
  status,
  description,
  lastAction,
  runCount,
}: AgentCardProps) {
  const s = STATUS_MAP[status] ?? STATUS_MAP.idle;
  return (
    <div className="agent-card" role="article" aria-label={`Ajan: ${name}`}>
      <div className="agent-card-header">
        <span className="agent-card-name">{name}</span>
        <Badge variant={s.variant}>{s.label}</Badge>
      </div>
      <p className="agent-card-desc">{description}</p>
      <div className="agent-card-meta">
        {lastAction && (
          <span className="agent-card-action">Son: {lastAction}</span>
        )}
        {runCount !== undefined && (
          <span className="agent-card-count">{runCount} çalışma</span>
        )}
      </div>
    </div>
  );
}
