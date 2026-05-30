import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import VoiceInterface from "./VoiceInterface";

const mockStartListening = vi.fn();
const mockStopListening = vi.fn();
let mockState: Record<string, unknown> = {
  isListening: false,
  isProcessing: false,
  transcript: "",
  startListening: mockStartListening,
  stopListening: mockStopListening,
};

vi.mock("../../stores/voiceStore", () => ({
  useVoiceStore: (selector?: (s: Record<string, unknown>) => unknown) => {
    const state = { ...mockState };
    return selector ? selector(state) : state;
  },
}));

describe("VoiceInterface", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockState = {
      isListening: false,
      isProcessing: false,
      transcript: "",
      startListening: mockStartListening,
      stopListening: mockStopListening,
    };
  });

  it("renders microphone button", () => {
    render(<VoiceInterface />);
    expect(screen.getByTitle("Sesi Etkinleştir")).toBeInTheDocument();
  });

  it("calls startListening on click when not listening", () => {
    render(<VoiceInterface />);
    fireEvent.click(screen.getByTitle("Sesi Etkinleştir"));
    expect(mockStartListening).toHaveBeenCalledOnce();
  });

  it("shows listening title when listening", () => {
    mockState.isListening = true;
    render(<VoiceInterface />);
    expect(screen.getByTitle("Dinlemeyi Durdur")).toBeInTheDocument();
  });

  it("shows spinner when processing", () => {
    mockState.isListening = true;
    mockState.isProcessing = true;
    const { container } = render(<VoiceInterface />);
    expect(container.querySelector(".spinner")).toBeInTheDocument();
  });

  it("shows transcript when available", () => {
    mockState.transcript = "Merhaba Adler";
    render(<VoiceInterface />);
    expect(screen.getByText("Merhaba Adler")).toBeInTheDocument();
  });

  it("applies active class when listening", () => {
    mockState.isListening = true;
    const { container } = render(<VoiceInterface />);
    expect(container.querySelector(".voice-interface-wrapper")).toHaveClass("active");
  });
});
