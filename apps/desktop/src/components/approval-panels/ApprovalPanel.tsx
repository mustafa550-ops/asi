import { useState, useCallback } from "react";
import { invoke } from "../../lib/tauri";
import { useTauriEvent } from "../../hooks/useTauriEvent";

interface Approval {
  id: string;
  task: string;
  summary: string;
}

interface ApprovalPanelProps {
  onResult?: (action: "approve" | "reject", summary: string) => void;
}

export default function ApprovalPanel({ onResult }: ApprovalPanelProps) {
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
      onResult?.(action, result);
    } catch (err) {
      onResult?.(action, `Hata: ${err}`);
    }
  };

  if (approvals.length === 0) return null;

  return (
    <div className="approval-panel">
      <div className="approval-list">
        {approvals.map((a) => (
          <div key={a.id} className="approval-card">
            <p><strong>Görev:</strong> {a.task}</p>
            <pre className="approval-summary">{a.summary}</pre>
            <div className="approval-buttons">
              <button onClick={() => respond(a.id, "approve")} className="btn-approve">Onayla</button>
              <button onClick={() => respond(a.id, "reject")} className="btn-reject">Reddet</button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
