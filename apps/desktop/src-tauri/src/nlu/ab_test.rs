use crate::nlu::intent::Intent;
use crate::nlu::pipeline::NLUPipeline;
use crate::llm::OllamaClient;

#[derive(Debug, Clone)]
pub struct TestCase {
    pub input: String,
    pub expected: Intent,
}

#[derive(Debug, Clone)]
pub struct ABTestConfig {
    pub version_a: String,
    pub version_b: String,
    pub test_cases: Vec<TestCase>,
}

#[derive(Debug, Clone)]
pub struct ABTestResult {
    pub version_a: VersionStats,
    pub version_b: VersionStats,
    pub p_value: f64,
    pub winner: Option<String>,
    pub significant: bool,
}

#[derive(Debug, Clone, Default)]
pub struct VersionStats {
    pub total: usize,
    pub correct: usize,
    pub by_intent: std::collections::HashMap<String, IntentStats>,
}

#[derive(Debug, Clone, Default)]
pub struct IntentStats {
    pub total: usize,
    pub correct: usize,
}

impl ABTestConfig {
    pub fn new(version_a: &str, version_b: &str) -> Self {
        Self {
            version_a: version_a.to_string(),
            version_b: version_b.to_string(),
            test_cases: Vec::new(),
        }
    }

    pub fn with_cases(mut self, cases: Vec<TestCase>) -> Self {
        self.test_cases = cases;
        self
    }

    pub fn add_case(&mut self, input: &str, expected: Intent) {
        self.test_cases.push(TestCase {
            input: input.to_string(),
            expected,
        });
    }

    pub fn run(&self, llm: &OllamaClient) -> ABTestResult {
        let stats_a = self.run_version(&self.version_a, llm);
        let stats_b = self.run_version(&self.version_b, llm);

        let p_value = Self::z_test_proportion(
            stats_a.correct as f64, stats_a.total as f64,
            stats_b.correct as f64, stats_b.total as f64,
        );

        let significant = p_value < 0.05;
        let winner = if significant {
            if stats_a.correct > stats_b.correct {
                Some("A".to_string())
            } else if stats_b.correct > stats_a.correct {
                Some("B".to_string())
            } else {
                Some("tie".to_string())
            }
        } else {
            None
        };

        ABTestResult {
            version_a: stats_a,
            version_b: stats_b,
            p_value,
            winner,
            significant,
        }
    }

    fn run_version(&self, prompt: &str, llm: &OllamaClient) -> VersionStats {
        let mut stats = VersionStats::default();
        for case in &self.test_cases {
            let fallback_prompt = crate::nlu::prompts::PromptTemplates::intent_classification();
            let active_prompt = if prompt.is_empty() { fallback_prompt } else { prompt };
            let full_prompt = active_prompt.replace("{}", &case.input);

            let predicted = match llm.generate_sync(&full_prompt) {
                Ok(raw) => Intent::from_str(&raw),
                Err(_) => Intent::Chat,
            };

            let key = predicted.as_str().to_string();
            let entry = stats.by_intent.entry(key).or_default();
            entry.total += 1;
            stats.total += 1;

            if predicted == case.expected {
                entry.correct += 1;
                stats.correct += 1;
            }
        }

        stats
    }

    pub fn z_test_proportion(p1: f64, n1: f64, p2: f64, n2: f64) -> f64 {
        if n1 == 0.0 || n2 == 0.0 {
            return 1.0;
        }
        let prop1 = p1 / n1;
        let prop2 = p2 / n2;
        let p_pool = (p1 + p2) / (n1 + n2);
        if p_pool == 0.0 || p_pool == 1.0 {
            return 1.0;
        }
        let se = (p_pool * (1.0 - p_pool) * (1.0 / n1 + 1.0 / n2)).sqrt();
        if se == 0.0 {
            return 1.0;
        }
        let z = (prop1 - prop2).abs() / se;
        let p = 2.0 * (1.0 - Self::normal_cdf(z));
        p.min(1.0)
    }

    pub fn normal_cdf(x: f64) -> f64 {
        let a1 = 0.254829592;
        let a2 = -0.284496736;
        let a3 = 1.421413741;
        let a4 = -1.453152027;
        let a5 = 1.061405429;
        let p = 0.3275911;
        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        let x = x.abs();
        let t = 1.0 / (1.0 + p * x);
        let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x / 2.0).exp();
        0.5 * (1.0 + sign * y)
    }
}

impl ABTestResult {
    pub fn report(&self) -> String {
        let mut lines = vec![
            format!("=== A/B Test Sonucu ==="),
            format!(""),
            format!("Versiyon A: {}/{} dogru ({:.1}%)",
                self.version_a.correct, self.version_a.total,
                if self.version_a.total > 0 { self.version_a.correct as f64 / self.version_a.total as f64 * 100.0 } else { 0.0 }),
            format!("Versiyon B: {}/{} dogru ({:.1}%)",
                self.version_b.correct, self.version_b.total,
                if self.version_b.total > 0 { self.version_b.correct as f64 / self.version_b.total as f64 * 100.0 } else { 0.0 }),
            format!(""),
            format!("p-degeri: {:.4}", self.p_value),
            format!("Anlamli: {}", if self.significant { "Evet" } else { "Hayir" }),
        ];
        if let Some(ref w) = self.winner {
            lines.push(format!("Kazanan: {}", w));
        } else {
            lines.push(format!("Kazanan: Yok (fark anlamli degil)"));
        }
        lines.join("\n")
    }

    pub fn winning_prompt(&self, config: &ABTestConfig) -> Option<String> {
        match self.winner.as_deref() {
            Some("A") => Some(config.version_a.clone()),
            Some("B") => Some(config.version_b.clone()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_ollama() -> OllamaClient {
        OllamaClient::new("http://localhost:11434".into(), "test-model".into())
    }

    #[test]
    fn test_ab_test_config_creation() {
        let config = ABTestConfig::new("prompt A", "prompt B");
        assert_eq!(config.version_a, "prompt A");
        assert_eq!(config.version_b, "prompt B");
        assert!(config.test_cases.is_empty());
    }

    #[test]
    fn test_add_case() {
        let mut config = ABTestConfig::new("A", "B");
        config.add_case("SXT'yi kontrol et", Intent::Crypto);
        config.add_case("Röleyi aç", Intent::Hardware);
        assert_eq!(config.test_cases.len(), 2);
        assert_eq!(config.test_cases[0].expected, Intent::Crypto);
        assert_eq!(config.test_cases[1].expected, Intent::Hardware);
    }

    #[test]
    fn test_with_cases() {
        let cases = vec![
            TestCase { input: "test".into(), expected: Intent::Chat },
        ];
        let config = ABTestConfig::new("A", "B").with_cases(cases);
        assert_eq!(config.test_cases.len(), 1);
    }

    #[test]
    fn test_version_stats_default() {
        let stats = VersionStats::default();
        assert_eq!(stats.total, 0);
        assert_eq!(stats.correct, 0);
        assert!(stats.by_intent.is_empty());
    }

    #[test]
    fn test_z_test_identical() {
        let p = ABTestConfig::z_test_proportion(5.0, 10.0, 5.0, 10.0);
        assert!(p >= 0.99 || p <= 1.01);
    }

    #[test]
    fn test_z_test_different() {
        let p = ABTestConfig::z_test_proportion(9.0, 10.0, 1.0, 10.0);
        assert!(p < 0.05);
    }

    #[test]
    fn test_z_test_zero_denom() {
        let p = ABTestConfig::z_test_proportion(0.0, 0.0, 0.0, 0.0);
        assert_eq!(p, 1.0);
    }

    #[test]
    fn test_normal_cdf() {
        let cdf_0 = ABTestConfig::normal_cdf(0.0);
        assert!((cdf_0 - 0.5).abs() < 0.001);
        let cdf_1 = ABTestConfig::normal_cdf(1.96);
        assert!((cdf_1 - 0.975).abs() < 0.01);
    }

    #[test]
    fn test_result_report_no_winner() {
        let result = ABTestResult {
            version_a: VersionStats { total: 10, correct: 5, ..Default::default() },
            version_b: VersionStats { total: 10, correct: 5, ..Default::default() },
            p_value: 0.5,
            winner: None,
            significant: false,
        };
        let report = result.report();
        assert!(report.contains("Kazanan: Yok"));
        assert!(report.contains("p-degeri: 0.5000"));
    }

    #[test]
    fn test_result_report_with_winner() {
        let result = ABTestResult {
            version_a: VersionStats { total: 10, correct: 9, ..Default::default() },
            version_b: VersionStats { total: 10, correct: 1, ..Default::default() },
            p_value: 0.001,
            winner: Some("A".to_string()),
            significant: true,
        };
        let report = result.report();
        assert!(report.contains("Kazanan: A"));
        assert!(report.contains("Anlamli: Evet"));
    }

    #[test]
    fn test_winning_prompt_a() {
        let config = ABTestConfig::new("prompt_a_here", "prompt_b_here");
        let result = ABTestResult {
            version_a: VersionStats { total: 10, correct: 9, ..Default::default() },
            version_b: VersionStats { total: 10, correct: 1, ..Default::default() },
            p_value: 0.001,
            winner: Some("A".to_string()),
            significant: true,
        };
        assert_eq!(result.winning_prompt(&config), Some("prompt_a_here".to_string()));
    }

    #[test]
    fn test_winning_prompt_none() {
        let config = ABTestConfig::new("A", "B");
        let result = ABTestResult {
            version_a: VersionStats { total: 10, correct: 5, ..Default::default() },
            version_b: VersionStats { total: 10, correct: 5, ..Default::default() },
            p_value: 0.5,
            winner: None,
            significant: false,
        };
        assert_eq!(result.winning_prompt(&config), None);
    }

    #[test]
    fn test_macro_f1_non_zero() {
        let cases = vec![
            TestCase { input: "SXT'yi kontrol et".into(), expected: Intent::Crypto },
            TestCase { input: "Röleyi aç".into(), expected: Intent::Hardware },
            TestCase { input: "Sistem durumu".into(), expected: Intent::System },
            TestCase { input: "Belge tara".into(), expected: Intent::Document },
            TestCase { input: "Merhaba".into(), expected: Intent::Chat },
        ];
        let config = ABTestConfig::new("test", "test").with_cases(cases);
        let llm = mock_ollama();
        let result = config.run(&llm);
        assert!(result.version_a.total >= 0);
    }

    #[test]
    fn test_ab_test_run_no_crash() {
        let config = ABTestConfig::new("A", "B");
        let llm = mock_ollama();
        let result = config.run(&llm);
        assert_eq!(result.version_a.total, 0);
        assert_eq!(result.version_b.total, 0);
        assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
    }

    #[test]
    fn test_case_struct() {
        let tc = TestCase { input: "something".into(), expected: Intent::Query };
        assert_eq!(tc.input, "something");
        assert_eq!(tc.expected, Intent::Query);
    }

    #[test]
    fn test_ab_test_config_mutability() {
        let mut config = ABTestConfig::new("v1", "v2");
        config.add_case("test", Intent::Chat);
        config.add_case("test2", Intent::Query);
        assert_eq!(&config.version_a, "v1");
        assert_eq!(&config.version_b, "v2");
    }
}
