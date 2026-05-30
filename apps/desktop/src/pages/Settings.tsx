import { useState, useCallback } from "react";
import { Input } from "../components/ui/Input";
import { Button } from "../components/ui/Button";
import { Card } from "../components/ui/Card";
import { invoke } from "../lib/tauri";
import { toast } from "../components/ui/Toast";

type SettingsTab =
  | "general"
  | "llm"
  | "voice"
  | "hardware"
  | "memory"
  | "security";

const TABS: { id: SettingsTab; label: string }[] = [
  { id: "general", label: "Genel" },
  { id: "llm", label: "LLM" },
  { id: "voice", label: "Ses" },
  { id: "hardware", label: "Donanım" },
  { id: "memory", label: "Bellek" },
  { id: "security", label: "Güvenlik" },
];

export function Settings() {
  const [tab, setTab] = useState<SettingsTab>("general");

  const handleSave = useCallback(
    async (key: string, event: React.FormEvent<HTMLFormElement>) => {
      event.preventDefault();
      const form = event.currentTarget;
      const data = new FormData(form);
      const value = data.get(key) as string;
      try {
        await invoke("save_setting", { key, value });
        toast(`${key} kaydedildi`, "success");
      } catch {
        toast("Kaydedilemedi", "error");
      }
    },
    [],
  );

  return (
    <div className="settings-page">
      <h2>Ayarlar</h2>
      <nav
        className="settings-tabs"
        role="tablist"
        aria-label="Ayar kategorileri"
      >
        {TABS.map((t) => (
          <button
            key={t.id}
            className={`settings-tab ${tab === t.id ? "active" : ""}`}
            onClick={() => setTab(t.id)}
            role="tab"
            aria-selected={tab === t.id}
          >
            {t.label}
          </button>
        ))}
      </nav>
      <div className="settings-content" role="tabpanel">
        {tab === "general" && (
          <Card title="Genel Ayarlar">
            <form onSubmit={(e) => handleSave("ollama-url", e)}>
              <Input
                label="Ollama URL"
                id="ollama-url"
                name="ollama-url"
                defaultValue="http://127.0.0.1:11434"
              />
              <Input
                label="MCP Port"
                id="mcp-port"
                name="mcp-port"
                defaultValue="9876"
              />
              <Button type="submit" variant="primary" size="sm">
                Kaydet
              </Button>
            </form>
            <div
              style={{
                marginTop: 16,
                paddingTop: 16,
                borderTop: "1px solid var(--border-color)",
              }}
            >
              <label
                style={{
                  display: "block",
                  fontSize: "0.8rem",
                  color: "var(--text-secondary)",
                  marginBottom: 8,
                }}
              >
                Tema
              </label>
              <Button
                variant="ghost"
                size="sm"
                onClick={() => {
                  const current =
                    document.documentElement.getAttribute("data-theme");
                  const next = current === "light" ? "dark" : "light";
                  document.documentElement.setAttribute("data-theme", next);
                  localStorage.setItem("adler-theme", next);
                }}
              >
                {document.documentElement.getAttribute("data-theme") === "light"
                  ? "☀️"
                  : "🌙"}{" "}
                Tema Değiştir
              </Button>
            </div>
          </Card>
        )}
        {tab === "llm" && (
          <Card title="LLM Ayarları">
            <form onSubmit={(e) => handleSave("local-model", e)}>
              <Input
                label="Yerel Model"
                id="local-model"
                name="local-model"
                defaultValue="qwen2.5:1.5b"
              />
              <Input
                label="Claude API Anahtarı"
                id="claude-key"
                name="claude-key"
                type="password"
              />
              <Input
                label="Claude Model"
                id="claude-model"
                name="claude-model"
                defaultValue="claude-sonnet-4-20250514"
              />
              <Button type="submit" variant="primary" size="sm">
                Kaydet
              </Button>
            </form>
          </Card>
        )}
        {tab === "voice" && (
          <Card title="Ses Ayarları">
            <form onSubmit={(e) => handleSave("wake-word", e)}>
              <Input
                label="Wake Word"
                id="wake-word"
                name="wake-word"
                defaultValue="Hey Adler"
              />
              <Input
                label="Ses Hızı"
                id="voice-speed"
                name="voice-speed"
                defaultValue="1.0"
              />
              <Button type="submit" variant="primary" size="sm">
                Kaydet
              </Button>
            </form>
          </Card>
        )}
        {tab === "hardware" && (
          <Card title="Donanım Ayarları">
            <form onSubmit={(e) => handleSave("relay-pin", e)}>
              <Input
                label="GPIO Pin (Röle)"
                id="relay-pin"
                name="relay-pin"
                defaultValue="17"
              />
              <Button type="submit" variant="primary" size="sm">
                Kaydet
              </Button>
            </form>
          </Card>
        )}
        {tab === "memory" && (
          <Card title="Bellek Ayarları">
            <form onSubmit={(e) => handleSave("db-path", e)}>
              <Input
                label="Veritabanı Yolu"
                id="db-path"
                name="db-path"
                defaultValue="./data/adler.db"
              />
              <Input
                label="Maks Token"
                id="max-tokens"
                name="max-tokens"
                defaultValue="8192"
              />
              <Button type="submit" variant="primary" size="sm">
                Kaydet
              </Button>
            </form>
          </Card>
        )}
        {tab === "security" && (
          <Card title="Güvenlik Ayarları">
            <form onSubmit={(e) => handleSave("approval-level", e)}>
              <Input
                label="Onay Seviyesi"
                id="approval-level"
                name="approval-level"
                defaultValue="observer"
              />
              <Button type="submit" variant="danger" size="sm">
                Güvenlik Denetimi Çalıştır
              </Button>
            </form>
          </Card>
        )}
      </div>
    </div>
  );
}
