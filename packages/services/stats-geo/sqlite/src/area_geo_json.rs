
use sqlx::{SqlitePool, query_as};
use services_stats_geo::{AreaGeoJson,AreaGeoJsonDao, ServiceResult};
use async_trait::async_trait;
///
///
///

pub struct AreaGeoJsonSqlite<'a> {

    pool:&'a SqlitePool

}

impl <'a> From<&'a SqlitePool> for  AreaGeoJsonSqlite<'a> {
    fn from(pool: &'a SqlitePool) -> Self {
        Self{
            pool
        }
    }
}

///
///
///
#[async_trait]
impl AreaGeoJsonDao for AreaGeoJsonSqlite<'_> {

    ///
    ///
    ///
    async fn find_children(&self,pid: &str) -> ServiceResult<Vec<AreaGeoJson>> {
        let sql = self.find_children_sql();
        let result:Vec<AreaGeoJson> = query_as(&sql)
            .bind(pid)
            .fetch_all(self.pool)
            .await?;
        Ok(result)
    }

    ///
    ///
    ///
    async fn update_center(&self,id:&str,center:&str)->ServiceResult<()>{
        let sql = self.update_center_sql();
        sqlx::query(&sql)
            .bind(id)
            .bind(center)
            .execute(self.pool)
            .await?;
        Ok(())
    }
}
