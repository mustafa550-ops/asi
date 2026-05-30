import { useState } from "react";
import { invoke } from "../lib/tauri";

interface CodeRunnerProps {
  code: string;
  language?: string;
  showLineNumbers?: boolean;
}

export default function CodeRunner({ code, language, showLineNumbers }: CodeRunnerProps) {
  const [output, setOutput] = useState<string | null>(null);
  const [running, setRunning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleRun = async () => {
    setRunning(true);
    setOutput(null);
    setError(null);
    try {
      const result = await invoke<string>("run_code", { code });
      setOutput(result);
    } catch (err) {
      setError(String(err));
    } finally {
      setRunning(false);
    }
  };

  const lines = code.split("\n");

  return (
    <div className="coderunner">
      <div className="coderunner-header">
        <span className="coderunner-lang">{language || "code"}</span>
        <button
          className="coderunner-run"
          onClick={handleRun}
          disabled={running}
        >
          {running ? "Çalişiyor..." : "▶ Çaliştir"}
        </button>
      </div>
      <pre className="coderunner-code">
        {showLineNumbers && (
          <code className="coderunner-lines">
            {lines.map((_, i) => (
              <span key={i} className="coderunner-line-num">{i + 1}</span>
            ))}
          </code>
        )}
        <code>{code}</code>
      </pre>
      {(output || error) && (
        <div className={`coderunner-output ${error ? "coderunner-error" : ""}`}>
          <strong>{error ? "Hata" : "Çikti"}:</strong>
          <pre>{error || output}</pre>
        </div>
      )}
    </div>
  );
}
