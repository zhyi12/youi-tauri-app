use sqlx::SqlitePool;
use services_stats_geo::{AreaGeoJsonService, ServiceResult};
use services_stats_geo_impl::AreaGeoJsonServiceImpl;
use services_stats_geo_sqlite::AreaGeoJsonSqlite;


const DB_PATH:&str = "/Users/zhouyi/Library/Application Support/youi/app.db";

#[tokio::test]
pub async fn find_children()->ServiceResult<()>{

    let pool = SqlitePool::connect(DB_PATH).await?;
    //
    let dao = AreaGeoJsonSqlite::from(&pool);

    let result  = AreaGeoJsonServiceImpl::new(&dao)
        .calculate_sub_center("460200000000").await?;
    //
    // println!("{}",result.len());

    Ok(())
}