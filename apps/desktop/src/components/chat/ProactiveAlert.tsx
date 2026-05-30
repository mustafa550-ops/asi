import { useState, useCallback } from "react";
import { useTauriEvent } from "../../hooks/useTauriEvent";

interface Alert {
  id: number;
  message: string;
  severity: "info" | "warning" | "critical";
}

let aid = 0;

export function ProactiveAlert() {
  const [alerts, setAlerts] = useState<Alert[]>([]);

  const handleAlert = useCallback((payload: string) => {
    const alert: Alert = { id: ++aid, message: payload, severity: "info" };
    try {
      const p = JSON.parse(payload);
      alert.message = p.message ?? payload;
      alert.severity = p.severity ?? "info";
    } catch {
      /* ignore */
    }
    setAlerts((prev) => [alert, ...prev].slice(0, 5));
    setTimeout(
      () => setAlerts((prev) => prev.filter((a) => a.id !== alert.id)),
      10000,
    );
  }, []);

  useTauriEvent("proactive-alert", handleAlert);

  if (alerts.length === 0) return null;

  return (
    <div className="proactive-alerts" role="alert" aria-live="assertive">
      {alerts.map((a) => (
        <div
          key={a.id}
          className={`proactive-alert proactive-alert-${a.severity}`}
        >
          <span className="proactive-icon">
            {a.severity === "critical"
              ? "🔴"
              : a.severity === "warning"
                ? "🟡"
                : "🔵"}
          </span>
          <span className="proactive-text">{a.message}</span>
          <button
            className="proactive-close"
            onClick={() =>
              setAlerts((prev) => prev.filter((x) => x.id !== a.id))
            }
          >
            &times;
          </button>
        </div>
      ))}
    </div>
  );
}
