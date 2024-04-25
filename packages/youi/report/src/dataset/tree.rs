use uuid::Uuid;
use crate::tree::LevelNode;

///
/// 数据集树节点
///
#[derive(Debug)]
pub enum Node{
    ///
    /// 数据表节点
    ///
    Table(TableNode),
    ///
    /// 子表文件夹节点
    ///
    Sub(SubNode),
    ///
    /// 从表节点
    ///
    Join(JoinNode),
    ///
    /// 合并表节点
    ///
    Union(UnionNode),
    ///
    /// 列节点
    ///
    Column(ColumnNode)
}

impl Node {

    ///
    ///
    ///
    pub fn id(&self)->&str{
        match self{
            Node::Table(x) => &x.id,
            Node::Sub(x) => &x.id,
            Node::Join(x) => &x.id,
            Node::Union(x) => &x.id,
            Node::Column(x) => &x.id
        }
    }

    pub fn name(&self)->&str{
        match self{
            Node::Table(x) => &x.name,
            _=>""
        }
    }
}

#[derive(Debug)]
pub struct TableNode{
    id:String,
    ///
    ///
    ///
    name:String,
    ///
    ///
    ///
    text:Option<String>,

    level:u32,
}

impl TableNode {
    pub fn new(name:String,text:String,level:u32)->Self{
        Self{
            id:Uuid::new_v4().to_string(),
            name,
            text:Some(text),
            level
        }
    }
}

impl TableNode {
    ///
    ///
    ///
    pub fn text(&self)->String{
        (self.text.as_ref().unwrap_or(&self.name)).to_string()
    }

    ///
    ///
    ///
    pub fn name(&self)->&str{
        &self.name
    }
    ///
    ///
    ///
    pub fn name_clone(&self)->String{
        self.name.to_string()
    }
}

#[derive(Debug)]
pub struct ColumnNode{
    id:String,
    level:u32,
}

impl LevelNode for Node {


    fn get_id(&self) -> String {
        match self {
            Node::Table(t) => t.id.to_string(),
            Node::Sub(t) => t.id.to_string(),
            Node::Join(t) => t.id.to_string(),
            Node::Union(t) => t.id.to_string(),
            Node::Column(t) => t.id.to_string(),
        }
    }
    ///
    ///
    ///
    fn get_level(&self) -> u32 {
        match self {
            Node::Table(t) => t.level,
            Node::Sub(t) => t.level,
            Node::Join(t) => t.level,
            Node::Union(t) => t.level,
            Node::Column(t) => t.level,
        }
    }

    fn get_text(&self) -> String {
        match self {
            Node::Table(t) => t.text(),
            Node::Column(_) => "".to_string(),
            Node::Sub(_) => "子表".to_string(),
            Node::Join(_) => "从表".to_string(),
            Node::Union(_) => "合并表".to_string(),
        }
    }

}

#[derive(Debug)]
pub struct SubNode{
    id:String,
    level:u32,
}

impl SubNode {
    pub fn new(level:u32)->Self{
        Self{
            id:Uuid::new_v4().to_string(),
            level
        }
    }
}

#[derive(Debug)]
pub struct UnionNode{
    id:String,
    level:u32,
}

impl UnionNode {
    pub fn new(level:u32)->Self{
        Self{
            id:Uuid::new_v4().to_string(),
            level
        }
    }
}

#[derive(Debug)]
pub struct JoinNode{
    id:String,
    level:u32,
}

impl JoinNode {
    pub fn new(level:u32)->Self{
        Self{
            id:Uuid::new_v4().to_string(),
            level
        }
    }
}

impl JoinNode {
    pub fn id(&self)->&str{
        &self.id
    }
}