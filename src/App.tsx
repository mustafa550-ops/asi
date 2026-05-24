import { useState } from "react";
import ChatPanel from "./components/chat/ChatPanel";
import Dashboard from "./components/dashboard/Dashboard";
import ApprovalPanel from "./components/approval-panels/ApprovalPanel";

export default function App() {
  const [tab, setTab] = useState<"chat" | "dashboard" | "approval">("chat");

  return (
    <div className="app">
      <header className="app-header">
        <h1>ADLER ASI</h1>
        <nav className="app-nav">
          <button className={tab === "chat" ? "active" : ""} onClick={() => setTab("chat")}>Sohbet</button>
          <button className={tab === "dashboard" ? "active" : ""} onClick={() => setTab("dashboard")}>Dashboard</button>
          <button className={tab === "approval" ? "active" : ""} onClick={() => setTab("approval")}>Onay</button>
        </nav>
      </header>
      <main className="app-main">
        {tab === "chat" && <ChatPanel />}
        {tab === "dashboard" && <Dashboard />}
        {tab === "approval" && <ApprovalPanel />}
      </main>
    </div>
  );
}
