import { type ReactNode } from "react";

export function Skeleton({ width = "100%", height = "16px" }: { width?: string; height?: string }) {
  return <div className="skeleton" style={{ width, height }} aria-hidden="true" />;
}

export function LoadingSpinner({ text = "Yükleniyor..." }: { text?: string }) {
  return (
    <div className="loading-spinner" role="status" aria-label={text}>
      <div className="spinner" />
      <span className="loading-text">{text}</span>
    </div>
  );
}

export function EmptyState({ icon = "📭", title, description, action }: { icon?: string; title: string; description?: string; action?: ReactNode }) {
  return (
    <div className="empty-state" role="status">
      <span className="empty-icon">{icon}</span>
      <h3 className="empty-title">{title}</h3>
      {description && <p className="empty-desc">{description}</p>}
      {action && <div className="empty-action">{action}</div>}
    </div>
  );
}
