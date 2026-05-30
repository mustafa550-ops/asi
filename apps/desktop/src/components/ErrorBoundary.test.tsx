import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "../test/test-utils";
import { ErrorBoundary } from "./ErrorBoundary";

const ProblemChild = ({ shouldThrow = false }: { shouldThrow?: boolean }) => {
  if (shouldThrow) throw new Error("Test hatası");
  return <div>Sağlıklı</div>;
};

beforeEach(() => {
  vi.spyOn(console, "error").mockImplementation(() => {});
});

afterEach(() => {
  vi.restoreAllMocks();
});

describe("ErrorBoundary", () => {
  it("renders children when no error", () => {
    render(
      <ErrorBoundary>
        <div>Sağlıklı</div>
      </ErrorBoundary>
    );
    expect(screen.getByText("Sağlıklı")).toBeInTheDocument();
  });

  it("renders fallback UI on error", () => {
    render(
      <ErrorBoundary>
        <ProblemChild shouldThrow={true} />
      </ErrorBoundary>
    );
    expect(screen.getByText("Bir hata oluştu")).toBeInTheDocument();
    expect(screen.getByText("Test hatası")).toBeInTheDocument();
  });

  it("renders retry button on error", () => {
    render(
      <ErrorBoundary>
        <ProblemChild shouldThrow={true} />
      </ErrorBoundary>
    );
    expect(screen.getByText("Tekrar Dene")).toBeInTheDocument();
  });

  it("recovers after retry click", () => {
    const { rerender } = render(
      <ErrorBoundary>
        <ProblemChild shouldThrow={true} />
      </ErrorBoundary>
    );
    expect(screen.getByText("Bir hata oluştu")).toBeInTheDocument();
    fireEvent.click(screen.getByText("Tekrar Dene"));
  });

  it("uses custom fallback when provided", () => {
    render(
      <ErrorBoundary fallback={<div>Özel hata</div>}>
        <ProblemChild shouldThrow={true} />
      </ErrorBoundary>
    );
    expect(screen.getByText("Özel hata")).toBeInTheDocument();
  });

  it("has alert role on error", () => {
    render(
      <ErrorBoundary>
        <ProblemChild shouldThrow={true} />
      </ErrorBoundary>
    );
    expect(screen.getByRole("alert")).toBeInTheDocument();
  });
});
