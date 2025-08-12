use anyhow::Result;
use chrono::prelude::*;
use physics_units::{TemperatureTrait, temperatures::Kelvin};
use rusqlite::{Connection, ToSql, types::FromSql};
use tokio::sync::Mutex;
use typed_db::prelude::*;

type Id = i64;

#[derive(Debug, Clone, Copy)]
pub struct Temperature(physics_units::temperatures::Temperature<Kelvin>);

impl Default for Temperature {
    fn default() -> Self {
        Self(Kelvin::new(0.0))
    }
}

impl ToSql for Temperature {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.0.0.0.to_sql()
    }
}

impl FromSql for Temperature {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let f = f64::column_result(value)?;
        Ok(Temperature(Kelvin::new(f)))
    }
}

impl DbType for Temperature {
    fn db_type() -> &'static str {
        f64::db_type()
    }
}

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
pub struct Sensor {
    #[primary_key]
    pub id: Id,
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
    HouseTemperature::create_table(&conn)?;
    Ok(())
}
