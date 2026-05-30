use std::path::Path;

pub struct RollbackManager;

impl RollbackManager {
    pub fn new() -> Self {
        Self
    }

    pub fn cleanup_temp(path: &str) {
        let p = Path::new(path);
        if p.exists() {
            if let Err(e) = std::fs::remove_dir_all(p) {
                log::warn!("Rollback: could not remove temp dir {}: {}", path, e);
            } else {
                log::info!("Rollback: temp dir {} cleaned", path);
            }
        }
    }

    pub fn remove_registered(name: &str, registry: &crate::assimilation::registry::ModuleRegistry) {
        if let Err(e) = registry.remove(name) {
            log::warn!("Rollback: could not remove registry entry '{}': {}", name, e);
        } else {
            log::info!("Rollback: registry entry '{}' removed", name);
        }
    }

    pub fn remove_modules_dir(name: &str, base_path: &str) {
        let dir = Path::new(base_path).join(name);
        if dir.exists() {
            if let Err(e) = std::fs::remove_dir_all(&dir) {
                log::warn!("Rollback: could not remove modules dir {:?}: {}", dir, e);
            } else {
                log::info!("Rollback: modules dir {:?} cleaned", dir);
            }
        }
    }

    pub fn full_rollback(
        name: &str,
        temp_path: &str,
        registry: &crate::assimilation::registry::ModuleRegistry,
        modules_base: &str,
        reason: &str,
    ) {
        log::error!("Assimilation rollback triggered for '{}': {}", name, reason);
        Self::cleanup_temp(temp_path);
        Self::remove_registered(name, registry);
        Self::remove_modules_dir(name, modules_base);
    }

    pub fn finalize(name: &str, temp_path: &str, modules_base: &str) -> Result<String, String> {
        let src = Path::new(temp_path);
        let dest = Path::new(modules_base).join(name);

        if dest.exists() {
            std::fs::remove_dir_all(&dest)
                .map_err(|e| format!("Cannot clean existing module dir: {}", e))?;
        }

        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Cannot create modules dir: {}", e))?;
        }

        // Try rename first (same fs), fall back to copy+remove (cross-device)
        if std::fs::rename(src, &dest).is_err() {
            log::info!("Fallback: copy directory (cross-device)");
            fn copy_dir(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
                std::fs::create_dir_all(dst)?;
                for entry in std::fs::read_dir(src)? {
                    let entry = entry?;
                    let file_type = entry.file_type()?;
                    let src_path = entry.path();
                    let dst_path = dst.join(entry.file_name());
                    if file_type.is_dir() {
                        copy_dir(&src_path, &dst_path)?;
                    } else {
                        std::fs::copy(&src_path, &dst_path)?;
                    }
                }
                Ok(())
            }
            copy_dir(src, &dest)
                .map_err(|e| format!("Cannot copy dir to modules: {}", e))?;
            std::fs::remove_dir_all(src)
                .map_err(|e| format!("Cannot clean temp dir after copy: {}", e))?;
            log::info!("Directory copied + temp cleaned");
        }

        log::info!("Assimilation finalized: {} -> {:?}", name, dest);
        Ok(dest.to_string_lossy().to_string())
    }
}
