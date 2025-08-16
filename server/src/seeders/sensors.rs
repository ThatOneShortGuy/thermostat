use anyhow::Result;
use rusqlite::Connection;

use crate::tables::Sensor;

pub fn sensors(conn: &Connection) -> Result<()> {
    let _ = Sensor::new()
        .with_id(0)
        .with_name("Main Thermostat BME280")
        .with_active(true)
        .build(&conn);

    Ok(())
}
