//! This module contains transport definitions for the sources and destinations implemented in ConnectorX.
#[cfg(all(feature = "src_sqlite"))]
mod sqlite_arrow2;

#[cfg(all(feature = "src_postgres"))]
mod postgres_arrow2;

#[cfg(all(feature = "src_sqlite"))]
pub use sqlite_arrow2::{SQLiteArrow2Transport, SQLiteArrow2TransportError};

#[cfg(all(feature = "src_postgres"))]
pub use postgres_arrow2::{PostgresArrow2Transport, PostgresArrow2TransportError};
