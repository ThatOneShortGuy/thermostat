use bme280::i2c::BME280;
use linux_embedded_hal::{Delay, I2cdev};

use anyhow::Result;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the Pi's I²C bus
    let mut i2c_bus = I2cdev::new("/dev/i2c-1")?;

    // initialize the BME280 using the primary I2C address 0x76
    // let mut bme280 = BME280::new_primary(i2c_bus);

    // or, initialize the BME280 using the secondary I2C address 0x77
    let mut bme280 = BME280::new_secondary(i2c_bus);

    // or, initialize the BME280 using a custom I2C address
    // let bme280_i2c_addr = 0x88;
    // let mut bme280 = BME280::new(i2c_bus, bme280_i2c_addr, Delay);

    // initialize the sensor
    bme280.init(&mut Delay).unwrap();

    // measure temperature, pressure, and humidity
    let measurements = bme280.measure(&mut Delay).unwrap();

    println!("Relative Humidity = {}%", measurements.humidity);
    println!("Temperature = {} deg C", measurements.temperature);
    println!("Pressure = {} pascals", measurements.pressure);

    Ok(())
}
