use serde::{Serialize, Deserialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegressionConfig{

    ///
    /// 线性回归线 (Linear Regression)
    /// 指数回归线 (Exponential Regression)
    /// 对数回归线 (Logarithmic Regression)
    /// 多项式回归线 (Polynomial Regression)
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    order: Option<usize>,
}