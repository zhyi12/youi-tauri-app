use itertools::Itertools;
use log::debug;
use crate::filter::{FilterItem, FilterNode, FilterOperator};
use crate::tree::{Tree, TreeVisitor};

struct SqlFilterTreeVisitor{
    script:String
}

impl SqlFilterTreeVisitor {
    fn new()->Self{
        Self{
            script:String::new()
        }
    }
}

///
/// expr1 == 1 && expr2 == 2 && (expr3 like '' or expr4 == 9)
///
impl TreeVisitor<&FilterNode> for SqlFilterTreeVisitor {
    fn start(&mut self) {
        self.script = String::new();
    }

    ///
    ///
    ///
    fn node_start(&mut self, node: &FilterNode, parent_node: Option<&FilterNode>) {
        match parent_node {
            None => {
                //最外层的 And、Or 生成 左括号
                self.script.push_str("(");
            }
            Some(parent) => {
                let is_start = self.script.ends_with("(");
                self.script.push_str(&match (node,parent) {
                    (FilterNode::Item(item),FilterNode::And(_))=> {
                        format!("{} {}",if is_start{""} else {" and "},item.sql())
                    }
                    (FilterNode::Item(item),FilterNode::Or(_))=>{
                        format!("{} {}",if is_start{""} else {" or "},item.sql())
                    }
                    (FilterNode::And(_)|FilterNode::Or(_),_)=>{
                        "(".to_string()
                    }
                    _=>"".to_string()
                });
            }
        }
    }

    ///
    ///
    ///
    fn node_end(&mut self, node: &FilterNode, _children: Vec<&FilterNode>) {
        match node {
            FilterNode::Or(_) | FilterNode::And(_) => {
                self.script.push_str(")")
            }
            _=> {}
        }
    }

    fn end(&mut self) {
        if self.script.len()>0{
            //删除最外层的括号
            self.script.remove(self.script.len()-1);
            self.script.remove(0);
        }
    }
}

impl FilterItem {

    ///
    /// sql 条件
    ///
    pub fn sql(&self)->String{
        let quote = "'";
        let op = FilterOperator::from(self.operator());
        let symbol = op.symbol();
        let value = match op{
            FilterOperator::Like=>self.get_wrap_value(quote,"%","%"),
            FilterOperator::StartWith=>self.get_wrap_value(quote,"","%"),
            FilterOperator::EndWith=>self.get_wrap_value(quote,"%",""),
            FilterOperator::IsNull|FilterOperator::IsNotNull=>"".to_string(),
            FilterOperator::Eq | FilterOperator::Neq |FilterOperator::Lt
                    | FilterOperator::Lte |FilterOperator::Gt | FilterOperator::Gte => self.get_quote_value(quote),
            FilterOperator::In=>format!("[{}]",self.get_quote_values(quote).iter().join(",")),
            _=>"".to_string(),
        };
        format!("{} {} {}",self.property(),symbol,value)
    }
}


impl Tree<FilterNode> {

    ///
    ///
    ///
    pub fn sql(&self)->String{
        let mut visitor = SqlFilterTreeVisitor::new();

        self.visit(&mut visitor);

        let sql = visitor.script.to_string();

        debug!("filter sql :{sql}");

        sql
    }

}