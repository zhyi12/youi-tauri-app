
extern crate core;

use log::debug;
use polars_core::prelude::DataFrame;
use crate::get_arrow2::get_arrow2;
use crate::prelude::parse_source;
use crate::sql::CXQuery;

mod data_order;
mod errors;
#[macro_use]
mod macros;
mod sql;
mod typesystem;
mod sources;
mod utils;
mod destinations;
mod get_arrow2;
mod constants;
mod source_router;
mod dispatcher;
mod transports;
mod pool;
//
pub mod prelude {
    pub use crate::data_order::{coordinate, DataOrder};
    pub use crate::source_router::*;
    pub use crate::destinations::arrow2::Arrow2Destination;
    pub use crate::errors::{ConnectorXError, ConnectorXOutError};
    pub use crate::dispatcher::{Dispatcher};
    pub use crate::transports::*;

    #[cfg(feature = "src_postgres")]
    pub use crate::sources::postgres::PostgresSource;
}

///
///
///
pub fn read_sql(conn:&str,query_sql:&str,protocol:&str)->destinations::arrow2::Result<DataFrame>{

    let source_conn = parse_source(conn,Some(protocol))?;

    let queries= vec![CXQuery::from( query_sql)];

    debug!("{}",&source_conn.proto);
    let destination = get_arrow2(&source_conn, Some(query_sql.to_string()), &queries).unwrap();

    destination.polars()

}