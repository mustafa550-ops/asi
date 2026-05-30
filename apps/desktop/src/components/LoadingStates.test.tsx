import { describe, it, expect } from "vitest";
import { render, screen } from "../test/test-utils";
import { Skeleton, LoadingSpinner, EmptyState } from "./LoadingStates";

describe("Skeleton", () => {
  it("renders with default dimensions", () => {
    const { container } = render(<Skeleton />);
    const el = container.querySelector(".skeleton");
    expect(el).toHaveStyle({ width: "100%", height: "16px" });
  });

  it("renders with custom dimensions", () => {
    const { container } = render(<Skeleton width="50%" height="32px" />);
    const el = container.querySelector(".skeleton");
    expect(el).toHaveStyle({ width: "50%", height: "32px" });
  });

  it("is hidden from accessibility tree", () => {
    const { container } = render(<Skeleton />);
    expect(container.querySelector(".skeleton")).toHaveAttribute("aria-hidden", "true");
  });
});

describe("LoadingSpinner", () => {
  it("renders with default text", () => {
    render(<LoadingSpinner />);
    expect(screen.getByText("Yükleniyor...")).toBeInTheDocument();
  });

  it("renders with custom text", () => {
    render(<LoadingSpinner text="Analiz ediliyor..." />);
    expect(screen.getByText("Analiz ediliyor...")).toBeInTheDocument();
  });

  it("has status role with aria-label", () => {
    render(<LoadingSpinner text="Yükleniyor..." />);
    expect(screen.getByRole("status")).toHaveAttribute("aria-label", "Yükleniyor...");
  });

  it("renders spinner element", () => {
    const { container } = render(<LoadingSpinner />);
    expect(container.querySelector(".spinner")).toBeInTheDocument();
  });
});

describe("EmptyState", () => {
  it("renders title", () => {
    render(<EmptyState title="Veri bulunamadı" />);
    expect(screen.getByText("Veri bulunamadı")).toBeInTheDocument();
  });

  it("renders description when provided", () => {
    render(<EmptyState title="Boş" description="Henüz kayıt yok." />);
    expect(screen.getByText("Henüz kayıt yok.")).toBeInTheDocument();
  });

  it("does not render description when not provided", () => {
    render(<EmptyState title="Boş" />);
    expect(screen.queryByText("Henüz kayıt yok.")).not.toBeInTheDocument();
  });

  it("renders action when provided", () => {
    render(<EmptyState title="Boş" action={<button>Ekle</button>} />);
    expect(screen.getByText("Ekle")).toBeInTheDocument();
  });

  it("renders default icon", () => {
    const { container } = render(<EmptyState title="Boş" />);
    expect(container.querySelector(".empty-icon")).toHaveTextContent("📭");
  });

  it("renders custom icon", () => {
    const { container } = render(<EmptyState title="Boş" icon="🔍" />);
    expect(container.querySelector(".empty-icon")).toHaveTextContent("🔍");
  });

  it("has status role", () => {
    render(<EmptyState title="Boş" />);
    expect(screen.getByRole("status")).toBeInTheDocument();
  });
});
