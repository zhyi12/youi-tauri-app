use regex::Regex;
use youi_code::{gen_code,gen_tree_code};
use youi_test::find_real_path;
use askama::Template;

#[test]
pub fn gen_desktop_item_code(){
    code_gen("desktopItem","base");
}

#[test]
pub fn gen_icon_code(){
    code_gen("icon","base");
}

#[test]
pub fn gen_meta_item_code(){
    code_gen("metaItem","dmp");
}

#[test]
pub fn gen_macro_indicator_code(){
    tree_code_gen("macroIndicator","datamacro");
}

#[test]
pub fn gen_macro_group_item_code(){
    tree_code_gen("macroGroupItem","datamacro");
}

pub fn code_gen(name:&str,module_name:&str){
    //代码生成
    let path = find_real_path("code","");
    let reg = Regex::new("/youi-tauri-app/\\S+").unwrap();
    let app_src_dir = reg.replace(&path,"/youi-tauri-app/src").to_string();

    gen_code(&app_src_dir,&format!("{path}/entity/{name}.json"),module_name);

}
///
/// 树结构实体代码
///
pub fn tree_code_gen(name:&str,module_name:&str){
    //代码生成
    let path = find_real_path("code","");
    let reg = Regex::new("/youi-tauri-app/\\S+").unwrap();
    let app_src_dir = reg.replace(&path,"/youi-tauri-app/src").to_string();

    gen_tree_code(&app_src_dir,&format!("{path}/entity/{name}.json"),module_name);
}