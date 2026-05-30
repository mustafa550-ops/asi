import { type ReactNode } from "react";

interface CardProps {
  title?: string;
  children: ReactNode;
  className?: string;
}

export function Card({ title, children, className = "" }: CardProps) {
  return (
    <div className={`ui-card ${className}`}>
      {title && <h3 className="ui-card-title">{title}</h3>}
      {children}
    </div>
  );
}
