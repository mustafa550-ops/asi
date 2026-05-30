pub struct PromptTemplates;

impl PromptTemplates {
    pub fn intent_classification() -> &'static str {
        "Kullanıcı mesajını tek bir kategoriye sınıflandır.\n\
         Kategoriler: sorgu, eylem, analiz, donanım, kripto, sistem, doküman, ses\n\
         Sadece kategori adını yaz, açıklama ekleme.\n\
         Mesaj: {}"
    }

    pub fn entity_extraction() -> &'static str {
        "Verilen metinden coin sembolü, fiyat, miktar ve GPIO pin numarası gibi \
         varlıkları JSON formatında çıkar.\n\
         Format: {\"entities\": [{\"type\": \"...\", \"value\": \"...\"}]}\n\
         Metin: {}"
    }

    pub fn context_resolution() -> &'static str {
        "Aşağıdaki konuşma bağlamına göre kullanıcının son mesajındaki \
         zamirlerin (bunu, şunu, onu) neyi işaret ettiğini belirle.\n\
         Bağlam: {}\n\
         Son mesaj: {}"
    }

    pub fn sentiment_analysis() -> &'static str {
        "Kullanıcı mesajının duygu durumunu belirle: urgent, neutral, casual\n\
         Sadece bir kelime yaz.\n\
         Mesaj: {}"
    }

    pub fn few_shot_examples() -> Vec<(&'static str, &'static str)> {
        vec![
            ("SXT'yi kontrol et", "kripto"),
            ("Röleyi aç", "donanim"),
            ("Sistem durumunu raporla", "sistem"),
            ("Notlarımı incele", "dokuman"),
            ("Hey Adler, merhaba", "chat"),
            ("Bugün hava nasıl?", "sorgu"),
        ]
    }
}
