
#[allow(dead_code)]
const KILO: usize = 1 << 10;

pub const RECORD_BATCH_SIZE: usize = 64 * KILO;

pub(crate) const SECONDS_IN_DAY: i64 = 86_400;

pub const CONNECTORX_PROTOCOL: &str = "cxprotocol";

#[cfg(any(
    feature = "src_postgres",
    feature = "src_mysql",
    feature = "src_oracle",
    feature = "src_mssql"
))]
pub const DB_BUFFER_SIZE: usize = 32;