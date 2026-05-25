import { describe, it, expect, beforeEach, vi } from "vitest";
import { useSkillStore } from "./skillStore";

vi.mock("../lib/tauri", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "../lib/tauri";

const mockSkills = [
  {
    id: 1,
    name: "TestSkill",
    description: "Test",
    triggers: ["test"],
    approval: "auto",
    steps: [{ order: 1, description: "Step 1" }],
    logic_code: null,
    evolution: [],
    run_count: 0,
    active: true,
    version: 1,
    created_at: "2026-01-01",
  },
];

describe("skillStore", () => {
  beforeEach(() => {
    useSkillStore.setState({
      skills: [],
      loading: false,
      error: "",
      panel: "list",
      selected: null,
      mdContent: "",
      result: "",
    });
    vi.clearAllMocks();
  });

  it("loadSkills skills'i doldurur", async () => {
    vi.mocked(invoke).mockResolvedValue(mockSkills);

    await useSkillStore.getState().loadSkills();

    expect(useSkillStore.getState().skills).toEqual(mockSkills);
    expect(useSkillStore.getState().loading).toBe(false);
  });

  it("loadSkills hata durumunda error set eder", async () => {
    vi.mocked(invoke).mockRejectedValue(new Error("DB hatası"));

    await useSkillStore.getState().loadSkills();

    expect(useSkillStore.getState().error).toContain("DB hatası");
    expect(useSkillStore.getState().loading).toBe(false);
  });

  it("toggleSkill invoke ve loadSkills cagirir", async () => {
    vi.mocked(invoke).mockResolvedValue("Skill 'TestSkill' pasif edildi");
    vi.mocked(invoke).mockResolvedValueOnce("Skill 'TestSkill' pasif edildi");
    vi.mocked(invoke).mockResolvedValueOnce(mockSkills);

    await useSkillStore.getState().toggleSkill("TestSkill");

    expect(invoke).toHaveBeenCalledWith("toggle_skill", { name: "TestSkill" });
  });

  it("setPanel panel degistirir", () => {
    useSkillStore.getState().setPanel("add");
    expect(useSkillStore.getState().panel).toBe("add");
  });

  it("setSelected secili skill'i gunceller", () => {
    useSkillStore.getState().setSelected(mockSkills[0]);
    expect(useSkillStore.getState().selected).toEqual(mockSkills[0]);
  });

  it("setMdContent icerigi gunceller", () => {
    useSkillStore.getState().setMdContent("# Skill: X");
    expect(useSkillStore.getState().mdContent).toBe("# Skill: X");
  });

  it("clearResult ve clearError mesajlari temizler", () => {
    useSkillStore.setState({ result: "ok", error: "err" });
    useSkillStore.getState().clearResult();
    useSkillStore.getState().clearError();
    expect(useSkillStore.getState().result).toBe("");
    expect(useSkillStore.getState().error).toBe("");
  });
});
