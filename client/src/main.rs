use std::{thread::sleep, time::Duration};

use bme280::{Measurements, i2c::BME280};
use chrono::Utc;
use linux_embedded_hal::{Delay, I2cdev};

use anyhow::Result;

use physics_units::{TemperatureTrait, temperatures::Celsius};
use server_core::{Bme280Payload, Temperature};

fn main() -> Result<()> {
    // Open the Pi's I²C bus
    let i2c_bus = I2cdev::new("/dev/i2c-1")?;

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
    loop {
        let Measurements {
            temperature,
            pressure,
            humidity,
            ..
        } = bme280.measure(&mut Delay).unwrap();

        let temperature = Celsius::new(temperature);
        let temperature = Temperature(temperature.convert_to());

        let data = Bme280Payload {
            temperature,
            pressure,
            humidity,
            date_time: Utc::now(),
            sensor_id: 0,
        };
        println!("Sending {data:?}");

        match ureq::post("http://192.168.0.191:23564/data").send_json(&data) {
            Ok(_body) => (),
            Err(err) => println!("Error: {err}"),
        };

        sleep(Duration::from_secs(30));
    }

    Ok(())
}
