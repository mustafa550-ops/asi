pub struct FrontendValidator;

impl FrontendValidator {
    pub fn detect_mocks(content: &str) -> Result<(), String> {
        let forbidden_patterns = [
            "mockData", "dummyData", "fakeApi", "placeholder", 
            "// TODO: replace with real api", "useState([{"
        ];

        for pattern in forbidden_patterns.iter() {
            if content.contains(pattern) {
                return Err(format!("Zero-Mock Policy Violation: Found forbidden pattern '{}'. You must implement the real backend endpoint first.", pattern));
            }
        }
        Ok(())
    }
}
