use std::fs::File;
use log::debug;

///
/// 启用测试日志
///
pub fn enable_test_log(){

    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        // Apply globally
        .apply().unwrap()

}

///
/// 反序列化
///
pub fn read_from_json<T: for<'de> serde::Deserialize<'de>>(module:&str,path:&str)->T{
    let file_path = find_real_path(module,path);
    debug!("load json {file_path}");
    let file = open_file(&file_path);
    serde_json::from_reader::<&File, T>(&file).unwrap()
}

///
///
///
pub fn find_real_path(module:&str,path:&str)->String{
    format!("{}/../{}/tests/data/{}",env!("CARGO_MANIFEST_DIR"),module,path)
}

///
///
///
pub fn open_file(file_path:&str) -> File{
    File::open(&file_path).unwrap()
}