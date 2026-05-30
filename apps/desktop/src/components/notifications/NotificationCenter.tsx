import { useState, useCallback } from "react";
import { useTauriEvent } from "../../hooks/useTauriEvent";
import { Badge } from "../ui/Badge";

interface Notification {
  id: number;
  text: string;
  severity: "info" | "warning" | "error";
  read: boolean;
  time: string;
}

let nid = 0;

export function NotificationCenter() {
  const [notifications, setNotifications] = useState<Notification[]>([]);
  const [open, setOpen] = useState(false);

  const addNotif = useCallback((payload: string) => {
    const n: Notification = {
      id: ++nid,
      text: payload,
      severity: "info",
      read: false,
      time: new Date().toLocaleTimeString(),
    };
    try {
      const p = JSON.parse(payload);
      n.text = p.message ?? p.detail ?? payload;
      n.severity = p.severity ?? "info";
    } catch {
      /* ignore */
    }
    setNotifications((prev) => [n, ...prev].slice(0, 50));
  }, []);

  useTauriEvent("notification", addNotif);
  useTauriEvent("pipeline-error", addNotif);

  const unread = notifications.filter((n) => !n.read).length;

  const markRead = (id: number) => {
    setNotifications((prev) =>
      prev.map((n) => (n.id === id ? { ...n, read: true } : n)),
    );
  };

  const markAllRead = () => {
    setNotifications((prev) => prev.map((n) => ({ ...n, read: true })));
  };

  const severityVariant = (s: string) => {
    switch (s) {
      case "error":
        return "error" as const;
      case "warning":
        return "warning" as const;
      default:
        return "info" as const;
    }
  };

  return (
    <div className="notif-center">
      <button
        className="notif-bell"
        onClick={() => setOpen(!open)}
        aria-label={`Bildirimler (${unread})`}
      >
        🔔 {unread > 0 && <span className="notif-badge">{unread}</span>}
      </button>
      {open && (
        <div className="notif-dropdown" role="menu">
          <div className="notif-header">
            <h3>Bildirimler</h3>
            {unread > 0 && (
              <button className="notif-mark-read" onClick={markAllRead}>
                Tümünü okundu işaretle
              </button>
            )}
          </div>
          <div className="notif-list">
            {notifications.length === 0 && (
              <p className="notif-empty">Bildirim yok.</p>
            )}
            {notifications.map((n) => (
              <div
                key={n.id}
                className={`notif-item ${n.read ? "read" : "unread"}`}
                onClick={() => markRead(n.id)}
                role="menuitem"
              >
                <Badge variant={severityVariant(n.severity)}>
                  {n.severity}
                </Badge>
                <span className="notif-text">{n.text}</span>
                <span className="notif-time">{n.time}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
