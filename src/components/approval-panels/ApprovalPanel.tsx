import { useState, useCallback } from "react";
import { invoke } from "../../lib/tauri";
import { useTauriEvent } from "../../hooks/useTauriEvent";

interface Approval {
  id: string;
  task: string;
  summary: string;
}

export default function ApprovalPanel() {
  const [approvals, setApprovals] = useState<Approval[]>([]);

  const handleApprovalEvent = useCallback((payload: string) => {
    try {
      const data = JSON.parse(payload) as Approval;
      setApprovals((p) => [...p, data]);
    } catch {
      /* ignore parse errors */
    }
  }, []);

  useTauriEvent("approval-required", handleApprovalEvent);

  const respond = async (id: string, action: "approve" | "reject") => {
    try {
      const result = await invoke<string>(
        action === "approve" ? "approve_action" : "reject_action",
        { id }
      );
      setApprovals((p) => p.filter((a) => a.id !== id));
      alert(`${action === "approve" ? "Onaylandı" : "Reddedildi"}: ${result}`);
    } catch (err) {
      alert(`Hata: ${err}`);
    }
  };

  return (
    <div className="approval-panel">
      <h2>Onay Bekleyen İşlemler</h2>
      <div className="approval-list">
        {approvals.length === 0 ? (
          <p style={{ color: "#888" }}>Bekleyen onay yok.</p>
        ) : (
          approvals.map((a) => (
            <div key={a.id} className="approval-card">
              <p><strong>Görev:</strong> {a.task}</p>
              <pre className="approval-summary">{a.summary}</pre>
              <div className="approval-buttons">
                <button onClick={() => respond(a.id, "approve")} className="btn-approve">Onayla</button>
                <button onClick={() => respond(a.id, "reject")} className="btn-reject">Reddet</button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
