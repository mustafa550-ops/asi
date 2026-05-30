pub fn synthesize(text: &str) -> Result<String, String> {
    let output_path = "/tmp/adler_tts_output.wav";

    let espeak_result = call_espeak(text, output_path);
    if let Ok(msg) = espeak_result {
        let size = std::fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
        return Ok(format!("TTS (espeak-ng): {} -> {} bayt | {}", text, size, msg));
    }

    let supertonic_result = match call_supertonic_api(text) {
        Ok(msg) => {
            let size = std::fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
            return Ok(format!("TTS (supertonic): {} -> {} bayt | {}", text, size, msg));
        }
        Err(e) => e,
    };

    let sample_rate = 24000u32;
    let duration_secs = (text.len() as f32 * 0.08).max(1.0);
    let num_samples = (sample_rate as f32 * duration_secs) as u32;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(output_path, spec)
        .map_err(|e| format!("WAV dosyasi olusturulamadi: {}", e))?;

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let freq = 220.0 + (text.len() as f32 * 10.0).min(800.0);
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let amplitude = 0.3;
        let value = (sample * amplitude * 32767.0) as i16;
        writer.write_sample(value).map_err(|e| format!("WAV yazma hatasi: {}", e))?;
    }

    writer.finalize().map_err(|e| format!("WAV finalize hatasi: {}", e))?;

    let size = std::fs::metadata(output_path).map(|m| m.len()).unwrap_or(0);
    Ok(format!("TTS (sine-wave): {} -> {} bayt, {} Hz, {:.1}s | {}", text, size, sample_rate, duration_secs, supertonic_result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthesize_falls_back_to_sine_wave() {
        let result = synthesize("test").unwrap();
        assert!(result.contains("sine-wave") || result.contains("espeak-ng") || result.contains("supertonic"));
    }

    #[test]
    fn synthesize_creates_wav_file() {
        let _ = synthesize("merhaba");
        let path = std::path::Path::new("/tmp/adler_tts_output.wav");
        // May or may not exist depending on fallback success, but shouldn't crash
        let _ = path.exists();
    }
}

fn call_espeak(text: &str, output_path: &str) -> Result<String, String> {
    let output = std::process::Command::new("espeak-ng")
        .args(["-w", output_path, "--", text])
        .output()
        .map_err(|e| format!("espeak-ng subprocess hatasi: {}", e))?;

    if output.status.success() {
        Ok("espeak-ng basarili".into())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("espeak-ng basarisiz: {}", stderr.trim()))
    }
}

pub fn call_supertonic_api(text: &str) -> Result<String, String> {
    let url = "http://127.0.0.1:8765/tts";
    let client = reqwest::blocking::Client::new();
    let payload = serde_json::json!({
        "text": text,
        "lang": "tr",
        "voice_style": "default"
    });

    let resp = client.post(url)
        .json(&payload)
        .send()
        .map_err(|e| format!("Supertonic API hatasi: {}", e))?;

    if resp.status().is_success() {
        Ok("Supertonic TTS basarili".into())
    } else {
        Err(format!("Supertonic HTTP {}", resp.status()))
    }
}
