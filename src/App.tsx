import { useState } from "react";

function App() {
  const [activeTab, setActiveTab] = useState<"chat" | "dashboard" | "skills" | "approval">("chat");

  return (
    <div className="app">
      <header>
        <h1>ADLER ASI</h1>
        <nav>
          <button onClick={() => setActiveTab("chat")}>Chat</button>
          <button onClick={() => setActiveTab("dashboard")}>Dashboard</button>
          <button onClick={() => setActiveTab("skills")}>Skills</button>
          <button onClick={() => setActiveTab("approval")}>Onay</button>
        </nav>
      </header>
      <main>
        {activeTab === "chat" && <div>Chat Arayüzü</div>}
        {activeTab === "dashboard" && <div>Dashboard</div>}
        {activeTab === "skills" && <div>Skills Manager</div>}
        {activeTab === "approval" && <div>Onay Paneli</div>}
      </main>
    </div>
  );
}

export default App;
