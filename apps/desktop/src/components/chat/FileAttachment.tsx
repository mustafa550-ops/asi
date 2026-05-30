import { useState, useRef } from "react";
import { toast } from "../ui/Toast";

interface AttachedFile {
  name: string;
  size: number;
  content: string;
}

interface FileAttachmentProps {
  onAttach: (file: AttachedFile) => void;
}

const MAX_SIZE = 10 * 1024 * 1024;

const ALLOWED = new Set([
  "md",
  "txt",
  "json",
  "csv",
  "yaml",
  "yml",
  "toml",
  "rs",
  "ts",
  "tsx",
  "py",
  "js",
  "html",
  "css",
  "xml",
  "sql",
]);

function fileIcon(name: string): string {
  const ext = name.split(".").pop()?.toLowerCase();
  switch (ext) {
    case "rs":
      return "🦀";
    case "ts":
    case "tsx":
      return "🔷";
    case "py":
      return "🐍";
    case "js":
      return "🟨";
    case "json":
    case "yaml":
    case "yml":
    case "toml":
    case "xml":
      return "📋";
    case "md":
      return "📝";
    case "csv":
      return "📊";
    case "sql":
      return "🗄️";
    case "html":
      return "🌐";
    case "css":
      return "🎨";
    default:
      return "📄";
  }
}

export function FileAttachment({ onAttach }: FileAttachmentProps) {
  const [dragging, setDragging] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  const handleFile = async (file: File) => {
    if (file.size > MAX_SIZE) {
      toast("Dosya boyutu 10MB'dan büyük olamaz.", "error");
      return;
    }
    const ext = file.name.split(".").pop()?.toLowerCase();
    if (!ext || !ALLOWED.has(ext)) {
      toast(`Desteklenmeyen dosya türü: .${ext}`, "error");
      return;
    }
    const content = await file.text();
    onAttach({ name: file.name, size: file.size, content });
    toast(`${fileIcon(file.name)} ${file.name} eklendi`, "success");
  };

  return (
    <div
      className={`file-attach ${dragging ? "file-attach-dragging" : ""}`}
      onDragOver={(e) => {
        e.preventDefault();
        setDragging(true);
      }}
      onDragLeave={() => setDragging(false)}
      onDrop={(e) => {
        e.preventDefault();
        setDragging(false);
        if (e.dataTransfer.files[0]) handleFile(e.dataTransfer.files[0]);
      }}
    >
      <button
        className="file-attach-btn"
        onClick={() => inputRef.current?.click()}
        aria-label="Dosya ekle"
        title="Dosya ekle"
      >
        📎
      </button>
      <input
        ref={inputRef}
        type="file"
        hidden
        accept=".md,.txt,.json,.csv,.yaml,.yml,.toml,.rs,.ts,.tsx,.py,.js,.html,.css,.xml,.sql"
        onChange={(e) => {
          if (e.target.files?.[0]) handleFile(e.target.files[0]);
        }}
      />
    </div>
  );
}
