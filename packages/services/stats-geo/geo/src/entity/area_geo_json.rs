///
///
///
#[derive(sqlx::FromRow,Debug)]
pub struct AreaGeoJson{
    ///
    ///
    ///
    pub area_id:String,

    ///
    ///
    ///
    pub text:String,
    ///
    ///
    ///
    pub geo_json:String,
    ///
    ///
    ///
    rect:String,
    ///
    ///
    ///
    pub center:String,

}