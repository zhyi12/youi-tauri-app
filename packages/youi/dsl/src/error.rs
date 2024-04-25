
use rhai::EvalAltResult;
use thiserror::Error;

pub type DslResult<T> = Result<T,DslError>;

#[derive(Error, Debug)]
pub enum DslError {

    #[error("脚本解析异常:{0}.")]
    DslParseError(String),

    #[error(transparent)]
    DataframeError(#[from] youi_dataframe::error::DataframeError),

    #[error("第{1}行,{2}列:{0}.")]
    EvalError(String,usize,usize),

    #[error("ErrorSystem {0}")]
    ErrorSystem(String),

    #[error("UnknownEvalError")]
    UnknownEvalError

}
///
///
///
pub fn to_dsl_error(e:&EvalAltResult)->DslError{
    match e {
        EvalAltResult::ErrorSystem(s,..) => DslError::ErrorSystem(s.to_string()),
        EvalAltResult::ErrorParsing(_, p) =>
            DslError::EvalError("DSL解析异常".to_string(),p.line().unwrap(),p.position().unwrap()),
        EvalAltResult::ErrorVariableExists(x, p) =>
            DslError::EvalError(format!("{}已经存在",x),p.line().unwrap(),p.position().unwrap()),
        EvalAltResult::ErrorFunctionNotFound(x, p) =>
            DslError::EvalError(format!("函数{}未找到",x),p.line().unwrap(),p.position().unwrap()),
        EvalAltResult::ErrorVariableNotFound(x, p) =>
            DslError::EvalError(format!("变量{}未找到",x),p.line().unwrap(),p.position().unwrap()),
        EvalAltResult::ErrorRuntime(x, p) =>
            DslError::EvalError(format!("ErrorRuntime {:?}",x),p.line().unwrap(),p.position().unwrap()),
        _=>DslError::UnknownEvalError
        // EvalAltResult::ErrorForbiddenVariable(_, _) => {}
        // EvalAltResult::ErrorVariableNotFound(_, _) => {}
        // EvalAltResult::ErrorPropertyNotFound(_, _) => {}
        // EvalAltResult::ErrorIndexNotFound(_, _) => {}
        //
        // EvalAltResult::ErrorModuleNotFound(_, _) => {}
        // EvalAltResult::ErrorInFunctionCall(_, _, _, _) => {}
        // EvalAltResult::ErrorInModule(_, _, _) => {}
        // EvalAltResult::ErrorUnboundThis(_) => {}
        // EvalAltResult::ErrorMismatchDataType(_, _, _) => {}
        // EvalAltResult::ErrorMismatchOutputType(_, _, _) => {}
        // EvalAltResult::ErrorIndexingType(_, _) => {}
        // EvalAltResult::ErrorArrayBounds(_, _, _) => {}
        // EvalAltResult::ErrorStringBounds(_, _, _) => {}
        // EvalAltResult::ErrorBitFieldBounds(_, _, _) => {}
        // EvalAltResult::ErrorFor(_) => {}
        // EvalAltResult::ErrorDataRace(_, _) => {}
        // EvalAltResult::ErrorNonPureMethodCallOnConstant(_, _) => {}
        // EvalAltResult::ErrorAssignmentToConstant(_, _) => {}
        // EvalAltResult::ErrorDotExpr(_, _) => {}
        // EvalAltResult::ErrorArithmetic(_, _) => {}
        // EvalAltResult::ErrorTooManyOperations(_) => {}
        // EvalAltResult::ErrorTooManyModules(_) => {}
        // EvalAltResult::ErrorStackOverflow(_) => {}
        // EvalAltResult::ErrorDataTooLarge(_, _) => {}
        // EvalAltResult::ErrorTerminated(_, _) => {}
        // EvalAltResult::ErrorCustomSyntax(_, _, _) => {}
        //
        // EvalAltResult::LoopBreak(_, _, _) => {}
        // EvalAltResult::Return(_, _) => {}
    }
}