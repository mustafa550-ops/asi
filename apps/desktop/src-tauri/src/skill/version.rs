use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Semver {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Semver {
    pub fn parse(version: &str) -> Result<Self, String> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err(format!("Gecersiz semver: '{}' (major.minor.patch bekleniyor)", version));
        }
        let major = parts[0].parse::<u32>()
            .map_err(|_| format!("Gecersiz major: '{}'", parts[0]))?;
        let minor = parts[1].parse::<u32>()
            .map_err(|_| format!("Gecersiz minor: '{}'", parts[1]))?;
        let patch = parts[2].parse::<u32>()
            .map_err(|_| format!("Gecersiz patch: '{}'", parts[2]))?;
        Ok(Semver { major, minor, patch })
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

    pub fn bump_major(&self) -> Self {
        Semver { major: self.major + 1, minor: 0, patch: 0 }
    }

    pub fn bump_minor(&self) -> Self {
        Semver { major: self.major, minor: self.minor + 1, patch: 0 }
    }

    pub fn bump_patch(&self) -> Self {
        Semver { major: self.major, minor: self.minor, patch: self.patch + 1 }
    }

    pub fn is_compatible(&self, other: &Semver) -> bool {
        self.major == other.major && self.major > 0
    }

    pub fn is_breaking(&self, other: &Semver) -> bool {
        self.major != other.major
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionRecord {
    pub skill_name: String,
    pub version: Semver,
    pub changelog: String,
    pub created_at: String,
}

pub struct VersionManager {
    pub history: Vec<VersionRecord>,
}

impl VersionManager {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn bump_version(current: &str, bump_type: &str) -> Result<String, String> {
        let semver = Semver::parse(current)?;
        let new = match bump_type {
            "major" => semver.bump_major(),
            "minor" => semver.bump_minor(),
            "patch" => semver.bump_patch(),
            _ => return Err(format!("Gecersiz bump turu: '{}' (major/minor/patch)", bump_type)),
        };
        Ok(new.to_string())
    }

    pub fn add_version(&mut self, skill_name: &str, version: &str, changelog: &str) -> Result<Semver, String> {
        let semver = Semver::parse(version)?;
        self.history.push(VersionRecord {
            skill_name: skill_name.to_string(),
            version: semver.clone(),
            changelog: changelog.to_string(),
            created_at: format!("{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs()),
        });
        Ok(semver)
    }

    pub fn rollback(&self, target_version: &str) -> Result<VersionRecord, String> {
        let target = Semver::parse(target_version)?;
        self.history.iter()
            .rev()
            .find(|r| r.version == target)
            .cloned()
            .ok_or_else(|| format!("Versiyon '{}' gecmiste bulunamadi", target_version))
    }

    pub fn latest_version(&self, skill_name: &str) -> Option<&VersionRecord> {
        self.history.iter()
            .filter(|r| r.skill_name == skill_name)
            .last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semver_parse_valid() {
        let v = Semver::parse("1.2.3").unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);
    }

    #[test]
    fn semver_parse_invalid_format() {
        assert!(Semver::parse("1.2").is_err());
        assert!(Semver::parse("abc").is_err());
        assert!(Semver::parse("").is_err());
    }

    #[test]
    fn semver_to_string() {
        let v = Semver::parse("2.0.1").unwrap();
        assert_eq!(v.to_string(), "2.0.1");
    }

    #[test]
    fn bump_major() {
        let v = Semver::parse("1.2.3").unwrap();
        let bumped = v.bump_major();
        assert_eq!(bumped.to_string(), "2.0.0");
    }

    #[test]
    fn bump_minor() {
        let v = Semver::parse("1.2.3").unwrap();
        let bumped = v.bump_minor();
        assert_eq!(bumped.to_string(), "1.3.0");
    }

    #[test]
    fn bump_patch() {
        let v = Semver::parse("1.2.3").unwrap();
        let bumped = v.bump_patch();
        assert_eq!(bumped.to_string(), "1.2.4");
    }

    #[test]
    fn is_compatible_same_major() {
        let a = Semver::parse("1.0.0").unwrap();
        let b = Semver::parse("1.5.0").unwrap();
        assert!(a.is_compatible(&b));
    }

    #[test]
    fn is_breaking_different_major() {
        let a = Semver::parse("1.0.0").unwrap();
        let b = Semver::parse("2.0.0").unwrap();
        assert!(a.is_breaking(&b));
    }

    #[test]
    fn bump_version_valid() {
        let r = VersionManager::bump_version("1.0.0", "patch").unwrap();
        assert_eq!(r, "1.0.1");
    }

    #[test]
    fn bump_version_invalid_tur() {
        assert!(VersionManager::bump_version("1.0.0", "invalid").is_err());
    }

    #[test]
    fn add_and_latest_version() {
        let mut vm = VersionManager::new();
        vm.add_version("test_skill", "1.0.0", "Ilk surum").unwrap();
        vm.add_version("test_skill", "1.1.0", "Yeni ozellik").unwrap();
        let latest = vm.latest_version("test_skill").unwrap();
        assert_eq!(latest.version.to_string(), "1.1.0");
    }

    #[test]
    fn rollback_to_version() {
        let mut vm = VersionManager::new();
        vm.add_version("test_skill", "1.0.0", "Ilk surum").unwrap();
        vm.add_version("test_skill", "2.0.0", "Breaking degisiklik").unwrap();
        let rolled = vm.rollback("1.0.0").unwrap();
        assert_eq!(rolled.version.to_string(), "1.0.0");
    }

    #[test]
    fn rollback_nonexistent() {
        let vm = VersionManager::new();
        assert!(vm.rollback("9.9.9").is_err());
    }

    #[test]
    fn empty_history_no_latest() {
        let vm = VersionManager::new();
        assert!(vm.latest_version("test").is_none());
    }
}
