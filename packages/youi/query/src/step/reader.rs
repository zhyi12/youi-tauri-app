use serde::{Serialize, Deserialize};
use crate::dsl::DslNode;
use crate::step::StepInfo;
use crate::reader::Reader as ReaderInfo;

#[derive(Serialize, Deserialize,Debug)]
pub struct Reader{
    ///
    ///
    ///
    #[serde(flatten)]
    pub(crate) info:StepInfo,

    #[serde(flatten)]
    reader:ReaderInfo

}

///
///
///
impl DslNode for Reader {

    ///
    ///
    ///
    fn to_dsl(&self,_input_step_id:Option<&str>) -> crate::error::QueryResult<String> {
        Ok(format!("let {} = {}",self.build_df_id(&self.info.id),
                   self.reader.to_dsl(None)?))
    }
}