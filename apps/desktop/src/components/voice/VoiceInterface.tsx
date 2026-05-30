import { useVoiceStore } from "../../stores/voiceStore";
import "./VoiceInterface.css";

interface VoiceInterfaceProps {
  onExpand?: () => void;
}

export default function VoiceInterface({ onExpand }: VoiceInterfaceProps) {
  const {
    isListening,
    isProcessing,
    transcript,
    startListening,
    stopListening,
  } = useVoiceStore();

  const handleToggle = () => {
    if (isListening) {
      stopListening();
    } else {
      startListening();
    }
  };

  return (
    <div className={`voice-interface-wrapper ${isListening ? "active" : ""}`}>
      {/* Dalga animasyonu (Sadece dinlerken) */}
      {isListening && (
        <div className="wave-container">
          <div className="wave"></div>
          <div className="wave"></div>
          <div className="wave"></div>
        </div>
      )}

      {/* Floating Mikrofon Butonu */}
      <button
        className={`voice-fab ${isListening ? "listening" : ""} ${isProcessing ? "processing" : ""}`}
        onClick={handleToggle}
        title={isListening ? "Dinlemeyi Durdur" : "Sesi Etkinleştir"}
      >
        {isProcessing ? (
          <span className="spinner"></span>
        ) : (
          <svg viewBox="0 0 24 24" fill="currentColor" width="24" height="24">
            <path d="M12 14c1.66 0 3-1.34 3-3V5c0-1.66-1.34-3-3-3S9 3.34 9 5v6c0 1.66 1.34 3 3 3zm5-3c0 2.76-2.24 5-5 5s-5-2.24-5-5H5c0 3.53 2.61 6.43 6 6.92V21h2v-3.08c3.39-.49 6-3.39 6-6.92h-2z" />
          </svg>
        )}
      </button>

      {/* Tam ekran butonu */}
      {onExpand && (
        <button
          className="voice-expand-btn"
          onClick={onExpand}
          title="Tam Ekran"
          aria-label="Tam ekran sesli asistan"
        >
          <svg viewBox="0 0 24 24" fill="currentColor" width="18" height="18">
            <path d="M7 14H5v5h5v-2H7v-3zm-2-4h2V7h3V5H5v5zm12 7h-3v2h5v-5h-2v3zM14 5v2h3v3h2V5h-5z" />
          </svg>
        </button>
      )}

      {/* Metin Dökümü */}
      {transcript && (
        <div className="voice-transcript-bubble">{transcript}</div>
      )}
    </div>
  );
}
