use bmp085::sensors::{Barometer, Thermometer};
use bmp085::{BMP085BarometerThermometer, SamplingMode, BMP085_I2C_ADDR};
use i2cdev::linux::LinuxI2CDevice;
use lazy_static::lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref SENSOR: Mutex<BMP085BarometerThermometer<LinuxI2CDevice>> = {
        let i2c_dev = LinuxI2CDevice::new(&crate::args::ARGS.i2c_device, BMP085_I2C_ADDR).unwrap();
        Mutex::new(BMP085BarometerThermometer::new(i2c_dev, SamplingMode::UltraHighRes).unwrap())
    };
}

pub struct SensorOutput {
    pub temperature: f64, // celsius
    pub pressure: f64,    // kPa
}

pub fn sensor_read() -> SensorOutput {
    let mut sensor = SENSOR.lock().unwrap();

    let temperature = sensor.temperature_celsius().unwrap();
    let pressure = sensor.pressure_kpa().unwrap();

    let temperature = (temperature * 100.0).round() / 100.0;
    let pressure = (pressure * 1000.0).round() / 1000.0;

    SensorOutput {
        temperature: temperature.into(),
        pressure: pressure.into(),
    }
}
