import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "../../test/test-utils";
import ChatPanel from "./ChatPanel";

const mockSend = vi.fn();
const mockSetInput = vi.fn();

vi.mock("../../stores/chatStore", () => ({
  useChatStore: (selector: (s: Record<string, unknown>) => unknown) => selector({
    messages: [
      { role: "adler", content: "ADLER ASI hazır." },
    ],
    input: "",
    loading: false,
    send: mockSend,
    setInput: mockSetInput,
  }),
}));

vi.mock("../MarkdownRenderer", () => ({
  MarkdownRenderer: ({ content }: { content: string }) => <div data-testid="md">{content}</div>,
}));

vi.mock("./TypingIndicator", () => ({
  TypingIndicator: () => <div data-testid="typing" />,
}));

vi.mock("./SlashCommands", () => ({
  SlashCommands: () => <div data-testid="slash" />,
}));

vi.mock("./ChatHistory", () => ({
  ChatHistory: () => <div data-testid="history" />,
}));

vi.mock("./ContextWindow", () => ({
  ContextWindow: () => <div data-testid="context" />,
}));

vi.mock("./FileAttachment", () => ({
  FileAttachment: ({ onAttach }: { onAttach: (f: { name: string }) => void }) => (
    <button data-testid="attach" onClick={() => onAttach({ name: "test.md", size: 100, content: "test" })}>
      Ekle
    </button>
  ),
}));

vi.mock("./ProactiveAlert", () => ({
  default: () => <div data-testid="proactive" />,
  ProactiveAlert: () => <div data-testid="proactive" />,
}));

describe("ChatPanel", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders chat messages", () => {
    render(<ChatPanel />);
    expect(screen.getByText("ADLER ASI hazır.")).toBeInTheDocument();
  });

  it("renders message input", () => {
    render(<ChatPanel />);
    expect(screen.getByLabelText("Mesaj girişi")).toBeInTheDocument();
  });

  it("renders send button", () => {
    render(<ChatPanel />);
    expect(screen.getByLabelText("Gönder")).toBeInTheDocument();
  });

  it("calls send on send button click", () => {
    render(<ChatPanel />);
    fireEvent.click(screen.getByLabelText("Gönder"));
    expect(mockSend).toHaveBeenCalledOnce();
  });

  it("calls send on Enter key", () => {
    render(<ChatPanel />);
    const input = screen.getByLabelText("Mesaj girişi");
    fireEvent.keyDown(input, { key: "Enter" });
    expect(mockSend).toHaveBeenCalledOnce();
  });

  it("calls setInput on input change", () => {
    render(<ChatPanel />);
    const input = screen.getByLabelText("Mesaj girişi");
    fireEvent.change(input, { target: { value: "test" } });
    expect(mockSetInput).toHaveBeenCalledWith("test");
  });

  it("renders history toggle button", () => {
    render(<ChatPanel />);
    expect(screen.getByText("Geçmiş")).toBeInTheDocument();
  });

  it("toggles history visibility on button click", () => {
    render(<ChatPanel />);
    fireEvent.click(screen.getByText("Geçmiş"));
    expect(screen.getByText("Gizle")).toBeInTheDocument();
  });

  it("renders proactive alerts", () => {
    render(<ChatPanel />);
    expect(screen.getByTestId("proactive")).toBeInTheDocument();
  });

  it("renders log role for messages area", () => {
    render(<ChatPanel />);
    expect(screen.getByRole("log")).toBeInTheDocument();
  });
});
