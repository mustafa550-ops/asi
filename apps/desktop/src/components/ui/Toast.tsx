import { useEffect, useState, useCallback } from "react";

export type ToastVariant = "success" | "error" | "info";

interface ToastMessage {
  id: number;
  text: string;
  variant: ToastVariant;
}

let toastId = 0;
const listeners: Array<(m: ToastMessage) => void> = [];

export function toast(text: string, variant: ToastVariant = "info") {
  const msg: ToastMessage = { id: ++toastId, text, variant };
  listeners.forEach((fn) => fn(msg));
}

export function ToastContainer() {
  const [items, setItems] = useState<ToastMessage[]>([]);

  const add = useCallback((m: ToastMessage) => {
    setItems((p) => [...p, m]);
    setTimeout(() => setItems((p) => p.filter((x) => x.id !== m.id)), 4000);
  }, []);

  useEffect(() => {
    listeners.push(add);
    return () => {
      listeners.splice(listeners.indexOf(add), 1);
    };
  }, [add]);

  return (
    <div className="ui-toast-container" aria-live="polite">
      {items.map((m) => (
        <div key={m.id} className={`ui-toast ui-toast-${m.variant}`}>
          {m.text}
        </div>
      ))}
    </div>
  );
}
