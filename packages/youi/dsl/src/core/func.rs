
///
/// 四则运算和逻辑比较
///
pub enum ColSymbolFunc{
    Operation,
    Comparison,
    Empty
}

impl From<&str> for ColSymbolFunc{
    fn from(symbol: &str) -> Self {
        let op = OperationFunc::from(symbol);
        let cp = ComparisonFunc::from(symbol);

        match (op,cp) {
            (OperationFunc::Empty,ComparisonFunc::Empty)=>Self::Empty,
            (OperationFunc::Empty,_)=>Self::Comparison,
            (_,ComparisonFunc::Empty)=>Self::Operation,
            (_,_)=>Self::Empty
        }
    }
}

///
/// 四则运算函数
///
pub enum OperationFunc{
    Add,
    Sub,
    Div,
    Mul,
    Empty,
}

impl From<&str> for OperationFunc {
    fn from(value: &str) -> Self {
        match value {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => Self::Empty
        }
    }
}

impl OperationFunc {
    pub fn name(&self)->&str{
        match self {
            OperationFunc::Add => "add",
            OperationFunc::Sub => "sub",
            OperationFunc::Div => "div",
            OperationFunc::Mul => "mul",
            OperationFunc::Empty => ""
        }
    }

    pub fn symbol(&self)->&str{
        match self {
            OperationFunc::Add => "+",
            OperationFunc::Sub => "-",
            OperationFunc::Div => "/",
            OperationFunc::Mul => "*",
            OperationFunc::Empty => ""
        }
    }
}

///
/// 比较符号
///
pub enum ComparisonFunc{
    Eq,
    NotEq,
    Gt,
    Gte,
    Lt,
    Lte,
    Empty,
}

impl From<&str> for ComparisonFunc {
    fn from(value: &str) -> Self {
        match value {
            "=="=>Self::Eq,
            "!="=>Self::NotEq,
            ">"=>Self::Gt,
            ">="=>Self::Gte,
            "<"=>Self::Lt,
            "<="=>Self::Lte,
            _ => Self::Empty
        }
    }
}

impl ComparisonFunc {
    pub fn name(&self)->&str {
        match self {
            ComparisonFunc::Eq => "eq",
            ComparisonFunc::NotEq => "not_eq",
            ComparisonFunc::Gt => "gt",
            ComparisonFunc::Gte => "gte",
            ComparisonFunc::Lt => "lt",
            ComparisonFunc::Lte => "lte",
            ComparisonFunc::Empty => ""
        }
    }

    pub fn symbol(&self)->&str {
        match self {
            ComparisonFunc::Eq => "==",
            ComparisonFunc::NotEq => "!=",
            ComparisonFunc::Gt => ">",
            ComparisonFunc::Gte => ">=",
            ComparisonFunc::Lt => "<",
            ComparisonFunc::Lte => "<=",
            ComparisonFunc::Empty => "",
        }
    }
}

///
/// 列函数
/// "col","col_expression","when","lit","concat_str","concat_list"
///
pub enum DfColExprFunc{
    ///
    ///
    ///
    Col,

    When,

    Then,

    Otherwise,

    Lit,

    ConcatStr,

    ConcatList,

    Empty

}

impl From<&str> for DfColExprFunc{

    fn from(value: &str) -> Self {
        match value {
            "col"=>Self::Col,
            "when"=>Self::When,
            "then"=>Self::Then,
            "otherwise"=>Self::Otherwise,
            "lit"=>Self::Lit,
            "concat_str"=>Self::ConcatStr,
            "concat_list"=>Self::ConcatList,
            _ => Self::Empty
        }
    }
}

pub enum LitWrapFunc{

    Then,

    Otherwise,

    Empty

}

impl From<&str> for LitWrapFunc {
    fn from(value: &str) -> Self {
        match  value{
            "then"=>Self::Then,
            "otherwise"=>Self::Otherwise,
            _=>Self::Empty
        }
    }
}