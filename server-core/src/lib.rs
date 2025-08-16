use std::fmt::Display;

use chrono::{DateTime, Utc};
use physics_units::{TemperatureTrait, temperatures::Kelvin};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Temperature(pub physics_units::temperatures::Temperature<Kelvin>);

impl Serialize for Temperature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.0.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Temperature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self(Kelvin::new(f64::deserialize(deserializer)?)))
    }
}

impl Default for Temperature {
    fn default() -> Self {
        Self(Kelvin::new(0.0))
    }
}

impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Bme280Payload {
    pub temperature: Temperature,
    pub pressure: f32,
    pub humidity: f32,
    pub date_time: DateTime<Utc>,
    pub sensor_id: i32,
}

#[cfg(feature = "server")]
impl rusqlite::ToSql for Temperature {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        self.0.0.0.to_sql()
    }
}

#[cfg(feature = "server")]
impl rusqlite::types::FromSql for Temperature {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let f = f64::column_result(value)?;
        Ok(Temperature(Kelvin::new(f)))
    }
}
#[cfg(feature = "server")]
impl typed_db::DbType for Temperature {
    fn db_type() -> &'static str {
        f64::db_type()
    }
}
