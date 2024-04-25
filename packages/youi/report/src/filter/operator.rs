

pub enum FilterOperator{
    ///
    /// 等于
    ///
    Eq,
    ///
    /// 不等于
    ///
    Neq,
    ///
    /// 小于
    ///
    Lt,
    ///
    /// 小于等于
    ///
    Lte,
    ///
    /// 大于
    ///
    Gt,
    ///
    /// 大于等于
    ///
    Gte,
    ///
    /// 不为空
    ///
    IsNotNull,
    ///
    /// 空
    ///
    IsNull,
    ///
    /// 以...字符开始
    ///
    StartWith,
    ///
    /// 以...字符结束
    ///
    EndWith,
    ///
    /// 模糊匹配
    ///
    Like,
    ///
    /// in
    ///
    In,

    Empty,
}

impl From<&str> for FilterOperator{
    fn from(value: &str) -> Self {
        match value {
            "eq"=>FilterOperator::Eq,
            "neq"=>FilterOperator::Neq,
            "lt"=>FilterOperator::Lt,
            "lte"=>FilterOperator::Lte,
            "gt"=>FilterOperator::Gt,
            "gte"=>FilterOperator::Gte,
            "start_with"=>FilterOperator::StartWith,
            "end_with"=>FilterOperator::EndWith,
            "like"=>FilterOperator::Like,
            "is_null"=>FilterOperator::IsNull,
            "is_not_null"=>FilterOperator::IsNotNull,
            _=>FilterOperator::Empty
        }
    }
}

impl FilterOperator {
    ///
    /// 符号
    ///
    pub fn symbol(&self)->&str{
        match self {
            FilterOperator::Eq => "=",
            FilterOperator::Neq => "!=",
            FilterOperator::Lt => "<",
            FilterOperator::Lte => "<=",
            FilterOperator::Gt => ">",
            FilterOperator::Gte => ">=",
            FilterOperator::In => "in",
            FilterOperator::IsNull=> "is null",
            FilterOperator::IsNotNull=> "is not null",
            FilterOperator::Like|FilterOperator::StartWith|FilterOperator::EndWith=> "like",
            _=>""
        }
    }
}