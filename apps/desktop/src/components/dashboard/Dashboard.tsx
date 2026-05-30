import { useEffect, useState } from "react";
import { invoke } from "../../lib/tauri";
import { AgentCard } from "./AgentCard";
import { EventStream } from "./EventStream";
import { SystemMetrics } from "./SystemMetrics";
import { Card } from "../ui/Card";

interface AgentInfo {
  name: string;
  status: string;
  description: string;
}

export default function Dashboard() {
  const [sysInfo, setSysInfo] = useState<string>("");
  const [agents, setAgents] = useState<AgentInfo[]>([]);

  useEffect(() => {
    invoke<string>("send_command", { command: "sistem durumu" })
      .then(setSysInfo)
      .catch(() => setSysInfo("Sistem bilgisi alınamadı"));
  }, []);

  useEffect(() => {
    invoke<string>("get_agent_statuses")
      .then((raw) => {
        const list: AgentInfo[] = JSON.parse(raw);
        setAgents(list);
      })
      .catch(() => {});
  }, []);

  return (
    <div className="dashboard-enhanced">
      <div className="dashboard-grid">
        <div className="dashboard-col-main">
          <Card title="Ajan Durumu">
            <div className="agent-grid">
              {agents.map((a) => (
                <AgentCard
                  key={a.name}
                  name={a.name}
                  status={a.status as "ready" | "idle" | "error"}
                  description={a.description}
                />
              ))}
            </div>
          </Card>
          <Card title="Sistem Bilgisi">
            <pre className="dashboard-sysinfo">{sysInfo}</pre>
          </Card>
        </div>
        <div className="dashboard-col-side">
          <SystemMetrics />
          <EventStream />
        </div>
      </div>
    </div>
  );
}
