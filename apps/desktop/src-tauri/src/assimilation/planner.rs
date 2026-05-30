use crate::assimilation::analyzer::Analysis;

#[derive(Debug)]
pub struct IntegrationPlan {
    pub steps: Vec<IntegrationStep>,
    pub estimated_impact: String,
    pub risk_level: String,
}

#[derive(Debug)]
pub struct IntegrationStep {
    pub order: u32,
    pub description: String,
    pub module: String,
    pub estimated_effort: String,
}

pub struct IntegrationPlanner;

impl IntegrationPlanner {
    pub fn plan(analysis: &Analysis, name: &str) -> IntegrationPlan {
        let mut steps = Vec::new();

        match analysis.language.as_str() {
            "Rust" => {
                steps.push(IntegrationStep {
                    order: 1,
                    description: format!("{} kaynak kodunu ADLER Rust çekirdeğine entegre et", name),
                    module: "core".into(),
                    estimated_effort: "medium".into(),
                });
                steps.push(IntegrationStep {
                    order: 2,
                    description: "Tauri bridge komutlarini tanimla".into(),
                    module: "bridge".into(),
                    estimated_effort: "low".into(),
                });
            }
            "TypeScript" | "JavaScript" => {
                steps.push(IntegrationStep {
                    order: 1,
                    description: format!("{} UI bilesenlerini React'e uyarla", name),
                    module: "ui".into(),
                    estimated_effort: "medium".into(),
                });
                steps.push(IntegrationStep {
                    order: 2,
                    description: "Tauri invoke wrapper'lari olustur".into(),
                    module: "bridge".into(),
                    estimated_effort: "low".into(),
                });
            }
            "Python" => {
                steps.push(IntegrationStep {
                    order: 1,
                    description: "Python kodunu Wasm sandbox'ta calistir".into(),
                    module: "sandbox".into(),
                    estimated_effort: "high".into(),
                });
                steps.push(IntegrationStep {
                    order: 2,
                    description: "Python-Rust FFI koprusu kur".into(),
                    module: "bridge".into(),
                    estimated_effort: "high".into(),
                });
            }
            _ => {
                steps.push(IntegrationStep {
                    order: 1,
                    description: format!("{} icin ozel entegrasyon plani", name),
                    module: "custom".into(),
                    estimated_effort: "unknown".into(),
                });
            }
        }

        let estimated_impact = match analysis.file_count {
            0..=5 => "dusuk".into(),
            6..=20 => "orta".into(),
            _ => "yuksek".into(),
        };
        let risk_level = match analysis.language.as_str() {
            "Rust" => "dusuk".into(),
            "TypeScript" => "dusuk".into(),
            "Python" => "orta".into(),
            _ => "yuksek".into(),
        };

        IntegrationPlan { steps, estimated_impact, risk_level }
    }
}
