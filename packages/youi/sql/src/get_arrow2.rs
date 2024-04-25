
use crate::{prelude::*, sql::CXQuery};
use fehler::{throw, throws};
use log::debug;

#[allow(unused_imports)]
use std::sync::Arc;

#[cfg(feature = "src_sqlite")]
use crate::sources::sqlite::SQLiteSource;

#[cfg(feature = "src_postgres")]
use crate::sources::postgres::{
    PostgresSource,
    rewrite_tls_args, BinaryProtocol as PgBinaryProtocol, CSVProtocol, CursorProtocol,
    SimpleProtocol,
};


#[cfg(feature = "src_postgres")]
use postgres::NoTls;
#[cfg(feature = "src_postgres")]
use postgres_openssl::MakeTlsConnector;

#[allow(unreachable_code, unreachable_patterns, unused_variables, unused_mut)]
#[throws(ConnectorXOutError)]
pub fn get_arrow2(
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
) -> Arrow2Destination {
    let mut destination = Arrow2Destination::new();
    let protocol = source_conn.proto.as_str();
    debug!("Protocol: {}", protocol);

    match source_conn.ty {
        #[cfg(feature = "src_sqlite")]
        SourceType::SQLite => {
            // remove the first "sqlite://" manually since url.path is not correct for windows
            let path = &source_conn.conn.as_str()[9..];
            let source = SQLiteSource::new(path, queries.len())?;
            let dispatcher = Dispatcher::<_, _, SQLiteArrow2Transport>::new(
                source,
                &mut destination,
                queries,
                origin_query,
            );
            dispatcher.run().unwrap();
        }
        #[cfg(feature = "src_postgres")]
        SourceType::Postgres => {
            let (config, tls) = rewrite_tls_args(&source_conn.conn)?;
            debug!("protocol is {protocol} {}",&source_conn.conn);
            match (protocol, tls) {
                ("csv", Some(tls_conn)) => {
                    let sb = PostgresSource::<CSVProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrow2Transport<CSVProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                ("csv", None) => {
                    let sb =
                        PostgresSource::<CSVProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher =
                        Dispatcher::<_, _, PostgresArrow2Transport<CSVProtocol, NoTls>>::new(
                            sb,
                            &mut destination,
                            queries,
                            origin_query,
                        );
                    dispatcher.run()?;
                }
                ("binary", Some(tls_conn)) => {
                    let sb = PostgresSource::<PgBinaryProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher =
                        Dispatcher::<
                            _,
                            _,
                            PostgresArrow2Transport<PgBinaryProtocol, MakeTlsConnector>,
                        >::new(sb, &mut destination, queries, origin_query);
                    dispatcher.run()?;
                }
                ("binary", None) => {
                    let sb = PostgresSource::<PgBinaryProtocol, NoTls>::new(
                        config,
                        NoTls,
                        queries.len(),
                    )?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrow2Transport<PgBinaryProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                ("cursor", Some(tls_conn)) => {
                    let sb = PostgresSource::<CursorProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrow2Transport<CursorProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                ("cursor", None) => {
                    let sb =
                        PostgresSource::<CursorProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrow2Transport<CursorProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    dispatcher.run()?;
                }
                ("simple", Some(tls_conn)) => {
                    let sb = PostgresSource::<SimpleProtocol, MakeTlsConnector>::new(
                        config,
                        tls_conn,
                        queries.len(),
                    )?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrow2Transport<SimpleProtocol, MakeTlsConnector>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }
                ("simple", None) => {
                    let sb =
                        PostgresSource::<SimpleProtocol, NoTls>::new(config, NoTls, queries.len())?;
                    let dispatcher = Dispatcher::<
                        _,
                        _,
                        PostgresArrow2Transport<SimpleProtocol, NoTls>,
                    >::new(
                        sb, &mut destination, queries, origin_query
                    );
                    debug!("Running dispatcher");
                    dispatcher.run()?;
                }

                _ => unimplemented!("{} protocol not supported", protocol),
            }
        }
        _ => throw!(ConnectorXOutError::SourceNotSupport(format!(
            "{:?}",
            source_conn.ty
        ))),

    }

    destination
}
