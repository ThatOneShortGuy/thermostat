use crate::{
    db_writer,
    tables::{HousePressure, HouseTemperature},
};

use axum::Json;
use server_core::Bme280Payload;
use tracing::debug;

use crate::{error::AppError, tables::HouseHumidity};

#[tracing::instrument]
pub async fn data_handler(
    Json(Bme280Payload {
        temperature,
        pressure,
        humidity,
        date_time,
        sensor_id,
    }): Json<Bme280Payload>,
) -> Result<(), AppError> {
    debug!("Recieved Data from {sensor_id}");
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
