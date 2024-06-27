use itertools::Itertools;
use youi_table::DataType;
use youi_tree::TreeVisitor;
use crate::filter::{FilterNode,FilterItem};

pub struct DslFilterTreeVisitor{
    pub(crate) dsl:String
}

impl DslFilterTreeVisitor {
    pub fn new()->Self{
        Self{
            dsl:String::new()
        }
    }
}

///
/// expr1.and(expr2).and(expr31.or(expr32))
///
impl TreeVisitor<&FilterNode> for DslFilterTreeVisitor {

    fn start(&mut self) {
        self.dsl = String::new();
    }

    ///
    ///
    ///
    fn node_start(&mut self, node: &FilterNode,parent_node:Option<&FilterNode>) {

        if parent_node.is_some(){
            let is_start = self.dsl.ends_with("(");
            let script = match (node,parent_node.unwrap()) {
                (FilterNode::Item(item),FilterNode::And(_))=> {
                    if is_start{
                        item_to_dsl(item)
                    }else{
                        format!(".and({})",item_to_dsl(item))
                    }
                },
                (FilterNode::Item(item),FilterNode::Or(_))=>{
                    if is_start{
                        item_to_dsl(item)
                    }else{
                        format!(".or({})",item_to_dsl(item))
                    }
                },
                (FilterNode::And(_)|FilterNode::Or(_),_)=>{
                    "(".to_string()
                },
                (_,_)=>"".to_string()
            };

            self.dsl.push_str(&script);
        }else{
            //最外层的 And、Or 生成 左括号
            self.dsl.push_str("(");
        }
    }

    fn node_end(&mut self, node: &FilterNode,_children:Vec<&FilterNode>) {
        match node {
            FilterNode::Or(_) | FilterNode::And(_) => {
                self.dsl.push_str(")")
            }
            _=> {}
        }
    }

    fn end(&mut self) {
        if self.dsl.len()>0{
            //删除最外层的括号
            self.dsl.remove(self.dsl.len()-1);
            self.dsl.remove(0);
        }
    }
}

///
///
///
pub fn item_to_dsl(item:&FilterItem)->String{

    let logic_dsl = match item.operator.as_str() {
        "is_not_null"|"not_null"=>format!("{}()",item.operator),
        "in"=>{
            format!("is_in([{}])",item.value.iter().map(|v|match v{
                DataType::String(x)=>format!("\"{}\"",x),
                DataType::Float(x)=>x.to_string(),
                DataType::Int(x)=>x.to_string(),
                DataType::Bool(x)=>x.to_string(),
                _=>"".to_string()
            }).join(","))
        },
        _=>{
            //单值
            let quote_value = item.get_quote_value("\"");

            let mut wrap_value = String::new();
            let no_need_wrap = &item.operator == "start_with" || &item.operator == "end_with";
            if !no_need_wrap{
                wrap_value.push_str("lit(");
            }
            wrap_value.push_str(&quote_value);
            if !no_need_wrap{
                wrap_value.push_str(")");
            }
            format!("{}({})",item.operator,wrap_value)
        }
    };

    format!("col(\"{}\").{}",item.property,logic_dsl)
}