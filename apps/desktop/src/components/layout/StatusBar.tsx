import { useEffect, useState } from "react";
import { invoke } from "../../lib/tauri";

interface BackendMetrics {
  cpu: number;
  memory: number;
  uptime: string;
  active_agents: number;
}

export function StatusBar() {
  const [agents, setAgents] = useState<number | null>(null);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const poll = async () => {
      try {
        const raw = await invoke<string>("get_system_metrics");
        const data: BackendMetrics = JSON.parse(raw);
        setAgents(data.active_agents);
        setConnected(true);
      } catch {
        setAgents(null);
        setConnected(false);
      }
    };
    poll();
    const id = setInterval(poll, 15000);
    return () => clearInterval(id);
  }, []);

  return (
    <footer className="layout-statusbar" role="status">
      <span className={`status-indicator ${connected ? "online" : "offline"}`}>
        Sistem {connected ? "✓" : "✗"}
      </span>
      <span className="status-sep">|</span>
      <span>{agents !== null ? `${agents} ajan aktif` : "-- ajan"}</span>
      <span className="status-sep">|</span>
      <span>ADLER ASI v0.2.2</span>
    </footer>
  );
}
