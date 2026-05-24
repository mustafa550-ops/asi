import { useEffect, useState } from "react";
import { invoke } from "../../lib/tauri";

interface SystemInfo {
  cpu: string;
  memory: string;
  uptime: string;
}

export default function Dashboard() {
  const [info, setInfo] = useState<SystemInfo | null>(null);

  useEffect(() => {
    invoke<string>("send_command", { command: "sistem durumu" })
      .then((r) => {
        const lines = r.split("\n");
        setInfo({
          cpu: lines.find((l) => l.includes("İşlemci"))?.trim() || "N/A",
          memory: lines.find((l) => l.includes("Bellek"))?.trim() || "N/A",
          uptime: lines.find((l) => l.includes("Çalışma"))?.trim() || "N/A",
        });
      })
      .catch(() => {});
  }, []);

  return (
    <div className="dashboard">
      <h2>Sistem Durumu</h2>
      {info ? (
        <div className="dashboard-cards">
          <div className="card">
            <h3>İşlemci</h3>
            <p>{info.cpu}</p>
          </div>
          <div className="card">
            <h3>Bellek</h3>
            <p>{info.memory}</p>
          </div>
          <div className="card">
            <h3>Çalışma</h3>
            <p>{info.uptime}</p>
          </div>
        </div>
      ) : (
        <p>Sistem bilgisi yükleniyor...</p>
      )}
    </div>
  );
}
