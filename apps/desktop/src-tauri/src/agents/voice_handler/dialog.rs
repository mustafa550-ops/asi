use super::queue::{AudioQueue, Priority, TtsParams, VoiceProfile, detect_language};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogState {
    Idle,
    Listening,
    Processing,
    Speaking,
    Waiting,
}

pub struct DialogManager {
    state: AtomicDialogState,
    queue: Arc<AudioQueue>,
    barge_in: AtomicBool,
    profile: std::sync::Mutex<VoiceProfile>,
    _auto_stop_ms: u64,
    _idle_timeout_ms: u64,
}

struct AtomicDialogState {
    inner: std::sync::Mutex<DialogState>,
}

impl AtomicDialogState {
    fn new(state: DialogState) -> Self {
        AtomicDialogState {
            inner: std::sync::Mutex::new(state),
        }
    }

    fn get(&self) -> DialogState {
        *self.inner.lock().unwrap()
    }

    fn set(&self, state: DialogState) {
        *self.inner.lock().unwrap() = state;
    }
}

impl DialogManager {
    pub fn new(queue: Arc<AudioQueue>) -> Self {
        DialogManager {
            state: AtomicDialogState::new(DialogState::Idle),
            queue,
            barge_in: AtomicBool::new(true),
            profile: std::sync::Mutex::new(VoiceProfile::Varsayilan),
            _auto_stop_ms: 2000,
            _idle_timeout_ms: 30000,
        }
    }

    pub fn state(&self) -> DialogState {
        self.state.get()
    }

    pub fn is_barge_in(&self) -> bool {
        self.barge_in.load(Ordering::Relaxed)
    }

    pub fn set_barge_in(&self, enabled: bool) {
        self.barge_in.store(enabled, Ordering::Relaxed);
    }

    pub fn profile(&self) -> VoiceProfile {
        *self.profile.lock().unwrap()
    }

    pub fn set_profile(&self, profile: VoiceProfile) {
        *self.profile.lock().unwrap() = profile;
    }

    pub fn on_wake_word(&self) {
        self.state.set(DialogState::Listening);
        log::info!("Diyalog: Listening → wake word algilandi");
    }

    pub fn on_speech_received(&self) {
        self.state.set(DialogState::Processing);
        log::info!("Diyalog: Processing → konusma alindi");
    }

    pub fn on_response_ready(&self, text: &str) {
        let should_interrupt = self.is_barge_in() && self.state.get() == DialogState::Speaking;
        if should_interrupt {
            self.queue.request_interrupt();
        }

        let lang = detect_language(text);
        let mut params = self.profile.lock().unwrap().to_tts_params();
        params.language = lang;
        self.queue.enqueue_with_params(text.into(), Priority::Normal, params);
        self.state.set(DialogState::Speaking);
        log::info!("Diyalog: Speaking → yanit kuyruga eklendi");
    }

    pub fn on_response_complete(&self) {
        if self.is_barge_in() {
            self.state.set(DialogState::Listening);
            log::info!("Diyalog: Speaking → Listening (barge-in acik)");
        } else {
            self.state.set(DialogState::Waiting);
            log::info!("Diyalog: Speaking → Waiting (barge-in kapali)");
        }
    }

    pub fn on_idle_timeout(&self) {
        self.state.set(DialogState::Idle);
        self.queue.clear();
        log::info!("Diyalog: Waiting → Idle (zaman asimi)");
    }

    pub fn speak_high_priority(&self, text: &str) {
        self.queue.enqueue(text.into(), Priority::High);
        if self.state.get() == DialogState::Idle {
            self.state.set(DialogState::Speaking);
        }
    }

    pub fn speak_with_params(&self, text: &str, params: TtsParams) {
        self.queue.enqueue_with_params(text.into(), Priority::Normal, params);
    }

    pub fn reset(&self) {
        self.state.set(DialogState::Idle);
        self.queue.clear();
    }
}

impl Default for DialogManager {
    fn default() -> Self {
        DialogManager::new(Arc::new(AudioQueue::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_dialog_starts_idle() {
        let dm = DialogManager::default();
        assert_eq!(dm.state(), DialogState::Idle);
    }

    #[test]
    fn wake_word_sets_listening() {
        let dm = DialogManager::default();
        dm.on_wake_word();
        assert_eq!(dm.state(), DialogState::Listening);
    }

    #[test]
    fn speech_received_sets_processing() {
        let dm = DialogManager::default();
        dm.on_wake_word();
        dm.on_speech_received();
        assert_eq!(dm.state(), DialogState::Processing);
    }

    #[test]
    fn response_ready_sets_speaking() {
        let dm = DialogManager::default();
        dm.on_wake_word();
        dm.on_speech_received();
        dm.on_response_ready("merhaba");
        assert_eq!(dm.state(), DialogState::Speaking);
    }

    #[test]
    fn response_complete_with_barge_in() {
        let dm = DialogManager::default();
        dm.set_barge_in(true);
        dm.on_wake_word();
        dm.on_speech_received();
        dm.on_response_ready("test");
        dm.on_response_complete();
        assert_eq!(dm.state(), DialogState::Listening);
    }

    #[test]
    fn response_complete_without_barge_in() {
        let dm = DialogManager::default();
        dm.set_barge_in(false);
        dm.on_wake_word();
        dm.on_speech_received();
        dm.on_response_ready("test");
        dm.on_response_complete();
        assert_eq!(dm.state(), DialogState::Waiting);
    }

    #[test]
    fn idle_timeout_returns_to_idle() {
        let dm = DialogManager::default();
        dm.on_wake_word();
        dm.on_idle_timeout();
        assert_eq!(dm.state(), DialogState::Idle);
    }

    #[test]
    fn high_priority_speech_works() {
        let dm = DialogManager::default();
        dm.speak_high_priority("uyari");
        assert!(!dm.queue.is_empty());
    }

    #[test]
    fn reset_clears_state_and_queue() {
        let dm = DialogManager::default();
        dm.on_wake_word();
        dm.speak_high_priority("test");
        dm.reset();
        assert_eq!(dm.state(), DialogState::Idle);
        assert!(dm.queue.is_empty());
    }

    #[test]
    fn barge_in_toggle() {
        let dm = DialogManager::default();
        assert!(dm.is_barge_in());
        dm.set_barge_in(false);
        assert!(!dm.is_barge_in());
    }

    #[test]
    fn profile_defaults_to_varsayilan() {
        let dm = DialogManager::default();
        assert_eq!(dm.profile(), VoiceProfile::Varsayilan);
    }

    #[test]
    fn set_profile_changes_profile() {
        let dm = DialogManager::default();
        dm.set_profile(VoiceProfile::Hizli);
        assert_eq!(dm.profile(), VoiceProfile::Hizli);
    }

    #[test]
    fn response_ready_auto_detects_language() {
        let dm = DialogManager::default();
        dm.on_wake_word();
        dm.on_speech_received();
        dm.on_response_ready("Hello, how are you?");
        assert_eq!(dm.state(), DialogState::Speaking);
    }
}
