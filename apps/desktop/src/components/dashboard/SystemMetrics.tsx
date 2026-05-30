import { useEffect, useState } from "react";
import { invoke } from "../../lib/tauri";

interface BackendMetrics {
  cpu: number;
  memory: number;
  uptime: string;
  active_agents: number;
}

export function SystemMetrics() {
  const [connected, setConnected] = useState(false);
  const [cpu, setCpu] = useState<number | null>(null);
  const [memory, setMemory] = useState<number | null>(null);
  const [uptime, setUptime] = useState<number | null>(null);

  useEffect(() => {
    const poll = async () => {
      try {
        const raw = await invoke<string>("get_system_metrics");
        const data: BackendMetrics = JSON.parse(raw);
        setCpu(data.cpu);
        setMemory(data.memory);
        setUptime(parseFloat(data.uptime) || 0);
        setConnected(true);
      } catch {
        setCpu(null);
        setMemory(null);
        setUptime(null);
        setConnected(false);
      }
    };
    poll();
    const id = setInterval(poll, 15000);
    return () => clearInterval(id);
  }, []);

  function MetricBar({
    label,
    value,
    max,
    color,
  }: {
    label: string;
    value: number | null;
    max: number;
    color: string;
  }) {
    const unit = label.includes("CPU")
      ? "%"
      : label.includes("Bellek")
        ? "GB"
        : "sn";
    const display = value !== null ? `${value.toFixed(1)}${unit}` : "--";
    const pct = value !== null ? Math.min((value / max) * 100, 100) : 0;
    return (
      <div className="metric-bar">
        <div className="metric-label">
          <span>{label}</span>
          <span>{display}</span>
        </div>
        <div className="metric-track">
          {value !== null && (
            <div
              className="metric-fill"
              style={{ width: `${pct}%`, background: color }}
            />
          )}
        </div>
      </div>
    );
  }

  return (
    <div
      className="system-metrics"
      role="region"
      aria-label="Sistem metrikleri"
    >
      <h3>
        Sistem Metrikleri
        {!connected && <span className="metric-offline-badge">çevrimdışı</span>}
      </h3>
      <MetricBar label="CPU Kullanımı" value={cpu} max={100} color="#58a6ff" />
      <MetricBar
        label="Bellek Kullanımı"
        value={memory}
        max={16}
        color="#3fb950"
      />
      <MetricBar
        label="Çalışma Süresi"
        value={uptime}
        max={86400}
        color="#d29922"
      />
    </div>
  );
}
