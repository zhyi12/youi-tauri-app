use async_trait::async_trait;
use youi_geo::{Geometry,Centroid, FeatureCollection,Id, Feature, GeoJsonString, Value,JsonObject, json_to_geo};
use services_stats_geo::{AreaGeoJson, AreaGeoJsonDao, AreaGeoJsonService, ServiceResult as Result};
use services_stats_geo_sqlite::AreaGeoJsonSqlite;

///
///
///
pub struct AreaGeoJsonServiceImpl<'a>{

    dao:&'a dyn AreaGeoJsonDao

}

impl<'a> AreaGeoJsonServiceImpl<'a> {

    #[cfg(feature = "sqlite")]
    pub fn new(dao:&'a AreaGeoJsonSqlite)->Self{
        Self{
            dao
        }
    }
}

#[async_trait]
impl<'a> AreaGeoJsonService for AreaGeoJsonServiceImpl<'a> {

    #[cfg(feature = "sqlite")]
    type Dao = AreaGeoJsonSqlite<'a>;

    async fn find_children(&self, pid: &str) -> Result<Vec<AreaGeoJson>> {
        Ok(self.dao.find_children(pid).await?)
    }
}

///
///
///
impl<'a> AreaGeoJsonServiceImpl<'a> {

    ///
    /// 获取所有下级行政区划的geojson
    ///
    pub async fn find_sub_geo_json(&mut self, pid: &str) -> Result<String>{
        let children = self.find_children(pid).await?;

        let features = children.iter()
            .map(|x|{
                let g  =  json_to_geo(&x.geo_json).unwrap();
                let mut f = Feature::from(Value::from(&g));
                f.id = Some(Id::String(x.area_id.clone()));
                let mut properties = JsonObject::new();
                properties.insert("areaId".to_string(),serde_json::Value::String((&x.area_id).to_string()));
                properties.insert("center".to_string(),serde_json::Value::String((&x.center).to_string()));
                properties.insert("name".to_string(),serde_json::Value::String(x.text.clone()));
                f.properties = Some(properties);
                f
            })
            .collect::<Vec<Feature>>();

        Ok(FeatureCollection{
            bbox: None,
            features,
            foreign_members: None
        }.to_string())
    }

    ///
    /// 计算下级行政区划的中心点
    ///
    pub  async fn calculate_sub_center(&mut self, pid: &str) -> Result<()>{
        let children = self.find_children(pid).await?;

        for i in 0..children.len(){
            let g  =  json_to_geo(&children[i].geo_json).unwrap();
            let center = g.centroid().unwrap();
            println!("{:?},{}",center,&children[i].center);
            //更新中心坐标数据
            self.dao.update_center(&children[i].area_id,&format!("{}\"lng\":{},\"lat\":{}{}","{",center.x(),center.y(),"}")).await?
        }

        Ok(())
    }
}