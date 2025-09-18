use crate::{
    db_writer,
    tables::{HousePressure, HouseTemperature},
};

use axum::Json;
use physics_units::{TemperatureTrait, temperatures::Fahrenheit};
use server_core::Bme280Payload;
use tracing::info;

use crate::{error::AppError, tables::HouseHumidity};

pub async fn data_handler(
    Json(Bme280Payload {
        temperature,
        pressure,
        humidity,
        date_time,
        sensor_id,
    }): Json<Bme280Payload>,
) -> Result<(), AppError> {
    info!(
        "Recieved Data from {sensor_id}: {humidity:.2}%, {pressure:.0} Pa, {:.2}",
        temperature.0.convert_to::<Fahrenheit>()
    );
    HouseHumidity::new()
        .with_sensor_id(sensor_id)
        .with_humidity(humidity)
        .with_date_time(date_time)
        .build_raw(db_writer!())?;

    HousePressure::new()
        .with_sensor_id(sensor_id)
        .with_pressure(pressure)
        .with_date_time(date_time)
        .build_raw(db_writer!())?;

    HouseTemperature::new()
        .with_sensor_id(sensor_id)
        .with_temperature(temperature)
        .with_date_time(date_time)
        .build_raw(db_writer!())?;

    Ok(())
}
