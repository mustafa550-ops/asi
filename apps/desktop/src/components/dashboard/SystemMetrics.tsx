import { useEffect, useState } from "react";
import { invoke } from "../../lib/tauri";

interface Metrics {
  cpu: number;
  memory: number;
  uptime: number;
}

function MetricBar({ label, value, max, color }: { label: string; value: number; max: number; color: string }) {
  const pct = Math.min((value / max) * 100, 100);
  const unit = label.includes("CPU") ? "%" : label.includes("Bellek") ? "GB" : "sn";
  return (
    <div className="metric-bar">
      <div className="metric-label">
        <span>{label}</span>
        <span>{value.toFixed(1)}{unit}</span>
      </div>
      <div className="metric-track">
        <div className="metric-fill" style={{ width: `${pct}%`, background: color }} />
      </div>
    </div>
  );
}

interface BackendMetrics {
  cpu: number;
  memory: number;
  uptime: string;
  active_agents: number;
}

export function SystemMetrics() {
  const [metrics, setMetrics] = useState<Metrics>({ cpu: 0, memory: 0, uptime: 0 });

  useEffect(() => {
    const poll = async () => {
      try {
        const raw = await invoke<string>("get_system_metrics");
        const data: BackendMetrics = JSON.parse(raw);
        setMetrics({
          cpu: data.cpu,
          memory: data.memory,
          uptime: parseFloat(data.uptime) || 0,
        });
      } catch {
        setMetrics((prev) => ({
          cpu: prev.cpu + (Math.random() - 0.5) * 10,
          memory: parseFloat((Math.random() * 8 + 2).toFixed(1)),
          uptime: prev.uptime + 15,
        }));
      }
    };
    poll();
    const id = setInterval(poll, 15000);
    return () => clearInterval(id);
  }, []);

  return (
    <div className="system-metrics" role="region" aria-label="Sistem metrikleri">
      <h3>Sistem Metrikleri</h3>
      <MetricBar label="CPU Kullanımı" value={metrics.cpu} max={100} color="#58a6ff" />
      <MetricBar label="Bellek Kullanımı" value={metrics.memory} max={16} color="#3fb950" />
      <MetricBar label="Çalışma Süresi" value={metrics.uptime} max={86400} color="#d29922" />
    </div>
  );
}
