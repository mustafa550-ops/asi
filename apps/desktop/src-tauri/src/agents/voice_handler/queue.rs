use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone)]
pub struct AudioItem {
    pub text: String,
    pub priority: Priority,
    pub tts_params: TtsParams,
}

#[derive(Debug, Clone)]
pub struct TtsParams {
    pub speed: f32,
    pub stability: f32,
    pub similarity_boost: f32,
    pub language: String,
}

impl Default for TtsParams {
    fn default() -> Self {
        TtsParams {
            speed: 1.0,
            stability: 0.3,
            similarity_boost: 0.7,
            language: "tr".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceProfile {
    Hizli,
    Sakin,
    Teknik,
    Varsayilan,
}

impl VoiceProfile {
    pub fn to_tts_params(&self) -> TtsParams {
        match self {
            VoiceProfile::Hizli => TtsParams {
                speed: 1.5,
                stability: 0.2,
                similarity_boost: 0.5,
                language: "tr".into(),
            },
            VoiceProfile::Sakin => TtsParams {
                speed: 0.7,
                stability: 0.7,
                similarity_boost: 0.9,
                language: "tr".into(),
            },
            VoiceProfile::Teknik => TtsParams {
                speed: 0.9,
                stability: 0.5,
                similarity_boost: 0.8,
                language: "tr".into(),
            },
            VoiceProfile::Varsayilan => TtsParams::default(),
        }
    }
}

pub fn detect_language(text: &str) -> String {
    let clean = text.to_lowercase();
    let tr_chars = ['ğ', 'ü', 'ş', 'ı', 'ö', 'ç', 'Ğ', 'Ü', 'Ş', 'İ', 'Ö', 'Ç'];
    let tr_count = clean.chars().filter(|c| tr_chars.contains(c)).count();
    let tr_words = ["bir", "bu", "ve", "ile", "için", "ama", "veya", "gibi", "kadar", "sonra",
                    "önce", "üzerinde", "arasında", "altında", "yaklaşık", "hem", "ya da",
                    "çünkü", "ancak", "fakat", "veya", "değil", "çok", "az", "büyük"];
    let en_words = ["the", "and", "for", "are", "but", "not", "you", "all", "can", "had",
                    "her", "was", "one", "our", "out", "has", "have", "been", "some", "them",
                    "than", "what", "when", "which", "will", "would", "about", "into"];
    let words: Vec<&str> = clean.split_whitespace().collect();
    let tr_word_count = words.iter().filter(|w| tr_words.contains(w)).count();
    let en_word_count = words.iter().filter(|w| en_words.contains(w)).count();

    if tr_count >= 2 || tr_word_count > en_word_count {
        "tr".into()
    } else if en_word_count > tr_word_count {
        "en".into()
    } else {
        "tr".into()
    }
}

pub struct AudioQueue {
    items: Mutex<VecDeque<AudioItem>>,
    playing: AtomicBool,
    interrupt_requested: AtomicBool,
}

impl AudioQueue {
    pub fn new() -> Self {
        AudioQueue {
            items: Mutex::new(VecDeque::new()),
            playing: AtomicBool::new(false),
            interrupt_requested: AtomicBool::new(false),
        }
    }

    fn insert_item(&self, item: AudioItem) {
        let is_high = item.priority == Priority::High && self.playing.load(Ordering::Relaxed);
        if is_high {
            self.interrupt_requested.store(true, Ordering::Relaxed);
        }
        let mut queue = self.items.lock().unwrap();
        if is_high {
            queue.push_front(item);
        } else {
            let insert_idx = queue.iter().position(|i| i.priority > item.priority)
                .unwrap_or(queue.len());
            queue.insert(insert_idx, item);
        }
    }

    pub fn enqueue(&self, text: String, priority: Priority) {
        self.insert_item(AudioItem {
            text,
            priority,
            tts_params: TtsParams::default(),
        });
    }

    pub fn enqueue_with_params(&self, text: String, priority: Priority, params: TtsParams) {
        self.insert_item(AudioItem { text, priority, tts_params: params });
    }

    pub fn dequeue(&self) -> Option<AudioItem> {
        let mut queue = self.items.lock().unwrap();
        queue.pop_front()
    }

    pub fn peek(&self) -> Option<AudioItem> {
        let queue = self.items.lock().unwrap();
        queue.front().cloned()
    }

    pub fn len(&self) -> usize {
        let queue = self.items.lock().unwrap();
        queue.len()
    }

    pub fn is_empty(&self) -> bool {
        let queue = self.items.lock().unwrap();
        queue.is_empty()
    }

    pub fn clear(&self) {
        let mut queue = self.items.lock().unwrap();
        queue.clear();
    }

    pub fn start_playing(&self) {
        self.playing.store(true, Ordering::Relaxed);
    }

    pub fn stop_playing(&self) {
        self.playing.store(false, Ordering::Relaxed);
    }

    pub fn is_playing(&self) -> bool {
        self.playing.load(Ordering::Relaxed)
    }

    pub fn request_interrupt(&self) {
        self.interrupt_requested.store(true, Ordering::Relaxed);
    }

    pub fn check_interrupt(&self) -> bool {
        self.interrupt_requested.swap(false, Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_queue_is_empty() {
        let q = AudioQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn enqueue_dequeue_single() {
        let q = AudioQueue::new();
        q.enqueue("test".into(), Priority::Normal);
        assert_eq!(q.len(), 1);
        let item = q.dequeue().unwrap();
        assert_eq!(item.text, "test");
        assert_eq!(item.priority, Priority::Normal);
    }

    #[test]
    fn priority_ordering() {
        let q = AudioQueue::new();
        q.enqueue("low".into(), Priority::Low);
        q.enqueue("high".into(), Priority::High);
        q.enqueue("normal".into(), Priority::Normal);

        assert_eq!(q.dequeue().unwrap().text, "high");
        assert_eq!(q.dequeue().unwrap().text, "normal");
        assert_eq!(q.dequeue().unwrap().text, "low");
    }

    #[test]
    fn high_priority_triggers_interrupt() {
        let q = AudioQueue::new();
        q.start_playing();
        q.enqueue("normal".into(), Priority::Normal);

        assert!(!q.interrupt_requested.load(Ordering::Relaxed));
        q.enqueue("high".into(), Priority::High);
        assert!(q.interrupt_requested.load(Ordering::Relaxed));
    }

    #[test]
    fn interrupt_flag_consumed_on_check() {
        let q = AudioQueue::new();
        q.request_interrupt();
        assert!(q.check_interrupt());
        assert!(!q.check_interrupt());
    }

    #[test]
    fn clear_empties_queue() {
        let q = AudioQueue::new();
        q.enqueue("a".into(), Priority::Normal);
        q.enqueue("b".into(), Priority::Normal);
        q.clear();
        assert!(q.is_empty());
    }

    #[test]
    fn peek_does_not_remove() {
        let q = AudioQueue::new();
        q.enqueue("peek".into(), Priority::Normal);
        let item = q.peek().unwrap();
        assert_eq!(item.text, "peek");
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn playing_state() {
        let q = AudioQueue::new();
        assert!(!q.is_playing());
        q.start_playing();
        assert!(q.is_playing());
        q.stop_playing();
        assert!(!q.is_playing());
    }

    #[test]
    fn enqueue_with_params() {
        let q = AudioQueue::new();
        let params = TtsParams {
            speed: 1.5,
            stability: 0.5,
            similarity_boost: 0.9,
            language: "en".into(),
        };
        q.enqueue_with_params("custom".into(), Priority::Low, params);
        let item = q.dequeue().unwrap();
        assert_eq!(item.tts_params.speed, 1.5);
        assert_eq!(item.tts_params.language, "en");
    }

    #[test]
    fn voice_profile_hizli_increases_speed() {
        let params = VoiceProfile::Hizli.to_tts_params();
        assert_eq!(params.speed, 1.5);
    }

    #[test]
    fn voice_profile_sakin_decreases_speed() {
        let params = VoiceProfile::Sakin.to_tts_params();
        assert_eq!(params.speed, 0.7);
    }

    #[test]
    fn detect_language_turkish() {
        let result = detect_language("Merhaba, bugün hava çok güzel");
        assert_eq!(result, "tr");
    }

    #[test]
    fn detect_language_english() {
        let result = detect_language("Hello, the weather is very nice today");
        assert_eq!(result, "en");
    }

    #[test]
    fn detect_language_short_text_defaults_to_tr() {
        let result = detect_language("xyz");
        assert_eq!(result, "tr");
    }
}
