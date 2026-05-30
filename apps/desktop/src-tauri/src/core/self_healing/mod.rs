pub mod log_analyzer;
pub mod patch;
pub mod git;

use std::sync::Mutex;

pub struct SelfHealingEngine {
    fix_count: Mutex<u32>,
}

impl SelfHealingEngine {
    pub fn new() -> Self {
        Self {
            fix_count: Mutex::new(0),
        }
    }

    pub fn dry_run(&self, code: &str, wasm: &super::wasm_sandbox::WasmSandbox) -> Result<String, String> {
        log::info!("SelfHealing: dry-run basladi ({} bytes)", code.len());

        let sys = sysinfo::System::new_all();
        let mem_before = sys.used_memory();

        let wasm_bytes = code.as_bytes();
        match wasm.execute(wasm_bytes) {
            Ok(output) => {
                let mem_after = {
                    let sys2 = sysinfo::System::new_all();
                    sys2.used_memory()
                };
                let mem_delta = if mem_after > mem_before { mem_after - mem_before } else { 0 };
                Ok(format!("Dry-run basarili — output: {}, memory delta: {}MB", output.trim(), mem_delta / 1024 / 1024))
            }
            Err(e) => {
                let analysis = log_analyzer::analyze(&e);
                Err(format!("Dry-run basarisiz: {}\nAnaliz: {}", e, analysis))
            }
        }
    }

    pub fn diagnose(&self, error: &str) -> Option<String> {
        let suggestion = self.classify_error(error);
        if suggestion.is_some() {
            return suggestion;
        }
        None
    }

    pub fn diagnose_with_memory(&self, error: &str, memory: &crate::MemoryManager) -> Result<String, String> {
        let analysis = log_analyzer::analyze(error);
        let mut report = String::new();

        let suggestion = self.classify_error(error);
        match suggestion {
            Some(s) => report.push_str(&format!("Tani: {}\nOnem: {}\n\n", s, self.severity(&analysis))),
            None => report.push_str("Tani: Bilinmeyen hata\n\n"),
        }

        let similar = memory.get_similar_decisions(error, 3)?;
        if !similar.is_empty() {
            report.push_str("Gecmis benzer kararlar:\n");
            for r in &similar {
                let outcome_icon = match r.outcome.as_str() {
                    "success" => "✓",
                    "failure" => "✗",
                    _ => "~",
                };
                report.push_str(&format!("  {} Karar: {} | Sonuc: {} | Guven: {:.0}%\n",
                    outcome_icon, r.decision, r.outcome, r.confidence * 100.0));
            }
            report.push('\n');
        }

        let high_conf = memory.get_high_confidence_decisions(0.8, 3)?;
        if !high_conf.is_empty() {
            report.push_str("Onerilen cozumler (yuksel guven):\n");
            for r in &high_conf {
                report.push_str(&format!("  - {} (guven: {:.0}%)\n", r.decision, r.confidence * 100.0));
            }
        }

        Ok(report)
    }

    pub fn suggest_optimizations(&self) -> Vec<String> {
        let mut tips = Vec::new();

        let sys = sysinfo::System::new_all();
        let mem_used = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let cpus = sys.cpus();
        let cpu_count = cpus.len();
        let avg_load: f32 = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpu_count as f32;

        if mem_used > 12.0 {
            tips.push(format!("RAM kullanimi yuksek ({:.1}GB). Bellek yonetimini optimize edin.", mem_used));
        }
        if avg_load > 80.0 {
            tips.push(format!("CPU yuku kritik (%{:.0}). Paralel islem sayisini azaltin.", avg_load));
        }
        if cpu_count >= 8 && avg_load < 20.0 {
            tips.push(format!("{} cekirdegin %{:.0}'si kullaniliyor. Paralel calisma potansiyeli var.", cpu_count, avg_load));
        }

        tips
    }

    fn classify_error(&self, error: &str) -> Option<String> {
        let analysis = log_analyzer::analyze(error);
        if analysis == "bilinmeyen hata" {
            return None;
        }
        match analysis.as_str() {
            "module_not_found" => Some("Eksik modul import edilmis. Modul adini kontrol edin veya Cargo.toml'a ekleyin.".into()),
            "syntax_error" => Some("Yazim hatasi tespit edildi. Sozdizimini kontrol edin.".into()),
            "memory_error" => Some("Bellek sorunu. Daha fazla RAM ayirin veya veri yapisini kucultun.".into()),
            "type_mismatch" => Some("Tip uyusmazligi. Degisken tiplerini kontrol edin.".into()),
            "connection_error" => Some("Baglanti hatasi. Ag baglantisi veya API erisilebilirligini kontrol edin.".into()),
            "permission_error" => Some("Yetki hatasi. Dosya veya klasor izinlerini kontrol edin.".into()),
            "not_found" => Some("Kaynak bulunamadi. Dosya yolunu veya URL'yi kontrol edin.".into()),
            "panic" => Some("Panik olustu. unwrap() veya expect() kullanimini gozden gecirin.".into()),
            _ => None,
        }
    }

    fn severity(&self, analysis: &str) -> &str {
        match analysis {
            "panic" | "memory_error" => "KRITIK",
            "module_not_found" | "type_mismatch" => "YUKSEK",
            "connection_error" | "permission_error" => "ORTA",
            _ => "DUSUK",
        }
    }

    pub fn auto_fix(&self, code: &str, error: &str) -> Result<String, String> {
        let fix = patch::generate_fix(code, error)?;
        log::info!("SelfHealing: patch uygulandi — {} bayt degisiklik", fix.len());

        let mut count = self.fix_count.lock().map_err(|e| e.to_string())?;
        *count += 1;

        if let Err(e) = git::auto_commit(&format!("[ADLER-SELFHEAL] Hata giderildi: {}", &error[..error.len().min(80)])) {
            log::warn!("Git commit basarisiz: {}", e);
        }

        Ok(fix)
    }

    pub fn fix_count(&self) -> u32 {
        self.fix_count.lock().map(|c| *c).unwrap_or(0)
    }
}
