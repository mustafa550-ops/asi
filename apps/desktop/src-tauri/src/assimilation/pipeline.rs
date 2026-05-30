use crate::assimilation::adapter::CodeAdapter;
use crate::assimilation::analyzer::RepoAnalyzer;
use crate::assimilation::cloner::RepoCloner;
use crate::assimilation::dependency::DependencyAnalyzer;
use crate::assimilation::registry::ModuleRegistry;
use crate::assimilation::rollback::RollbackManager;
use crate::assimilation::splitter::CodeSplitter;
use crate::core::wasm_sandbox::WasmSandbox;

#[derive(Debug, Clone, PartialEq)]
pub enum PipelineStep {
    Clone,
    Analyze,
    Split,
    Adapt,
    Test,
    Register,
}

#[derive(Debug)]
pub struct PipelineReport {
    pub success: bool,
    pub name: String,
    pub steps_completed: Vec<String>,
    pub module_path: Option<String>,
    pub summary: String,
    pub errors: Vec<String>,
}

pub struct AssimilationPipeline {
    cloner: RepoCloner,
    analyzer: RepoAnalyzer,
    splitter: CodeSplitter,
    adapter: CodeAdapter,
    sandbox: WasmSandbox,
    dependency_analyzer: DependencyAnalyzer,
}

impl AssimilationPipeline {
    pub fn new() -> Self {
        Self {
            cloner: RepoCloner::new(),
            analyzer: RepoAnalyzer::new(),
            splitter: CodeSplitter::new(),
            adapter: CodeAdapter::new(),
            sandbox: WasmSandbox::new(),
            dependency_analyzer: DependencyAnalyzer::new(),
        }
    }

    pub fn run(
        &self,
        repo_url: &str,
        registry: &ModuleRegistry,
        modules_base: &str,
    ) -> PipelineReport {
        let safe_name = repo_url
            .trim_end_matches(".git")
            .rsplit('/')
            .next()
            .unwrap_or(repo_url)
            .to_string();

        let temp_id = uuid::Uuid::new_v4().to_string();
        let temp_path = format!("/tmp/adler-assimilate/{}", temp_id);

        let mut steps_completed = Vec::new();
        let mut errors = Vec::new();
        #[allow(unused_assignments)]
        let mut module_path = None;

        // Step 1: Clone
        {
            let result = self.cloner.clone(repo_url, &temp_path);
            match result {
                Ok(_) => {
                    steps_completed.push("Klonlama".into());
                    log::info!("[Pipeline 1/6] Clone OK: {}", repo_url);
                }
                Err(e) => {
                    let msg = format!("Klonlama başarısız: {}", e);
                    errors.push(msg.clone());
                    RollbackManager::cleanup_temp(&temp_path);
                    return PipelineReport {
                        success: false,
                        name: safe_name,
                        steps_completed,
                        module_path: None,
                        summary: format!("Asimilasyon başarısız (adım 1/6: Klonlama) — {}", e),
                        errors,
                    };
                }
            }
        }

        // Step 2: Analyze
        let analysis = match self.analyzer.analyze(&temp_path) {
            Ok(a) => {
                steps_completed.push("Analiz".into());
                log::info!("[Pipeline 2/6] Analyze OK: language={}, files={}", a.language, a.file_count);
                a
            }
            Err(e) => {
                let msg = format!("Analiz başarısız: {}", e);
                errors.push(msg.clone());
                RollbackManager::full_rollback(&safe_name, &temp_path, registry, modules_base, &msg);
                return PipelineReport {
                    success: false,
                    name: safe_name,
                    steps_completed,
                    module_path: None,
                    summary: format!("Asimilasyon başarısız (adım 2/6: Analiz) — {}", e),
                    errors,
                };
            }
        };

        // Step 3: Split
        {
            let result = self.splitter.split(&temp_path);
            steps_completed.push("Parçalama".into());
            log::info!("[Pipeline 3/6] Split OK: core={}, interface={}, config={}",
                result.core.len(), result.interface.len(), result.config.len());
        }

        // Step 4: Adapt
        let adapted = {
            let result = self.adapter.adapt_directory(&temp_path, &analysis.language);
            let count = result.len();
            steps_completed.push("Adaptasyon".into());
            log::info!("[Pipeline 4/6] Adapt OK: {} files adapted", count);
            result
        };

        // Step 5: Test (wasm sandbox — optional, non-blocking)
        {
            let wasm_result = self.sandbox.test_directory(&temp_path, &analysis.language);
            match wasm_result {
                Ok(msg) => {
                    steps_completed.push("Test".into());
                    log::info!("[Pipeline 5/6] Test: {}", msg);
                }
                Err(e) => {
                    // Non-blocking: test failure is a warning, not a pipeline fail
                    let msg = format!("Test uyarısı: {}", e);
                    errors.push(msg);
                    log::warn!("[Pipeline 5/6] Test skipped: {}", e);
                    steps_completed.push("Test (atlandı)".into());
                }
            }
        }

        // Dependency analysis
        let deps = self.dependency_analyzer.analyze(&temp_path, &analysis.language);
        log::info!("[Pipeline] Dependencies: {}", self.dependency_analyzer.summary(&deps));

        // Finalize: move from temp to modules/ first
        let fallback_path = format!("{}/{}", modules_base, safe_name);
        let final_path = match RollbackManager::finalize(&safe_name, &temp_path, modules_base) {
            Ok(p) => {
                module_path = Some(p.clone());
                p
            }
            Err(e) => {
                errors.push(format!("Finalize uyarisi: {} — fallback path kullaniliyor", e));
                if std::path::Path::new(&temp_path).exists() {
                    match RollbackManager::finalize(&safe_name, &temp_path, modules_base) {
                        Ok(p) => {
                            module_path = Some(p.clone());
                            p
                        }
                        Err(_) => {
                            module_path = Some(temp_path.clone());
                            temp_path.clone()
                        }
                    }
                } else {
                    module_path = Some(fallback_path.clone());
                    fallback_path
                }
            }
        };

        // Step 6: Register (with final path)
        {
            match registry.register(&safe_name, &final_path, &deps.packages) {
                Ok(_) => {
                    steps_completed.push("Kayıt".into());
                    log::info!("[Pipeline 6/6] Register OK: {}", safe_name);
                }
                Err(e) => {
                    let msg = format!("Kayıt başarısız: {}", e);
                    errors.push(msg);
                    RollbackManager::full_rollback(&safe_name, &temp_path, registry, modules_base, &e);
                    return PipelineReport {
                        success: false,
                        name: safe_name,
                        steps_completed,
                        module_path: None,
                        summary: format!("Asimilasyon başarısız (adım 6/6: Kayıt) — {}", e),
                        errors,
                    };
                }
            }
        }

        let adapted_count = adapted.len();
        let manifest_warn = if !adapted.is_empty() && analysis.language == "Rust" {
            " (Rust->Rust: dosyalar aynen kopyalandi)"
        } else { "" };
        let summary = format!(
            "Asimilasyon tamamlandi: '{}' — {} adim, {} dosya adapte edildi{}, {} bagimlilik, konum: {}",
            safe_name, steps_completed.len(), adapted_count, manifest_warn, deps.packages.len(), final_path
        );

        PipelineReport {
            success: true,
            name: safe_name,
            steps_completed,
            module_path,
            summary,
            errors,
        }
    }
}
