use std::collections::HashMap;

pub struct PromptTemplate {
    pub template: String,
    pub variables: Vec<String>,
}

pub struct PromptEngine {
    templates: HashMap<String, PromptTemplate>,
}

impl PromptEngine {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        templates.insert("intent".into(), PromptTemplate {
            template: "Kullanıcı mesajını tek bir kategoriye sınıflandır.\nKategoriler: sorgu, eylem, analiz, donanım, kripto, sistem, doküman, ses\nSadece kategori adını yaz.\nMesaj: {{input}}".into(),
            variables: vec!["input".into()],
        });
        templates.insert("analyze".into(), PromptTemplate {
            template: "Aşağıdaki veriyi analiz et ve kısa bir özet çıkar:\n\n{{input}}".into(),
            variables: vec!["input".into()],
        });
        templates.insert("summarize".into(), PromptTemplate {
            template: "Aşağıdaki metni 3 cümleyle özetle:\n\n{{input}}".into(),
            variables: vec!["input".into()],
        });
        templates.insert("system_status".into(), PromptTemplate {
            template: "Sistem durumu: CPU {{cpu}}%, RAM {{ram}}GB/{{total_ram}}GB, uptime {{uptime}}sn\nKısa bir durum raporu hazırla.".into(),
            variables: vec!["cpu".into(), "ram".into(), "total_ram".into(), "uptime".into()],
        });
        Self { templates }
    }

    pub fn render(&self, name: &str, vars: &HashMap<&str, String>) -> Result<String, String> {
        let tpl = self.templates.get(name)
            .ok_or_else(|| format!("Template '{}' bulunamadi", name))?;
        let mut result = tpl.template.clone();
        for var in &tpl.variables {
            let val = vars.get(var.as_str())
                .ok_or_else(|| format!("Template '{}' icin '{}' degiskeni gerekli", name, var))?;
            result = result.replace(&format!("{{{{{}}}}}", var), val);
        }
        Ok(result)
    }

    pub fn register(&mut self, name: &str, template: &str, variables: Vec<String>) {
        self.templates.insert(name.into(), PromptTemplate {
            template: template.into(),
            variables,
        });
    }

    pub fn list_templates(&self) -> Vec<String> {
        self.templates.keys().cloned().collect()
    }
}
