use std::fs::File;
use std::io::Write;
use askama::Template;

use crate::entity::*;
use crate::dao::*;
use crate::services::*;
use crate::page::*;
use crate::store::*;

mod entity;
mod dao;
mod services;
mod store;
mod page;

///
/// 生成一般实体代码
///
pub fn gen_code(base_path:&str,model_path:&str,module_name:&str){

    let file = File::open(&model_path).unwrap();
    let model = serde_json::from_reader::<&File, EntityModel>(&file).unwrap();

    let name = model.name.as_str();
    let cname = model.cname.as_str();
    let caption = model.caption.as_str();
    let table_name = model.table_name.as_str();
    //
    gen_entity_code(base_path,module_name,&model.properties,name,cname,table_name);
    gen_dao_code(base_path,module_name,&model.properties,name,cname,table_name);
    gen_services_code(base_path,module_name,&model.properties,name,cname,caption);
    gen_store_code(base_path,module_name,&model.properties,name,cname,caption);
    gen_page_code(&format!("{}/routes/{}/{}",base_path,"tmp",model.name),module_name,&model.properties,name,cname,caption);
}

///
/// 生成树形实体代码
///
pub fn gen_tree_code(base_path:&str,model_path:&str,module_name:&str){
    let file = File::open(&model_path).unwrap();
    let model = serde_json::from_reader::<&File, EntityModel>(&file).unwrap();

    let name = model.name.as_str();
    let cname = model.cname.as_str();
    let caption = model.caption.as_str();
    let table_name = model.table_name.as_str();

    gen_tree_entity_code(base_path,module_name,&model.properties,name,cname,table_name);
    gen_tree_dao_code(base_path,module_name,&model.properties,name,cname,table_name);
    gen_tree_services_code(base_path,module_name,&model.properties,name,cname,caption);
    gen_tree_store_code(base_path,module_name,&model.properties,name,cname,caption);
    gen_tree_page_code(&format!("{}/routes/{}/{}",base_path,"tmp",model.name),module_name,&model.properties,name,cname,caption);

}

///
/// ts 实体
///
fn gen_entity_code(base_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,table_name:&str){
    let entity = EntityTemplate {
        name,
        cname,
        properties:properties.clone()
    };

    let sql = SqlTemplate {
        name,
        cname,
        table_name,
        properties:properties.iter().filter(|p|p.name != "id").cloned().collect::<Vec<Property>>()
    };

    let content:String = entity.render().unwrap();
    let sql_content:String = sql.render().unwrap();

    //sql脚本
    write_file(&format!("{}/lib/app-entity/{}/{}.d.ts",base_path,module_name,name),&content);
    write_file(&format!("{}/lib/app-entity/{}/{}.sql",base_path,module_name,name),&sql_content);
}

fn gen_tree_entity_code(base_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,table_name:&str){
    let entity = TreeEntityTemplate {
        name,
        cname,
        properties:properties.clone()
    };

    let sql = SqlTemplate {
        name,
        cname,
        table_name,
        properties:properties.iter().filter(|p|p.name != "id").cloned().collect::<Vec<Property>>()
    };

    let content:String = entity.render().unwrap();
    let sql_content:String = sql.render().unwrap();

    //sql脚本
    write_file(&format!("{}/lib/app-entity/{}/{}.d.ts",base_path,module_name,name),&content);
    write_file(&format!("{}/lib/app-entity/{}/{}.sql",base_path,module_name,name),&sql_content);
}
///
/// ts sql文件
///
fn gen_dao_code(base_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,table_name:&str){
    let dao = DaoTemplate {
        name,
        cname,
        table_name,
        properties:properties.clone()
    };

    let content:String = dao.render().unwrap();

    write_file(&format!("{}/lib/app-dao/{}/{}.sql.ts",base_path,module_name,name),&content);
}

fn gen_tree_dao_code(base_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,table_name:&str){
    let dao = TreeDaoTemplate {
        name,
        cname,
        table_name,
        properties:properties.clone()
    };

    let content:String = dao.render().unwrap();

    write_file(&format!("{}/lib/app-dao/{}/{}.sql.ts",base_path,module_name,name),&content);
}

///
/// ts services
///
fn gen_services_code(base_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,caption:&str){
    let services = ServicesTemplate {
        name,
        cname,
        module_name,
        caption,
        properties:properties.clone()
    };

    let content:String = services.render().unwrap();
    write_file(&format!("{}/lib/app-services/{}/{}Services.ts",base_path,module_name,name),&content);
}

fn gen_store_code(base_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,caption:&str){
    let services = StoreTemplate {
        name,
        cname,
        module_name,
        caption,
        properties:properties.clone()
    };

    let content:String = services.render().unwrap();

    write_file(&format!("{}/lib/app-stores/{}/{}Store.ts",base_path,module_name,name),&content);
}

fn gen_tree_services_code(base_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,caption:&str){
    let services = TreeServicesTemplate {
        name,
        cname,
        module_name,
        caption,
        properties:properties.clone()
    };

    let content:String = services.render().unwrap();

    write_file(&format!("{}/lib/app-services/{}/{}Services.ts",base_path,module_name,name),&content);
}

fn gen_tree_store_code(base_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,caption:&str){
    let services = TreeStoreTemplate {
        name,
        cname,
        module_name,
        caption,
        properties:properties.clone()
    };

    let content:String = services.render().unwrap();
    write_file(&format!("{}/lib/app-stores/{}/{}Store.ts",base_path,module_name,name),&content);
}
///
/// 生产CURD页面
///
fn gen_page_code(page_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,caption:&str){
    let page = PageTemplate {
        name,
        cname,
        module_name,
        caption,
        properties:properties.clone()
    };

    let form = FormTemplate {
        name,
        cname,
        module_name,
        caption,
        properties:properties.clone()
    };

    let content:String = page.render().unwrap();
    let form_content:String = form.render().unwrap();

    write_file(&format!("{}/+page.svelte",page_path),&content);
    write_file(&format!("{}/EditForm.svelte",page_path),&form_content);
}

///
/// 树页面
///
fn gen_tree_page_code(page_path:&str,module_name:&str,properties:&Vec<Property>,name:&str,cname:&str,caption:&str){

    let tree_page_layout = TreePageTemplate {name, cname, module_name, caption, properties:properties.clone()};
    let tree_page_layout_ts = TreePageTsTemplate{name, cname, module_name, caption, properties:properties.clone()};
    let tree_page_path = TreePagePathTemplate{name, cname, module_name, caption, properties:properties.clone()};
    let tree_page_path_ts = TreePagePathTsTemplate{name, cname, module_name, caption, properties:properties.clone()};

    let content_tree_page_layout:String = tree_page_layout.render().unwrap();
    let content_tree_page_layout_ts:String = tree_page_layout_ts.render().unwrap();
    let content_tree_page_path:String = tree_page_path.render().unwrap();
    let content_tree_page_path_ts:String = tree_page_path_ts.render().unwrap();

    //树主页面
    write_file(&format!("{}/+layout@.svelte",page_path),&content_tree_page_layout);
    write_file(&format!("{}/+layout.ts",page_path),&content_tree_page_layout_ts);
    write_file(&format!("{}/+page.svelte",page_path),"");
    //节点信息页
    write_file(&format!("{}/[...path]/+page.svelte",page_path),&content_tree_page_path);
    write_file(&format!("{}/[...path]/+page.ts",page_path),&content_tree_page_path_ts);

}
///
///
///
fn write_file(file_path:&str,content:&str){
    //自动创建文件夹
    let mut parts:Vec<&str> = file_path.split("/").collect();
    parts.pop();
    let dir = parts.join("/");
    std::fs::create_dir_all(dir).unwrap();

    if std::fs::metadata(file_path).is_err(){
        let mut file = File::create(file_path).unwrap();
        file.write(content.as_bytes()).unwrap();
    }

}