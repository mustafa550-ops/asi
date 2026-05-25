use super::gpio;

pub fn set(pin: u8, state: bool) -> Result<(), String> {
    gpio::write(pin, state)
}
