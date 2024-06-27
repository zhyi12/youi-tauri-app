use serde::{Serialize, Deserialize};

///
///
///
#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct OrConnect{
    id:String,
    level:u32,
}

impl OrConnect {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_text(&self) -> String {
        "或".to_string()
    }
}

#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct AndConnect{
    id:String,
    level:u32,
}

impl AndConnect {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_text(&self) -> String {
        "且".to_string()
    }
}

