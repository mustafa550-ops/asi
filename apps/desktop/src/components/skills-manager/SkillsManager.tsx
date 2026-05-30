import { useEffect } from "react";
import { useSkillStore } from "../../stores/skillStore";
import type { Skill } from "../../stores/skillStore";

export default function SkillsManager() {
  const skills = useSkillStore((s) => s.skills);
  const loading = useSkillStore((s) => s.loading);
  const error = useSkillStore((s) => s.error);
  const panel = useSkillStore((s) => s.panel);
  const selected = useSkillStore((s) => s.selected);
  const mdContent = useSkillStore((s) => s.mdContent);
  const result = useSkillStore((s) => s.result);

  const loadSkills = useSkillStore((s) => s.loadSkills);
  const toggleSkill = useSkillStore((s) => s.toggleSkill);
  const deleteSkill = useSkillStore((s) => s.deleteSkill);
  const runSkill = useSkillStore((s) => s.runSkill);
  const addSkill = useSkillStore((s) => s.addSkill);
  const setPanel = useSkillStore((s) => s.setPanel);
  const setSelected = useSkillStore((s) => s.setSelected);
  const setMdContent = useSkillStore((s) => s.setMdContent);
  const clearResult = useSkillStore((s) => s.clearResult);
  const clearError = useSkillStore((s) => s.clearError);

  useEffect(() => {
    loadSkills();
  }, [loadSkills]);

  useEffect(() => {
    if (result) {
      const timer = setTimeout(clearResult, 5000);
      return () => clearTimeout(timer);
    }
  }, [result, clearResult]);

  const goToList = () => {
    clearResult();
    clearError();
    setPanel("list");
  };

  if (loading) {
    return <div className="skills-manager"><p>Yetenekler yukleniyor...</p></div>;
  }

  if (panel === "add") {
    return (
      <div className="skills-manager">
        <div className="skills-header">
          <h2>Yeni Yetenek Ekle</h2>
          <button className="btn-back" onClick={goToList}>Geri</button>
        </div>
        <div className="add-skill-area">
          <textarea
            className="add-skill-textarea"
            rows={16}
            placeholder="Skill manifestosu (.md) icerigini yapistirin..."
            value={mdContent}
            onChange={(e) => setMdContent(e.target.value)}
          />
          <button className="btn-add" onClick={addSkill} disabled={!mdContent.trim()}>
            Ekle
          </button>
        </div>
        {result && <pre className="skill-result">{result}</pre>}
        {error && <pre className="skill-error">{error}</pre>}
      </div>
    );
  }

  if (panel === "detail" && selected) {
    return (
      <div className="skills-manager">
        <div className="skills-header">
          <h2>{selected.name}</h2>
          <button className="btn-back" onClick={goToList}>Geri</button>
        </div>
        <div className="skill-detail">
          <p><strong>Açiklama:</strong> {selected.description || "(yok)"}</p>
          <p><strong>Sürüm:</strong> v{selected.version}</p>
          <p><strong>Çalışma:</strong> {selected.run_count} kez</p>
          <p><strong>Onay:</strong> {selected.approval}</p>
          <p><strong>Durum:</strong> {selected.active ? "Aktif" : "Pasif"}</p>
          <p><strong>Tetikleyiciler:</strong> {selected.triggers.join(", ") || "(yok)"}</p>
          <div className="skill-detail-section">
            <strong>Adimlar:</strong>
            {selected.steps.length === 0 ? <span> (yok)</span> : (
              <ol>{selected.steps.map((s) => <li key={s.order}>{s.description}</li>)}</ol>
            )}
          </div>
          {selected.logic_code && (
            <div className="skill-detail-section">
              <strong>Kod:</strong>
              <pre className="skill-code">{selected.logic_code}</pre>
            </div>
          )}
          {selected.evolution.length > 0 && (
            <div className="skill-detail-section">
              <strong>Gelişim Geçmişi:</strong>
              <ul>{selected.evolution.map((e, i) => <li key={i}>{e}</li>)}</ul>
            </div>
          )}
          <div className="skill-detail-actions">
            <button
              className={`btn-toggle ${selected.active ? "btn-deactivate" : "btn-activate"}`}
              onClick={() => toggleSkill(selected.name)}
            >
              {selected.active ? "Pasif Yap" : "Aktif Yap"}
            </button>
            <button className="btn-run" onClick={() => runSkill(selected.name)}>Çaliştir</button>
            <button className="btn-delete" onClick={() => deleteSkill(selected.name)}>Sil</button>
          </div>
        </div>
        {result && <pre className="skill-result">{result}</pre>}
        {error && <pre className="skill-error">{error}</pre>}
      </div>
    );
  }

  return (
    <div className="skills-manager">
      <div className="skills-header">
        <h2>Yetenekler ({skills.length})</h2>
        <button className="btn-add" onClick={() => { setPanel("add"); clearError(); clearResult(); }}>
          + Yeni Ekle
        </button>
      </div>
      {error && <pre className="skill-error">{error}</pre>}
      {result && <pre className="skill-result">{result}</pre>}
      {skills.length === 0 ? (
        <p className="skills-empty">Henüz yetenek eklenmemiş. Yeni bir yetenek eklemek için "+ Yeni Ekle" butonunu kullanin.</p>
      ) : (
        <div className="skills-table">
          {skills.map((skill: Skill) => (
            <div key={skill.name} className="skill-row" onClick={() => { setSelected(skill); setPanel("detail"); }}>
              <div className="skill-row-info">
                <span className="skill-name">{skill.name}</span>
                <span className="skill-meta">
                  {skill.triggers.length} tetikleyici &middot; {skill.run_count} çalişma &middot; v{skill.version}
                </span>
              </div>
              <div className="skill-row-actions" onClick={(e) => e.stopPropagation()}>
                {skill.active
                  ? <span className="skill-badge active">Aktif</span>
                  : <span className="skill-badge inactive">Pasif</span>
                }
                <button className="btn-toggle-sm" onClick={() => toggleSkill(skill.name)} title={skill.active ? "Pasif yap" : "Aktif yap"}>
                  {skill.active ? "Pasif" : "Aktif"}
                </button>
                <button className="btn-run-sm" onClick={() => runSkill(skill.name)} title="Çaliştir">Çaliştir</button>
                <button className="btn-delete-sm" onClick={() => deleteSkill(skill.name)} title="Sil">Sil</button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
