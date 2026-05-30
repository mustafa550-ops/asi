import { useState, useEffect, useRef } from "react";

interface SlashCommandsProps {
  input: string;
  onSelect: (command: string) => void;
  onClose: () => void;
}

const COMMANDS = [
  { cmd: "/analiz", hint: "Kripto analizi yap" },
  { cmd: "/donanim", hint: "Donanım durumunu kontrol et" },
  { cmd: "/piyasa", hint: "Piyasa bilgisi al" },
  { cmd: "/sistem", hint: "Sistem durumunu göster" },
  { cmd: "/bellek", hint: "Bellek durumunu sorgula" },
  { cmd: "/yeteneek", hint: "Yetenek yönetimi" },
];

export function SlashCommands({
  input,
  onSelect,
  onClose,
}: SlashCommandsProps) {
  const [selectedIdx, setSelectedIdx] = useState(0);
  const ref = useRef<HTMLDivElement>(null);

  const filtered = COMMANDS.filter((c) =>
    c.cmd.startsWith(input.toLowerCase()),
  );

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === "ArrowDown") {
        e.preventDefault();
        setSelectedIdx((i) => Math.min(i + 1, filtered.length - 1));
      }
      if (e.key === "ArrowUp") {
        e.preventDefault();
        setSelectedIdx((i) => Math.max(i - 1, 0));
      }
      if (e.key === "Enter" && filtered[selectedIdx]) {
        e.preventDefault();
        onSelect(filtered[selectedIdx].cmd);
      }
      if (e.key === "Escape") {
        onClose();
      }
    };
    document.addEventListener("keydown", handler);
    return () => document.removeEventListener("keydown", handler);
  }, [filtered, selectedIdx, onSelect, onClose]);

  if (filtered.length === 0) return null;

  return (
    <div className="chat-slash-menu" ref={ref} role="listbox">
      {filtered.map((c, i) => (
        <div
          key={c.cmd}
          className={`chat-slash-item ${i === selectedIdx ? "active" : ""}`}
          onClick={() => onSelect(c.cmd)}
          role="option"
          aria-selected={i === selectedIdx}
        >
          <strong>{c.cmd}</strong>
          <span className="slash-hint">{c.hint}</span>
        </div>
      ))}
    </div>
  );
}
