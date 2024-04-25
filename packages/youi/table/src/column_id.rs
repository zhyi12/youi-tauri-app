

///
/// 表格列ID
///
pub struct ColumnId{
    ///
    /// 序号
    ///
    index:u32,
    ///
    /// 字母
    ///
    name:String

}
///
///
///
impl From<&str> for ColumnId {

    fn from(name: &str) -> Self {
        Self{
            index:name_to_id(name),
            name:name.to_string()
        }
    }
}

impl From<u32> for ColumnId {
    fn from(index: u32) -> Self {
        Self{
            index,
            name:id_to_name(index)
        }
    }
}

impl ColumnId {
    ///
    ///
    ///
    pub fn index(&self)->u32{
        self.index
    }

    pub fn name(&self)->&str{
        &self.name
    }
}

///
/// excel 列号转序号（从1开始）
///
fn name_to_id(name:&str)->u32{
    let len = name.len();
    (0..len).map(|idx|{
        let opt_char = name[idx..idx+1].chars().next();
        match opt_char {
            None => 0,
            Some(c) =>  (c as u32 - 64)*26_u32.pow((len - idx - 1) as u32)
        }
    }).reduce(|a,v|a+v).unwrap()
}
///
/// 序号转字符
///
fn id_to_name(id: u32) -> String {
    let mut dividers = Vec::default();

    let mut reminder = id;
    loop {
        let next_reminder = reminder / 27;
        dividers.push(reminder - next_reminder * 26);

        if next_reminder == 0 {
            break;
        }

        reminder = next_reminder;
    }

    dividers
        .into_iter()
        .rev()
        .map(|c| char::from(c as u8 + 64))
        .collect()
}
