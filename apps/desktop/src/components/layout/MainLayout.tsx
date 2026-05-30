import { useState, type ReactNode } from "react";
import { Sidebar } from "./Sidebar";
import { Header } from "./Header";
import { StatusBar } from "./StatusBar";
import { NotificationCenter } from "../notifications/NotificationCenter";

export type TabId = "chat" | "dashboard" | "approval" | "skills" | "settings";

const NAV_ITEMS = [
  { id: "chat", label: "Sohbet", icon: "💬" },
  { id: "dashboard", label: "Dashboard", icon: "📊" },
  { id: "approval", label: "Onay", icon: "✅" },
  { id: "skills", label: "Yetenekler", icon: "🧩" },
  { id: "settings", label: "Ayarlar", icon: "⚙️" },
];

interface MainLayoutProps {
  activeTab: TabId;
  onTabChange: (tab: TabId) => void;
  children: ReactNode;
}

export function MainLayout({
  activeTab,
  onTabChange,
  children,
}: MainLayoutProps) {
  const [collapsed, setCollapsed] = useState(false);

  const currentLabel =
    NAV_ITEMS.find((n) => n.id === activeTab)?.label ?? "ADLER ASI";

  return (
    <div className="layout-root">
      <Sidebar
        items={NAV_ITEMS}
        activeTab={activeTab}
        onTabChange={(id) => onTabChange(id as TabId)}
        collapsed={collapsed}
        onToggle={() => setCollapsed((c) => !c)}
      />
      <div className="layout-main-area">
        <Header title={currentLabel} subtitle="Otonom Dijital Operatör">
          <NotificationCenter />
        </Header>
        <main className="layout-content" role="main">
          {children}
        </main>
        <StatusBar />
      </div>
    </div>
  );
}
