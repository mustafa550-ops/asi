use crate::skill::Skill;
use std::collections::HashMap;

pub struct SkillMarket {
    skills: Vec<Skill>,
}

impl SkillMarket {
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }

    pub fn add(&mut self, skill: Skill) {
        self.skills.push(skill);
    }

    pub fn add_all(&mut self, skills: Vec<Skill>) {
        self.skills.extend(skills);
    }

    pub fn search(&self, query: &str) -> Vec<&Skill> {
        let q = query.to_lowercase();
        self.skills.iter()
            .filter(|s| {
                s.name.to_lowercase().contains(&q)
                    || s.description.to_lowercase().contains(&q)
                    || s.triggers.iter().any(|t| t.to_lowercase().contains(&q))
                    || s.tags.iter().any(|t| t.to_lowercase().contains(&q))
            })
            .collect()
    }

    pub fn by_category(&self, category: &str) -> Vec<&Skill> {
        self.skills.iter()
            .filter(|s| s.category.to_lowercase() == category.to_lowercase())
            .collect()
    }

    pub fn top_rated(&self, limit: usize) -> Vec<&Skill> {
        let mut sorted: Vec<&Skill> = self.skills.iter()
            .filter(|s| s.rating_count > 0)
            .collect();
        sorted.sort_by(|a, b| {
            b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted.truncate(limit);
        sorted
    }

    pub fn most_run(&self, limit: usize) -> Vec<&Skill> {
        let mut sorted: Vec<&Skill> = self.skills.iter().collect();
        sorted.sort_by(|a, b| b.run_count.cmp(&a.run_count));
        sorted.truncate(limit);
        sorted
    }

    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self.skills.iter()
            .map(|s| s.category.clone())
            .filter(|c| !c.is_empty())
            .collect();
        cats.sort();
        cats.dedup();
        cats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skill::SkillStep;

    fn make_skill(name: &str, cat: &str, rating: f64, rating_count: i64, run_count: i64) -> Skill {
        Skill {
            id: 0, name: name.into(), description: format!("[{}] description", name),
            triggers: vec![format!("trigger_{}", name.to_lowercase())], approval: "auto".into(),
            steps: vec![SkillStep { order: 1, description: "Step".into() }],
            logic_code: None, evolution: vec![], run_count, active: true,
            version: 1, created_at: "0".into(), category: cat.into(),
            tags: vec![format!("tag_{}", cat)], rating, rating_count,
        }
    }

    #[test]
    fn empty_market_returns_empty() {
        let m = SkillMarket::new();
        assert!(m.search("test").is_empty());
        assert!(m.by_category("finance").is_empty());
    }

    #[test]
    fn search_finds_by_name() {
        let mut m = SkillMarket::new();
        m.add(make_skill("Borsa_Analiz", "finance", 4.0, 10, 100));
        let results = m.search("Borsa");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn search_finds_by_trigger() {
        let mut m = SkillMarket::new();
        m.add(make_skill("Test", "gen", 3.0, 5, 10));
        let results = m.search("trigger_test");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn search_finds_by_tag() {
        let mut m = SkillMarket::new();
        m.add(make_skill("Skill_X", "finance", 4.0, 10, 50));
        let results = m.search("tag_finance");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn search_case_insensitive() {
        let mut m = SkillMarket::new();
        m.add(make_skill("Borsa_Analiz", "finance", 4.0, 10, 100));
        let results = m.search("borsa");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn by_category_filters() {
        let mut m = SkillMarket::new();
        m.add(make_skill("Skill_A", "finance", 4.0, 10, 100));
        m.add(make_skill("Skill_B", "system", 3.0, 5, 50));
        assert_eq!(m.by_category("finance").len(), 1);
        assert_eq!(m.by_category("system").len(), 1);
    }

    #[test]
    fn top_rated_returns_highest_first() {
        let mut m = SkillMarket::new();
        m.add(make_skill("Low", "gen", 2.0, 10, 10));
        m.add(make_skill("High", "gen", 5.0, 10, 10));
        let top = m.top_rated(1);
        assert_eq!(top[0].name, "High");
    }

    #[test]
    fn top_rated_ignores_unrated() {
        let mut m = SkillMarket::new();
        m.add(make_skill("A", "gen", 0.0, 0, 10));
        m.add(make_skill("B", "gen", 4.0, 5, 10));
        assert_eq!(m.top_rated(10).len(), 1);
    }

    #[test]
    fn most_run_returns_most_frequent() {
        let mut m = SkillMarket::new();
        m.add(make_skill("Popular", "gen", 3.0, 5, 200));
        m.add(make_skill("Quiet", "gen", 4.0, 10, 5));
        let top = m.most_run(1);
        assert_eq!(top[0].name, "Popular");
    }

    #[test]
    fn categories_lists_unique() {
        let mut m = SkillMarket::new();
        m.add(make_skill("A", "finance", 4.0, 10, 10));
        m.add(make_skill("B", "finance", 3.0, 5, 10));
        m.add(make_skill("C", "system", 5.0, 10, 10));
        let cats = m.categories();
        assert_eq!(cats.len(), 2);
        assert!(cats.contains(&"finance".to_string()));
        assert!(cats.contains(&"system".to_string()));
    }

    #[test]
    fn add_all_bulk() {
        let mut m = SkillMarket::new();
        m.add_all(vec![
            make_skill("XYZ_Alpha", "finance", 3.0, 5, 10),
            make_skill("Beta_Util", "system", 4.0, 8, 20),
        ]);
        assert_eq!(m.search("Alpha").len(), 1);
        assert_eq!(m.search("Beta").len(), 1);
    }
}
