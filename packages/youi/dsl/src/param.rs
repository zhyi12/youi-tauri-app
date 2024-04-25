use polars_core::export::regex::{Captures, Regex};
use crate::Param;

impl Param {
    pub fn same_param(&self,name:&str)->bool{
        name == self.name.as_str()
    }

    pub fn get_value(&self)->String{
        self.value.clone()
    }
}

impl Param {
    pub fn new(name:&str,value:&str,data_type:&str)->Self{
        Param{
            name:name.to_string(),
            value:value.to_string(),
            data_type:data_type.to_string()
        }
    }
}

///
/// ${paramName}格式参数替换
///
pub fn transform_param(script:&str,params:&Vec<Param>)->String{
    let regex = Regex::new(r"\$\{(?<param>\w+)\}").unwrap();
    let replacement = |caps: &Captures| -> Result<String, &'static str> {
        let param_name = caps.name("param").unwrap().as_str().to_string();
        let opt_param = params.iter().find(|p|p.same_param(&param_name));
        Ok(match opt_param {
            None => "".to_string(),
            Some(param) => param.get_value()
        })
    };
    replace_all(&regex,script,&replacement).unwrap()
}
///
/// 参数变量替换
///
fn replace_all<E>(
    re: &Regex,
    haystack: &str,
    replacement: impl Fn(&Captures) -> Result<String, E>
) -> Result<String, E> {
    let mut new = String::with_capacity(haystack.len());
    let mut last_match = 0;
    for caps in re.captures_iter(haystack) {
        let m = caps.get(0).unwrap();
        new.push_str(&haystack[last_match..m.start()]);
        new.push_str(&replacement(&caps)?);
        last_match = m.end();
    }
    new.push_str(&haystack[last_match..]);
    Ok(new)
}

