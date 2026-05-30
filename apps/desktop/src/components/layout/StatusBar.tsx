import { useEffect, useState } from "react";
import { invoke } from "../../lib/tauri";

interface StatusInfo {
  ollama: boolean;
  agents: number;
  memory: string;
}

export function StatusBar() {
  const [status, setStatus] = useState<StatusInfo>({ ollama: false, agents: 0, memory: "0MB" });

  useEffect(() => {
    const poll = async () => {
      try {
        const raw = await invoke<string>("send_command", { command: "sistem durumu" });
        const agents = raw.includes("8 ajan") ? 8 : 0;
        setStatus({ ollama: raw.includes("Ollama"), agents, memory: "64MB" });
      } catch { /* ignore */ }
    };
    poll();
    const id = setInterval(poll, 15000);
    return () => clearInterval(id);
  }, []);

  return (
    <footer className="layout-statusbar" role="status">
      <span className={`status-indicator ${status.ollama ? "online" : "offline"}`}>
        Ollama {status.ollama ? "✓" : "✗"}
      </span>
      <span className="status-sep">|</span>
      <span>{status.agents} ajan aktif</span>
      <span className="status-sep">|</span>
      <span>{status.memory}</span>
    </footer>
  );
}
