import { useState, useCallback } from "react";
import { useTauriEvent } from "../../hooks/useTauriEvent";

interface SystemEvent {
  time: string;
  type: string;
  detail: string;
}

export function EventStream() {
  const [events, setEvents] = useState<SystemEvent[]>([]);
  const [filter, setFilter] = useState("");

  const handleEvent = useCallback((payload: string) => {
    try {
      const parsed = JSON.parse(payload);
      const evt: SystemEvent = {
        time: new Date().toLocaleTimeString(),
        type: parsed.type ?? "unknown",
        detail: typeof parsed === "string" ? parsed : JSON.stringify(parsed),
      };
      setEvents((prev) => [evt, ...prev].slice(0, 100));
    } catch {
      setEvents((prev) =>
        [
          {
            time: new Date().toLocaleTimeString(),
            type: "raw",
            detail: payload,
          },
          ...prev,
        ].slice(0, 100),
      );
    }
  }, []);

  useTauriEvent("pipeline-step", handleEvent);
  useTauriEvent("pipeline-error", handleEvent);
  useTauriEvent("pipeline-complete", handleEvent);
  useTauriEvent("agent-update", handleEvent);

  const filtered = filter
    ? events.filter((e) => e.type.includes(filter) || e.detail.includes(filter))
    : events;

  return (
    <div className="event-stream" role="log" aria-label="Sistem olayları">
      <div className="event-stream-header">
        <h3>Olay Akışı</h3>
        <input
          className="event-filter"
          placeholder="Filtrele..."
          value={filter}
          onChange={(e) => setFilter(e.target.value)}
          aria-label="Olay filtrele"
        />
      </div>
      <div className="event-list">
        {filtered.length === 0 && (
          <p className="event-empty">Henüz olay yok.</p>
        )}
        {filtered.map((e, i) => (
          <div key={i} className={`event-row event-${e.type}`}>
            <span className="event-time">{e.time}</span>
            <span className={`event-type event-type-${e.type}`}>{e.type}</span>
            <span className="event-detail">{e.detail}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
