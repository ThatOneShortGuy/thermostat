use std::{thread::sleep, time::Duration};

use bme280::{Measurements, i2c::BME280};
use chrono::Utc;
use linux_embedded_hal::{Delay, I2cdev};
use rppal::gpio::Gpio;

use anyhow::{Context, Result};

use physics_units::{
    TemperatureTrait,
    temperatures::{Celsius, Fahrenheit},
};
use server_core::{Bme280Payload, Temperature};

use tracing::Level;

fn main_loop(mut bme280: BME280<I2cdev>) -> ! {
    loop {
        let Measurements {
            temperature,
            pressure,
            humidity,
            ..
        } = match bme280.measure(&mut Delay) {
            Ok(m) => m,
            Err(err) => {
                tracing::error!("Failed getting measurements: {err:?}");
                sleep(Duration::from_secs(10));
                continue;
            }
        };

        let temperature = Celsius::new(temperature);
        let temperature = Temperature(temperature.convert_to());

        let data = Bme280Payload {
            temperature,
            pressure,
            humidity,
            date_time: Utc::now(),
            sensor_id: 0,
        };

        tracing::info!(
            "Sending {pressure:.2} Pa, {humidity:.2}%, {}",
            temperature.0.convert_to::<Fahrenheit>()
        );

        match ureq::post("http://192.168.0.191:23564/data").send_json(&data) {
            Ok(_body) => (),
            Err(err) => tracing::error!("Error: {err}"),
        };

        sleep(Duration::from_secs(20));
    }
}

fn main() -> Result<()> {
    // Initialize the logger
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::Subscriber::builder()
        // .with_file(true)
        // .with_line_number(true)
        .with_max_level(Level::DEBUG)
        .init();
    #[cfg(not(debug_assertions))]
    tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(Level::INFO)
        .with_ansi(true)
        .init();

    let _h = std::thread::spawn(|| {
        let gpio = Gpio::new().context("new gpio")?;
        let mut bcm06 = gpio.get(6)?.into_output();
        let mut bcm13 = gpio.get(13)?.into_output();
        let mut bcm19 = gpio.get(19)?.into_output();
        let mut bcm26 = gpio.get(26)?.into_output();

        for _ in 0..2 {
            println!("turning high {}", bcm06.is_set_high());
            bcm06.set_low();
            sleep(Duration::from_millis(200));
            bcm13.set_low();
            sleep(Duration::from_millis(200));
            bcm19.set_low();
            sleep(Duration::from_millis(200));
            bcm26.set_low();
            sleep(Duration::from_millis(200));
            println!("turning low {}", bcm06.is_set_high());
            bcm06.set_high();
            sleep(Duration::from_millis(200));
            bcm13.set_high();
            sleep(Duration::from_millis(200));
            bcm19.set_high();
            sleep(Duration::from_millis(200));
            bcm26.set_high();
            sleep(Duration::from_millis(200));
        }
        Ok::<_, anyhow::Error>(())
    });

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

    // do one measurement give it a second to let it calibrate
    let _ = bme280.measure(&mut Delay);

    main_loop(bme280);
}
