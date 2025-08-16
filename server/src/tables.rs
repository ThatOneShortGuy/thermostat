use anyhow::Result;
use chrono::prelude::*;
use physics_units::TemperatureTrait;
use rusqlite::Connection;
use server_core::Temperature;
use tokio::sync::Mutex;
use typed_db::prelude::*;

use crate::seeders;

type Id = i32;

#[derive(Debug, Clone, DbTable)]
#[allow(unused)]
pub struct HouseTemperature {
    #[primary_key]
    pub id: Id,
    #[foreign_key(Sensor::id)]
    pub sensor_id: Id,
    pub temperature: Temperature,
    pub date_time: DateTime<Utc>,
    #[default(CURRENT_TIMESTAMP)]
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Clone, DbTable)]
#[allow(unused)]
pub struct HousePressure {
    #[primary_key]
    pub id: Id,
    #[foreign_key(Sensor::id)]
    pub sensor_id: Id,
    pub pressure: f32,
    pub date_time: DateTime<Utc>,
    #[default(CURRENT_TIMESTAMP)]
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Clone, DbTable)]
#[allow(unused)]
pub struct HouseHumidity {
    #[primary_key]
    pub id: Id,
    #[foreign_key(Sensor::id)]
    pub sensor_id: Id,
    pub humidity: f32,
    pub date_time: DateTime<Utc>,
    #[default(CURRENT_TIMESTAMP)]
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Clone, DbTable)]
#[allow(unused)]
pub struct Sensor {
    #[primary_key]
    pub id: Id,
    #[unique]
    pub name: String,
    pub active: bool,
    #[default(CURRENT_TIMESTAMP)]
    pub created_date: DateTime<Utc>,
}

pub async fn initialize_tables(conn: &Mutex<Connection>) -> Result<()> {
    let span = tracing::trace_span!("initialize_tables", conn = ?conn);
    let _enter = span.enter();
    let conn = conn.lock().await;
    Sensor::create_table(&conn)?;
    seeders::sensors::sensors(&conn)?;
    HouseTemperature::create_table(&conn)?;
    HouseHumidity::create_table(&conn)?;
    HousePressure::create_table(&conn)?;
    Ok(())
}
