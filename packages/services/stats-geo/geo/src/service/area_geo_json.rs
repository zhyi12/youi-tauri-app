use async_trait::async_trait;
use crate::{AreaGeoJson, AreaGeoJsonDao,ServiceResult as Result};
///
///
///
#[async_trait]
pub trait AreaGeoJsonService:Send+Sync{

    type Dao: AreaGeoJsonDao;

    ///
    ///
    ///
    async fn find_children(&self,pid:&str)->Result<Vec<AreaGeoJson>>;

}