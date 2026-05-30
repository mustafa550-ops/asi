import { type ReactNode } from "react";

interface NavItem {
  id: string;
  label: string;
  icon?: string;
}

interface SidebarProps {
  items: NavItem[];
  activeTab: string;
  onTabChange: (id: string) => void;
  collapsed: boolean;
  onToggle: () => void;
  children?: ReactNode;
}

export function Sidebar({
  items,
  activeTab,
  onTabChange,
  collapsed,
  onToggle,
  children,
}: SidebarProps) {
  return (
    <aside
      className={`layout-sidebar ${collapsed ? "collapsed" : ""}`}
      role="navigation"
      aria-label="Ana navigasyon"
    >
      <div className="sidebar-header">
        {!collapsed && <span className="sidebar-logo">ADLER</span>}
        <button
          className="sidebar-toggle"
          onClick={onToggle}
          aria-label={collapsed ? "Genişlet" : "Daralt"}
        >
          {collapsed ? "☰" : "✕"}
        </button>
      </div>
      <nav className="sidebar-nav">
        {items.map((item) => (
          <button
            key={item.id}
            className={`sidebar-nav-item ${activeTab === item.id ? "active" : ""}`}
            onClick={() => onTabChange(item.id)}
            aria-current={activeTab === item.id ? "page" : undefined}
          >
            {item.icon && <span className="sidebar-icon">{item.icon}</span>}
            {!collapsed && <span className="sidebar-label">{item.label}</span>}
          </button>
        ))}
      </nav>
      {children && <div className="sidebar-footer">{children}</div>}
    </aside>
  );
}
