use std::sync::{Arc, Mutex};
use rusqlite::{params, Connection};

use crate::llm::OllamaClient;
use crate::skill::{Skill, SkillStep};

pub struct SkillRegistry {
    conn: Arc<Mutex<Connection>>,
}

impl SkillRegistry {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        if let Ok(c) = conn.lock() {
            c.execute_batch(
                "CREATE TABLE IF NOT EXISTS skill_registry (
                    id INTEGER PRIMARY KEY,
                    name TEXT UNIQUE NOT NULL,
                    description TEXT DEFAULT '',
                    triggers TEXT DEFAULT '[]',
                    approval TEXT DEFAULT 'required',
                    steps TEXT DEFAULT '[]',
                    logic_code TEXT DEFAULT '',
                    evolution TEXT DEFAULT '[]',
                    run_count INTEGER DEFAULT 0,
                    active INTEGER DEFAULT 1,
                    version INTEGER DEFAULT 1,
                    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
                );"
            ).ok();
        }
        Self { conn }
    }

    pub fn register(&self, name: &str, description: &str, triggers: &[String],
                    approval: &str, steps: &[SkillStep], logic_code: Option<&str>,
                    evolution: &[String]) -> Result<i64, String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        let triggers_json = serde_json::to_string(triggers).map_err(|e| e.to_string())?;
        let steps_json = serde_json::to_string(steps).map_err(|e| e.to_string())?;
        let evolution_json = serde_json::to_string(evolution).map_err(|e| e.to_string())?;

        c.execute(
            "INSERT INTO skill_registry (name, description, triggers, approval, steps, logic_code, evolution)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(name) DO UPDATE SET
                description = ?2, triggers = ?3, approval = ?4,
                steps = ?5, logic_code = ?6, evolution = ?7,
                version = version + 1",
            params![name, description, triggers_json, approval, steps_json,
                    logic_code.unwrap_or(""), evolution_json],
        ).map_err(|e| e.to_string())?;

        Ok(c.last_insert_rowid())
    }

    pub fn find_by_trigger(&self, text: &str, ollama: Option<&OllamaClient>) -> Result<Vec<Skill>, String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = c.prepare(
            "SELECT id, name, description, triggers, approval, steps, logic_code, evolution, run_count, active, version, created_at
             FROM skill_registry WHERE active = 1"
        ).map_err(|e| e.to_string())?;

        let text_lower = text.to_lowercase();
        let rows = stmt.query_map([], Self::map_skill_row).map_err(|e| e.to_string())?;

        let skills: Vec<Skill> = rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

        let exact: Vec<Skill> = skills.iter()
            .filter(|s| s.triggers.iter().any(|t| text_lower.contains(&t.to_lowercase())))
            .cloned()
            .collect();

        if !exact.is_empty() {
            return Ok(exact);
        }

        if let Some(llm) = ollama {
            return self.semantic_match(text, &skills, llm);
        }

        Ok(Vec::new())
    }

    fn map_skill_row(row: &rusqlite::Row) -> rusqlite::Result<Skill> {
        Ok(Skill {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            triggers: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
            approval: row.get(4)?,
            steps: serde_json::from_str(&row.get::<_, String>(5)?).unwrap_or_default(),
            logic_code: {
                let code: String = row.get(6)?;
                if code.is_empty() { None } else { Some(code) }
            },
            evolution: serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or_default(),
            run_count: row.get(8)?,
            active: row.get::<_, i32>(9)? != 0,
            version: row.get(10)?,
            created_at: row.get(11)?,
        })
    }

    fn semantic_match(&self, text: &str, skills: &[Skill], ollama: &OllamaClient) -> Result<Vec<Skill>, String> {
        let mut scored: Vec<(i32, Skill)> = Vec::new();

        for skill in skills {
            let trigger_list = skill.triggers.join(", ");
            let prompt = format!(
                "Kullanici mesaji: \"{}\"\nSkill tetikleyicileri: [{}]\n\
                 Skill aciklamasi: {}\n\n\
                 Bu mesaj bu skill ile ilgili mi? Sadece 0-100 arasi bir sayi ver.",
                text, trigger_list, skill.description
            );

            match ollama.generate_sync(&prompt) {
                Ok(resp) => {
                    let num: i32 = resp.trim().chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse()
                        .unwrap_or(0);
                    if num > 50 {
                        scored.push((num, skill.clone()));
                    }
                }
                Err(_) => continue,
            }
        }

        scored.sort_by(|a, b| b.0.cmp(&a.0));
        Ok(scored.into_iter().map(|(_, s)| s).collect())
    }

    pub fn get_by_name(&self, name: &str) -> Result<Option<Skill>, String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = c.prepare(
            "SELECT id, name, description, triggers, approval, steps, logic_code, evolution, run_count, active, version, created_at
             FROM skill_registry WHERE name = ?1"
        ).map_err(|e| e.to_string())?;

        let mut rows = stmt.query_map(params![name], Self::map_skill_row)
            .map_err(|e| e.to_string())?;

        match rows.next() {
            Some(row) => Ok(Some(row.map_err(|e| e.to_string())?)),
            None => Ok(None),
        }
    }

    pub fn list(&self) -> Result<Vec<Skill>, String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = c.prepare(
            "SELECT id, name, description, triggers, approval, steps, logic_code, evolution, run_count, active, version, created_at
             FROM skill_registry ORDER BY name"
        ).map_err(|e| e.to_string())?;

        let rows = stmt.query_map([], Self::map_skill_row).map_err(|e| e.to_string())?;

        rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
    }

    pub fn increment_run_count(&self, name: &str) -> Result<(), String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        c.execute(
            "UPDATE skill_registry SET run_count = run_count + 1 WHERE name = ?1",
            params![name],
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_run_count(&self, name: &str) -> Result<i64, String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        c.query_row(
            "SELECT run_count FROM skill_registry WHERE name = ?1",
            params![name],
            |row| row.get(0),
        ).map_err(|e| e.to_string())
    }

    pub fn count(&self) -> Result<i64, String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        c.query_row("SELECT COUNT(*) FROM skill_registry", [], |row| row.get(0))
            .map_err(|e| e.to_string())
    }

    pub fn conn_clone(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }

    pub fn remove(&self, name: &str) -> Result<(), String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        c.execute("DELETE FROM skill_registry WHERE name = ?1", params![name])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn activate(&self, name: &str) -> Result<(), String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        c.execute("UPDATE skill_registry SET active = 1 WHERE name = ?1", params![name])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn deactivate(&self, name: &str) -> Result<(), String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        c.execute("UPDATE skill_registry SET active = 0 WHERE name = ?1", params![name])
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_version(&self, name: &str) -> Result<i32, String> {
        let c = self.conn.lock().map_err(|e| e.to_string())?;
        c.query_row(
            "SELECT version FROM skill_registry WHERE name = ?1",
            params![name],
            |row| row.get(0),
        ).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db;

    fn test_registry() -> SkillRegistry {
        let conn = db::open(std::path::Path::new(":memory:")).unwrap();
        SkillRegistry::new(conn)
    }

    #[test]
    fn test_register_and_get() {
        let reg = test_registry();
        let id = reg.register("test_skill", "Bir test skill", &["selam".into()], "auto", &[], None, &[]).unwrap();
        assert!(id > 0);

        let skill = reg.get_by_name("test_skill").unwrap().unwrap();
        assert_eq!(skill.name, "test_skill");
        assert_eq!(skill.triggers, vec!["selam"]);
        assert_eq!(skill.version, 1);
        assert!(skill.active);
    }

    #[test]
    fn test_re_register_bumps_version() {
        let reg = test_registry();
        reg.register("vskill", "", &["a".into()], "auto", &[], None, &[]).unwrap();
        reg.register("vskill", "guncel", &["b".into()], "required", &[], None, &[]).unwrap();
        let skill = reg.get_by_name("vskill").unwrap().unwrap();
        assert_eq!(skill.version, 2);
        assert_eq!(skill.description, "guncel");
    }

    #[test]
    fn test_activate_deactivate() {
        let reg = test_registry();
        reg.register("tog", "", &["x".into()], "auto", &[], None, &[]).unwrap();

        let skill = reg.get_by_name("tog").unwrap().unwrap();
        assert!(skill.active);

        reg.deactivate("tog").unwrap();
        let skill = reg.get_by_name("tog").unwrap().unwrap();
        assert!(!skill.active);

        reg.activate("tog").unwrap();
        let skill = reg.get_by_name("tog").unwrap().unwrap();
        assert!(skill.active);
    }

    #[test]
    fn test_find_by_trigger_substring() {
        let reg = test_registry();
        reg.register("btc", "Bitcoin analizi", &["bitcoin".into(), "btc".into()], "auto", &[], None, &[]).unwrap();
        reg.register("eth", "Ethereum analizi", &["ethereum".into(), "eth".into()], "auto", &[], None, &[]).unwrap();

        let matched = reg.find_by_trigger("BTC fiyati nedir?", None).unwrap();
        assert!(matched.iter().any(|s| s.name == "btc"));
        assert!(!matched.iter().any(|s| s.name == "eth"));
    }

    #[test]
    fn test_find_by_trigger_inactive() {
        let reg = test_registry();
        reg.register("inaktif", "", &["test".into()], "auto", &[], None, &[]).unwrap();
        reg.deactivate("inaktif").unwrap();

        let matched = reg.find_by_trigger("test mesaji", None).unwrap();
        assert!(matched.is_empty());
    }

    #[test]
    fn test_increment_run_count() {
        let reg = test_registry();
        reg.register("counter", "", &["say".into()], "auto", &[], None, &[]).unwrap();
        assert_eq!(reg.get_run_count("counter").unwrap(), 0);
        reg.increment_run_count("counter").unwrap();
        assert_eq!(reg.get_run_count("counter").unwrap(), 1);
        reg.increment_run_count("counter").unwrap();
        assert_eq!(reg.get_run_count("counter").unwrap(), 2);
    }

    #[test]
    fn test_remove() {
        let reg = test_registry();
        reg.register("silinecek", "", &["x".into()], "auto", &[], None, &[]).unwrap();
        reg.remove("silinecek").unwrap();
        assert!(reg.get_by_name("silinecek").unwrap().is_none());
    }

    #[test]
    fn test_list() {
        let reg = test_registry();
        reg.register("a", "", &["a".into()], "auto", &[], None, &[]).unwrap();
        reg.register("b", "", &["b".into()], "auto", &[], None, &[]).unwrap();
        let skills = reg.list().unwrap();
        assert_eq!(skills.len(), 2);
    }

    #[test]
    fn test_count() {
        let reg = test_registry();
        assert_eq!(reg.count().unwrap(), 0);
        reg.register("c1", "", &["x".into()], "auto", &[], None, &[]).unwrap();
        assert_eq!(reg.count().unwrap(), 1);
    }
}
