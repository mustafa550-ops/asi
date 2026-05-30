pub struct IntentRepair;

impl IntentRepair {
    pub fn handle_unknown(&self, text: &str) -> String {
        let lower = text.to_lowercase();

        if lower.contains("ne") || lower.contains("nedir") || lower.contains("what") {
            return format!("'{}' hakkında bilgi mi almak istediniz?", text);
        }
        if lower.contains("yap") || lower.contains("çalıştır") || lower.contains("run") {
            return format!("'{}' komutunu mu çalıştırmak istediniz?", text);
        }
        if lower.contains("kontrol") || lower.contains("check") || lower.contains("control") {
            return format!("'{}' kontrolünü mü yapmak istediniz?", text);
        }

        format!("Ne yapmamı istediğinizi tam anlayamadım. Aşağıdakilerden birini deneyebilirsiniz:\n\
                 - /analiz <coin> (kripto analizi)\n\
                 - /donanim (donanım kontrolü)\n\
                 - /sistem (sistem durumu)\n\
                 - /dokuman <konu> (doküman sorgulama)\n\
                 - /ses (sesli asistan)")
    }
}
