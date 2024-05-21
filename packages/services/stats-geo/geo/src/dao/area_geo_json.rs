use async_trait::async_trait;
use crate::entity::AreaGeoJson;
use crate::ServiceResult;

#[async_trait]
pub trait AreaGeoJsonDao:Sync+Send {

    ///
    ///
    ///
    fn find_children_sql(&self)->String{
        r"select t_.area_id,t_.geo_json,t_.rect,t_.center,ids_.text
            from (select id,text from youi_geo_area where pid=?1) ids_
            left join youi_geo_json t_ on ids_.id = t_.area_id".to_string()
    }

    fn update_center_sql(&self)->String{
        r"update youi_geo_json set center=?2 where area_id=?1".to_string()
    }
    ///
    ///
    ///
    async fn find_children(&self,pid:&str)->ServiceResult<Vec<AreaGeoJson>>;

    ///
    /// 更新行政区划中心坐标
    ///
    async fn update_center(&self,id:&str,center:&str)->ServiceResult<()>;
}