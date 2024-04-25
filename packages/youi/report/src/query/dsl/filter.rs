use log::debug;
use crate::filter::{FilterItem, FilterNode};
use crate::tree::{Tree, TreeVisitor};


struct DslFilterTreeVisitor{
    dsl:String
}

impl DslFilterTreeVisitor {
    fn new()->Self{
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
                        item.dsl()
                    }else{
                        format!(".and({})",item.dsl())
                    }
                },
                (FilterNode::Item(item),FilterNode::Or(_))=>{
                    if is_start{
                        item.dsl()
                    }else{
                        format!(".or({})",item.dsl())
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

impl Tree<FilterNode> {

    ///
    ///
    ///
    pub fn dsl(&self)->String{
        let mut visitor = DslFilterTreeVisitor::new();

        self.visit(&mut visitor);

        let dsl = visitor.dsl.to_string();

        debug!("nodes :{:?}",self.nodes());
        debug!("filter dsl :{dsl}");

        dsl
    }

}

///
///
///
impl FilterItem {
    ///
    /// 条件表达式
    ///
    pub fn dsl(&self)->String{
        //单值
        let quote_value = self.get_quote_value("\"");

        let mut wrap_value = String::new();
        let no_need_wrap = self.operator() == "start_with" || self.operator() == "end_with";
        if !no_need_wrap{
            wrap_value.push_str("lit(");
        }
        wrap_value.push_str(&quote_value);
        if !no_need_wrap{
            wrap_value.push_str(")");
        }

        format!("col(\"{}\").{}({})",self.property(),self.operator(),wrap_value)
    }
}
