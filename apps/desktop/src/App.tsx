import { useState } from "react";
import { MainLayout, type TabId } from "./components/layout/MainLayout";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { ToastContainer } from "./components/ui/Toast";
import { useTheme } from "./hooks/useTheme";
import { useKeyboard } from "./hooks/useKeyboard";
import ChatPanel from "./components/chat/ChatPanel";
import Dashboard from "./components/dashboard/Dashboard";
import ApprovalPanel from "./components/approval-panels/ApprovalPanel";
import SkillsManager from "./components/skills-manager/SkillsManager";
import { Settings } from "./pages/Settings";
import VoiceInterface from "./components/voice/VoiceInterface";

export default function App() {
  const [tab, setTab] = useState<TabId>("chat");
  useTheme();
  useKeyboard({ "Ctrl+Shift+A": () => setTab("chat") });

  return (
    <ErrorBoundary>
      <MainLayout activeTab={tab} onTabChange={setTab}>
        {tab === "chat" && <ChatPanel />}
        {tab === "dashboard" && <Dashboard />}
        {tab === "approval" && <ApprovalPanel />}
        {tab === "skills" && <SkillsManager />}
        {tab === "settings" && <Settings />}
      </MainLayout>
      <ToastContainer />
      <VoiceInterface />
    </ErrorBoundary>
  );
}
