import { describe, it, expect, beforeEach, vi } from "vitest";
import { render, screen, waitFor } from "@testing-library/react";
import SkillsManager from "./SkillsManager";

const mockInvoke = vi.fn();
vi.mock("../../lib/tauri", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

const mockSkills = [
  {
    id: 1,
    name: "Borsa_Analiz",
    description: "Kripto analizi",
    triggers: ["btc", "eth"],
    approval: "auto",
    steps: [{ order: 1, description: "Veri çek" }],
    logic_code: null,
    evolution: ["v1.0: Ilk surum"],
    run_count: 3,
    active: true,
    version: 1,
    created_at: "2026-01-01",
  },
  {
    id: 2,
    name: "Ses_Kontrol",
    description: "Sesli komut",
    triggers: ["ses"],
    approval: "auto",
    steps: [{ order: 1, description: "Dinle" }],
    logic_code: null,
    evolution: [],
    run_count: 0,
    active: false,
    version: 1,
    created_at: "2026-01-02",
  },
];

describe("SkillsManager", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("bos liste mesaji gosterir", async () => {
    mockInvoke.mockResolvedValue([]);
    render(<SkillsManager />);
    await waitFor(() => {
      expect(screen.getByText(/Henüz yetenek eklenmemiş/i)).toBeDefined();
    });
  });

  it("skill listesini gosterir", async () => {
    mockInvoke.mockResolvedValue(mockSkills);
    render(<SkillsManager />);
    await waitFor(() => {
      expect(screen.getByText("Borsa_Analiz")).toBeDefined();
      expect(screen.getByText("Ses_Kontrol")).toBeDefined();
    });
  });

  it("yukleme durumunda loading mesaji gosterir", async () => {
    mockInvoke.mockImplementation(() => new Promise(() => {}));
    render(<SkillsManager />);
    expect(screen.getByText(/Yetenekler yukleniyor/i)).toBeDefined();
  });
});
