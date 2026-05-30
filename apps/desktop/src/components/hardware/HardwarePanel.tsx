import { useEffect, useState } from "react";
import { invoke } from "../../lib/tauri";

interface DiscoveredDevice {
  name: string;
  bus_type: string;
  address: string;
  description: string;
}

interface HardwareConfig {
  enabled: boolean;
  gpio_pins: Record<string, number>;
  sensor_types: string[];
  relay_pulse_ms: number;
  safety_max_temp: number;
  watchdog_timeout_s: number;
}

export default function HardwarePanel() {
  const [devices, setDevices] = useState<DiscoveredDevice[]>([]);
  const [config, setConfig] = useState<HardwareConfig | null>(null);
  const [sensorData, setSensorData] = useState<string>("--");
  const [gpioPin, setGpioPin] = useState(17);
  const [gpioValue, setGpioValue] = useState<string>("?");
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const scanHardware = async () => {
    setScanning(true);
    setError(null);
    try {
      const raw = await invoke<string>("hw_detect");
      setDevices(JSON.parse(raw));
    } catch (e) {
      setError(String(e));
    }
    setScanning(false);
  };

  const readSensors = async () => {
    try {
      const result = await invoke<string>("sensor_read");
      setSensorData(result);
    } catch {
      setSensorData("Sensor okunamadi");
    }
  };

  const readGpio = async () => {
    try {
      const result = await invoke<string>("gpio_read", { pin: gpioPin });
      setGpioValue(result);
    } catch {
      setGpioValue("HATA");
    }
  };

  const writeGpio = async (value: boolean) => {
    try {
      await invoke<string>("gpio_write", { pin: gpioPin, value });
      readGpio();
    } catch (e) {
      setError(String(e));
    }
  };

  const toggleRelay = async (pin: number, on: boolean) => {
    try {
      await invoke<string>("relay_set", { pin, on });
    } catch (e) {
      setError(String(e));
    }
  };

  useEffect(() => {
    invoke<string>("hw_config_get")
      .then((raw) => setConfig(JSON.parse(raw)))
      .catch(() => {});
    scanHardware();
    readSensors();
  }, []);

  const busIcons: Record<string, string> = {
    onewire: "🌡",
    gpio: "⚡",
    i2c: "🔌",
    thermal: "🔥",
  };

  return (
    <div className="hardware-panel">
      <h3>Donanim Paneli</h3>

      <div className="hardware-actions">
        <button className="btn btn-secondary" onClick={scanHardware} disabled={scanning}>
          {scanning ? "Taranıyor..." : "Donanim Tara"}
        </button>
        <button className="btn btn-secondary" onClick={readSensors}>
          Sensor Oku
        </button>
      </div>

      {error && <div className="hardware-error">{error}</div>}

      <div className="hardware-section">
        <h4>Sensor</h4>
        <pre className="hardware-sensor-data">{sensorData}</pre>
      </div>

      <div className="hardware-section">
        <h4>GPIO</h4>
        <div className="hardware-gpio-row">
          <label>Pin:</label>
          <input
            type="number"
            min={0}
            max={255}
            value={gpioPin}
            onChange={(e) => setGpioPin(Number(e.target.value))}
          />
          <span className="hardware-gpio-value">Deger: {gpioValue}</span>
          <button className="btn btn-sm" onClick={() => readGpio()}>Oku</button>
          <button className="btn btn-sm btn-success" onClick={() => writeGpio(true)}>HIGH</button>
          <button className="btn btn-sm btn-danger" onClick={() => writeGpio(false)}>LOW</button>
        </div>
      </div>

      <div className="hardware-section">
        <h4>Role</h4>
        <div className="hardware-relay-row">
          {config?.gpio_pins && Object.entries(config.gpio_pins).map(([name, pin]) => (
            <div key={name} className="hardware-relay-item">
              <span>{name} (pin {pin})</span>
              <button className="btn btn-sm btn-success" onClick={() => toggleRelay(pin, true)}>AC</button>
              <button className="btn btn-sm btn-danger" onClick={() => toggleRelay(pin, false)}>KAPA</button>
            </div>
          ))}
        </div>
      </div>

      <div className="hardware-section">
        <h4>Kesfedilen Cihazlar ({devices.length})</h4>
        <div className="hardware-device-list">
          {devices.map((d, i) => (
            <div key={i} className="hardware-device-item">
              <span className="hardware-device-icon">{busIcons[d.bus_type] || "❓"}</span>
              <span className="hardware-device-name">{d.name}</span>
              <span className="hardware-device-desc">{d.description}</span>
            </div>
          ))}
          {devices.length === 0 && <span className="hardware-empty">Cihaz bulunamadi</span>}
        </div>
      </div>
    </div>
  );
}
