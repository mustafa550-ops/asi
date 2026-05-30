interface BadgeProps {
  variant?: "success" | "warning" | "error" | "info" | "neutral";
  children: string;
}

export function Badge({ variant = "neutral", children }: BadgeProps) {
  return <span className={`ui-badge ui-badge-${variant}`}>{children}</span>;
}
