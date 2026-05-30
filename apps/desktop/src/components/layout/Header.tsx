interface HeaderProps {
  title: string;
  subtitle?: string;
  children?: React.ReactNode;
}

export function Header({ title, subtitle, children }: HeaderProps) {
  return (
    <header className="layout-header" role="banner">
      <div className="header-info">
        <h1 className="header-title">{title}</h1>
        {subtitle && <span className="header-subtitle">{subtitle}</span>}
      </div>
      <div className="header-actions">{children}</div>
    </header>
  );
}
