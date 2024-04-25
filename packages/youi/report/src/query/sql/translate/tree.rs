use std::collections::HashMap;
use log::debug;
use petgraph::graph::NodeIndex;
use petgraph::visit::Walker;
use crate::error::{ReportResult as Result};
use rhai::{AST, Engine, Expr, FnCallExpr, Stmt};
use youi_flow::Dag;
use crate::error::ReportError::NotConnectFunc;
use crate::query::sql::translate::translator::{COND, COND_AND, COND_OR};


///
/// ("period_id = 20230000".or("area_id = '42'").or("area_id = '41'")).and("name1 in(['1', '2', '3'])").and("respondent_id = 1".or("respondent_id = 2"))
///  中间格式转sql语句,通过构建条件树来解决括号嵌套生成问题
///
pub struct CondTreeBuilder<'a>{
    engine: &'a Engine,

    cond_expression:&'a str,
    ///
    /// 树节点
    ///
    nodes:Vec<ConditionNode>,

    ///
    /// 有向无环图
    ///
    graph:Dag<usize,(usize,usize)>
}

impl<'a> From<&'a Engine> for CondTreeBuilder<'a> {
    fn from(engine: &'a Engine) -> Self {
        Self{
            engine,
            cond_expression: "",
            nodes:vec![],
            graph:Dag::new()
        }
    }
}

impl<'a> CondTreeBuilder<'a> {

    pub fn cond_expression(mut self,cond_expression:&'a str)->Self{
        self.cond_expression = cond_expression;
        self
    }
}

impl<'a> CondTreeBuilder<'a> {
    ///
    ///
    ///
    pub fn build(&mut self)->Result<String>{
        self.nodes = vec![];
        debug!("tree cond expression {:?}",self.cond_expression);
        let ast:AST = self.engine.compile_expression(&self.cond_expression)?;
        debug!("tree ast {:?}",ast);

        //遍历ast，构建条件节点（表达式节点、连接节点）
        let statements = ast.statements();
        for stmt in statements {
            self.visit_stmt(stmt)?;
        }
        //设置左条件表达式的父亲节点
        self.process_node_tree()?;
        //构建有向无环图
        self.process_graph()?;

        // debug!("tree \n{}",self.nodes.iter().map(|n|format!("{:?}",n)).join("\n"));
        self.build_sql()
    }

    ///
    /// 构建sql
    ///
    fn build_sql(&self)->Result<String>{
        CondSqlBuilder::new(&self.nodes,&self.graph)
            .build()
    }
    ///
    /// 基于有向无环图构建树
    ///
    fn process_graph(&mut self)->Result<()>{
        let mut node_idx_map:HashMap<&str,usize> = HashMap::new();
        //根节点
        self.graph.add_node(0);
        //增加图节点
        self.nodes
            .iter()
            .zip(0..self.nodes.len())
            .for_each(|(n,idx)|{
                let node_idx = idx+1;
                self.graph.add_node(node_idx);
                node_idx_map.insert(n.id(),node_idx);
            });
        //增加节点间的连接
        for idx in 0..self.nodes.len(){
            let index = self.nodes.len()-idx-1;
            let node = &self.nodes[index];
            let source_id = node.pid();
            let source_idx = match node_idx_map.get(source_id){
                None=>0,
                Some(x)=>*x
            };
            //连接线
            self.graph.add_edge(NodeIndex::new(source_idx),NodeIndex::new(index+1),(source_idx,index+1));
        }
        Ok(())
    }
    ///
    /// 节点树处理
    ///
    fn process_node_tree(&mut self)->Result<()>{
        let count = self.nodes.len();
        //设置连接节点两边直接关联的表达式节点的pid
        let expr_conn_map:HashMap<String,String> = HashMap::from_iter((0..count).map(|idx|match &self.nodes[idx] {
            ConditionNode::Exp(exp) => {
                if exp.pid.is_empty(){
                    //往下查找，conn_level = level+1
                    let parent_conn_id = self.find_parent_conn_id(count,idx,exp.level);
                    vec![(exp.id.to_string(),parent_conn_id)]
                }else{
                    vec![]
                }
            },
            ConditionNode::And(id,pid,level)|ConditionNode::Or(id,pid,level) => {
                if pid.is_empty() && *level>0{
                    //先往下查找，conn_level = level+1
                    let mut parent_conn_id = self.find_parent_conn_id(count,idx,*level);
                    //继续往上查找
                    if parent_conn_id.is_empty(){
                        debug!("parent_conn_id {parent_conn_id} - {level} - {id}");
                        parent_conn_id = self.find_top_conn_id(idx,*level);
                    }
                    vec![(id.to_string(),parent_conn_id)]
                }else{
                    vec![]
                }
            },
            _=>vec![]
        }).flatten().collect::<Vec<(String,String)>>());

        self.nodes.iter_mut().for_each(|node|{
            match node {
                ConditionNode::Exp(exp) => {
                    if exp.pid.is_empty(){
                        let opt_conn_id = expr_conn_map.get(&exp.id);
                        match opt_conn_id {
                            None => {}
                            Some(conn_id) => {
                                exp.pid = conn_id.to_string();
                            }
                        }
                    }
                },
                ConditionNode::And(id,pid,..) | ConditionNode::Or(id,pid,..)=>{
                    if pid.is_empty(){
                        let opt_conn_id = expr_conn_map.get(id);
                        match opt_conn_id {
                            None => {}
                            Some(conn_id) => {
                                *pid = conn_id.to_string();
                            }
                        }
                    }
                }
                _=>{}
            }
        });

        Ok(())
    }
    ///
    /// 连接直接相关的conn id
    ///
    fn find_parent_conn_id(&self,count:usize,idx:usize,exp_level:usize)->String{
        let opt_parent_conn_idx = (idx+1..count).find(|conn_idx|{
            match &self.nodes[*conn_idx] {
                ConditionNode::And(_,_,level)|ConditionNode::Or(_,_,level)=>exp_level == level+1,
                _=>false
            }
        });

        match opt_parent_conn_idx {
            None => "".to_string(),
            Some(parent_conn_idx) => match &self.nodes[parent_conn_idx]{
                ConditionNode::And(id,..) |  ConditionNode::Or(id,..) =>id.to_string(),
                _=>"".to_string()
            }
        }
    }

    ///
    /// 往上查找父级节点
    ///
    fn find_top_conn_id(&self,idx:usize,exp_level:usize)->String{
        let opt_parent_conn_idx = (0..idx).find(|i|{
            let conn_idx = idx - i -1;
            match &self.nodes[conn_idx+1] {
                ConditionNode::And(_,_,level)|ConditionNode::Or(_,_,level)=>exp_level == level+1,
                _=>false
            }
        });

        match opt_parent_conn_idx {
            None => "".to_string(),
            Some(parent_conn_idx) => match &self.nodes[parent_conn_idx+1]{
                ConditionNode::And(id,..) |  ConditionNode::Or(id,..) =>id.to_string(),
                _=>"".to_string()
            }
        }
    }

    ///
    /// 处理表达式类型的Stmt
    ///
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<()>{
        match stmt {
            Stmt::Expr(x) => self.visit_expr(x,0,"",false,true)?,
            _=>{}
        }
        Ok(())
    }

    ///
    /// 生成UUID
    ///
    fn build_node_id()->String{
        uuid::Uuid::new_v4().to_string()
    }

    ///
    /// 表达式处理
    ///
    fn visit_expr(&mut self,expr:&Expr,level:usize,parent_conn_id:&str,chain:bool,begin:bool) -> Result<()>{
        match expr {
            Expr::StringConstant(x,_)=>{
                self.nodes.push(self.build_exp_node(&x.to_string(),parent_conn_id,level));
            }
            Expr::Dot(x,..)=>{
                // 是否是cond函数，用于规避括号开始的表达式Ast识别为Index的问题
                let has_start_cond = level==0 && match &x.lhs {
                    Expr::FnCall(x,..)=>{
                        let name = &x.name.to_string();
                        name == COND
                    },
                    _=>false
                };
                if has_start_cond{
                    match &x.lhs {
                        //cond 开始开始处理，args[0]为表达式
                        Expr::FnCall(f, ..) => {
                            self.visit_expr(&f.args[0],1,"",false,true)?;
                        },
                        _=>{}
                    }
                    //继续处理右半部分
                    self.visit_expr(&x.rhs,0,"",false,begin)?;
                }else{
                    self.visit_connect(&x.lhs,&x.rhs,level,parent_conn_id,begin)?;
                }
            }
            Expr::FnCall(x,_)|Expr::MethodCall(x,_)=>{
                let mut pid = parent_conn_id.to_string();
                if begin || !chain{
                    let connect_node = self.build_connect_node(&x, parent_conn_id, level)?;
                    pid = connect_node.id().to_string();
                    self.nodes.push(connect_node);
                }
                //arg为dot的时候
                match &x.args[0]{
                    Expr::Dot(dot,..)=>{
                        //参数为Dot类型时，参数为表达式组, 组的begin参数设置为true。
                        self.visit_connect(&dot.lhs,&dot.rhs,level+1,&pid,true)?;
                    }
                    _=>{
                        //其他参数表达式
                        self.visit_expr(&x.args[0],level+1,&pid,false,begin)?;
                    }
                }
            }
            _=>{
                debug!("tree other {:?}",expr)
            }
        };
        Ok(())
    }

    ///
    /// 1、lhs为条件字符串、rhs为MethodCall : "col(\"period_id\") == 20230000 && col(\"area_id\") == \"42\""
    /// 2、lhs为连接函数、rhs为MethodCall 右边
    /// 3、lhs为条件字符串、rhs为Dot 左边=右边+1
    /// 4、lhs为连接函数、rhs为Dot （左右同级）
    ///
    fn visit_connect(&mut self,lhs:&Expr,rhs:&Expr,level:usize,parent_conn_id:&str,begin:bool)->Result<()>{
        match (lhs,rhs) {
            (Expr::StringConstant(_,..),Expr::MethodCall(_,..))=>{
                let pid = if begin {parent_conn_id} else {""};
                //左边部分处理
                self.visit_expr(lhs,level+1,"",false,begin)?;
                //右边部分处理
                self.visit_expr(rhs,level,pid,false,true)?;
            }
            (Expr::StringConstant(..),Expr::Dot(..))=>{
                //链式表达式的开始
                self.visit_expr(lhs,level+1,"",false,false)?;
                self.visit_expr(rhs,level,"",false,true)?;
            }
            (Expr::MethodCall(m1,..),Expr::MethodCall(m2,..))=>{
                let lhs_name = &m1.name;
                let rhs_name = &m2.name;
                //连接节点
                let conn_node = self.build_connect_node(m1,parent_conn_id,level)?;
                let conn_id = conn_node.id().to_string();
                self.nodes.push(conn_node);
                self.visit_expr(lhs,level,&conn_id,true,false)?;

                if lhs_name == rhs_name{
                    //跳过有表达式的函数节点，父节点直接使用左边的连接节点
                    self.visit_expr(&m2.args[0],level+1,&conn_id,true,false)?;
                }else{
                    self.visit_expr(rhs,level,parent_conn_id,false,false)?;
                }
            }
            (Expr::MethodCall(f,..),Expr::Dot(dot,..))=>{
                let left_name = &f.name.to_string();
                self.visit_expr(lhs,level,parent_conn_id,true,false)?;
                let right_name = match &dot.lhs {
                    Expr::MethodCall(rf,..)=>{
                        &rf.name
                    },
                    _=>""
                };
                self.visit_expr(rhs,level,parent_conn_id,left_name.eq(right_name),false)?;
            }
            (_,_)=>{
                debug!("other lhs {:?}",lhs);
                debug!("other rhs {:?}",rhs);
            }
        };
        Ok(())
    }

    ///
    /// 创建连接节点
    ///
    fn build_connect_node(&self,fn_expr:&FnCallExpr,parent_conn_id:&str,level:usize)->Result<ConditionNode>{
        let name = &fn_expr.name.to_string();
        let conn_id = Self::build_node_id();
        if name == COND_AND{
            Ok(ConditionNode::And(conn_id.to_string(),parent_conn_id.to_string(),level))
        }else if name == COND_OR {
            Ok(ConditionNode::Or(conn_id.to_string(), parent_conn_id.to_string(), level))
        }else{
            Err(NotConnectFunc(name.to_string()))
        }
    }

    ///
    /// 创建表达式节点
    ///
    fn build_exp_node(&self,cond:&str,parent_conn_id:&str,level:usize)->ConditionNode{
        ConditionNode::Exp(ExpressionNode::new(Self::build_node_id(),parent_conn_id.to_string(),cond.to_string(),level))
    }
}

#[derive(Debug,Clone)]
pub enum ConditionNode{
    ///
    /// (id,pid,level)
    ///
    And(String,String,usize),
    ///
    /// (id,pid,level)
    ///
    Or(String,String,usize),
    ///
    /// (id,pid,expr,level)
    ///
    Exp(ExpressionNode),

    Root
}

impl ConditionNode {
    pub fn id(&self)->&str{
        match self {
            ConditionNode::And(id, _, _)|ConditionNode::Or(id, _, _) => id,
            ConditionNode::Exp(exp) => &exp.id,
            ConditionNode::Root=>"",
        }
    }

    pub fn pid(&self)->&str{
        match self {
            ConditionNode::And(_, pid, _)|ConditionNode::Or(_, pid, _) => pid,
            ConditionNode::Exp(exp) => &exp.pid,
            ConditionNode::Root=>"",
        }
    }
}

pub struct CondSqlBuilder<'a>{
    nodes:&'a Vec<ConditionNode>,
    dag:&'a Dag<usize,(usize,usize)>,
    script:String,
}

impl<'a> CondSqlBuilder<'a>  {
    pub fn new(nodes:&'a Vec<ConditionNode>,
               dag:&'a Dag<usize,(usize,usize)>)->Self{
        Self{
            nodes,dag,script:"".to_string()
        }
    }
}

impl<'a> CondSqlBuilder<'a>  {

    pub fn build(&mut self)->Result<String>{
        self.script = "".to_string();
        //
        self.visit_children(0,"")?;

        if self.nodes.len()>1 && self.script.len()>1{
            //删除最外层的括号
            self.script.pop();
            self.script.remove(0);
        }

        debug!("cond sql build: {}",self.script);

        Ok(self.script.to_string())
    }

    ///
    ///
    ///
    fn visit_children(&mut self,parent_idx:usize,parent_symbol:&str)->Result<()>{
        let children = self.dag.children(NodeIndex::new(parent_idx));

        let mut count = 0;
        let start = self.script.len();

        self.script.push_str("(");

        children.iter(self.dag).for_each(|(_,n)|{
            let node_idx = n.index();
            if node_idx>0 {
                if count>0{
                    self.script.push_str(&format!(" {} ",parent_symbol));
                }
                match &self.nodes[node_idx-1] {
                    ConditionNode::And(_, _, _) => {
                        self.visit_children(n.index(),COND_AND).unwrap();
                    },
                    ConditionNode::Or(_, _, _) => {
                        self.visit_children(n.index(),COND_OR).unwrap();
                    }
                    ConditionNode::Exp(exp) => {
                        self.script.push_str(&exp.expression);
                    }
                    _ => {}
                }
            }
            count = count+1;
        });

        if count>1{
            self.script.push_str(")");
        }else{
            //删除start位置的括号
            self.script.remove(start);
        }

        Ok(())
    }
}

#[derive(Debug,Clone)]
pub struct ExpressionNode{
    id:String,
    pid:String,
    expression:String,
    level:usize
}

impl ExpressionNode {
    pub fn new(id:String,pid:String,expression:String,level:usize)->Self{
        Self{
            id,
            pid,
            expression,
            level
        }
    }
}
