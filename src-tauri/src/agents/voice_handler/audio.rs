use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct AudioBuffer {
    pub samples: Vec<i16>,
    pub sample_rate: u32,
}

static RECORDING: AtomicBool = AtomicBool::new(false);

pub fn record_loop() -> Result<cpal::Stream, String> {
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| "Mikrofon bulunamadi".to_string())?;

    let config = device.default_input_config()
        .map_err(|e| format!("Giris konfigurasyon hatasi: {}", e))?;

    let sample_rate = config.sample_rate().0;
    let channels = config.channels() as usize;

    log::info!("Mikrofon: {} @ {} Hz, {} kanal", device.name().unwrap_or("?".to_string()), sample_rate, channels);

    let err_fn = |err| log::error!("Ses akisi hatasi: {}", err);

    let _recording = Arc::new(AtomicBool::new(true));
    RECORDING.store(true, Ordering::Relaxed);

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            if !RECORDING.load(Ordering::Relaxed) {
                return;
            }
            let mut i16_buf: Vec<i16> = Vec::with_capacity(data.len());
            for &sample in data {
                let clamped = if sample > 1.0 { 1.0 } else if sample < -1.0 { -1.0 } else { sample };
                i16_buf.push((clamped * 32767.0) as i16);
            }
            super::wake_word::feed_audio(&i16_buf);
        },
        err_fn,
        None,
    ).map_err(|e| format!("Ses akisi baslatilamadi: {}", e))?;

    stream.play().map_err(|e| format!("Akis oynatilamadi: {}", e))?;
    Ok(stream)
}

pub fn stop_recording() {
    RECORDING.store(false, Ordering::Relaxed);
}

pub fn is_recording() -> bool {
    RECORDING.load(Ordering::Relaxed)
}
