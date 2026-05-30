import { useEffect, useRef } from "react";
import { useVoiceStore } from "../../stores/voiceStore";

interface VoiceAssistantProps {
  onClose: () => void;
}

const STATUS_LABELS: Record<string, string> = {
  idle: "Hazır",
  listening: "Dinliyor...",
  processing: "İşleniyor...",
  speaking: "Konuşuyor...",
};

export default function VoiceAssistant({ onClose }: VoiceAssistantProps) {
  const { isListening, isProcessing, transcript, startListening, stopListening } = useVoiceStore();
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const handleKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") onClose();
    };
    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  }, [onClose]);

  const status = isProcessing ? "processing" : isListening ? "listening" : "idle";

  return (
    <div className="voice-assistant-overlay" role="dialog" aria-label="Sesli Asistan">
      <div className="voice-assistant-card">
        <div className="voice-assistant-header">
          <h2>Sesli Asistan</h2>
          <span className="voice-assistant-status" data-status={status}>
            {STATUS_LABELS[status]}
          </span>
          <button className="voice-assistant-close" onClick={onClose} aria-label="Kapat" title="Kapat (Esc)">
            &times;
          </button>
        </div>

        <div className="voice-assistant-visual">
          {isListening && (
            <div className="voice-assistant-waveform">
              {Array.from({ length: 5 }).map((_, i) => (
                <div key={i} className="waveform-bar" style={{ animationDelay: `${i * 0.15}s` }} />
              ))}
            </div>
          )}
          {isProcessing && (
            <div className="voice-assistant-spinner" />
          )}
          {!isListening && !isProcessing && (
            <div className="voice-assistant-idle-icon">
              <svg viewBox="0 0 24 24" fill="currentColor" width="64" height="64">
                <path d="M12 14c1.66 0 3-1.34 3-3V5c0-1.66-1.34-3-3-3S9 3.34 9 5v6c0 1.66 1.34 3 3 3zm5-3c0 2.76-2.24 5-5 5s-5-2.24-5-5H5c0 3.53 2.61 6.43 6 6.92V21h2v-3.08c3.39-.49 6-3.39 6-6.92h-2z" />
              </svg>
            </div>
          )}
        </div>

        {transcript && (
          <div className="voice-assistant-text">{transcript}</div>
        )}

        <div className="voice-assistant-controls">
          <button
            className={`voice-assistant-mic ${isListening ? "active" : ""}`}
            onClick={isListening ? stopListening : startListening}
            disabled={isProcessing}
          >
            {isListening ? "Durdur" : "Başla"}
          </button>
        </div>

        <p className="voice-assistant-hint">
          {isListening
            ? "Konuşmaya başlayabilirsiniz. Durdurmak için butona veya Esc tuşuna basın."
            : "Sesli komut vermek için butona tıklayın."}
        </p>
      </div>
    </div>
  );
}
