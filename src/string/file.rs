use std::fs::File;
use std::io::{ErrorKind, Error, Write, Read};

/// 读取文件内容
// let match_result = file_get_contents(&"Cargo.toml");
//    match match_result {
//        Ok(is_match) => {
//            println!("{}", is_match);
//        },
//        Err(err) => {
//            eprintln!("{}", err)
//        }
//   }
pub fn file_get_contents(source_name: &str) -> Result<String, Error> {
    let str1 = source_name;
    let is_matched_url = str1.contains("http://") || str1.contains("https://");
    // 获取网页内容 todo
    if is_matched_url {
        return Ok("is matched url".to_string());
    }
    let file_name = source_name;
    // 获取文件内容
    match File::open(file_name) {
        Ok(mut f) => {
            let mut content = String::new();
            f.read_to_string(&mut content);
            return Ok(content);
        }
        Err(err) => {
            return Err(Error::new(ErrorKind::Other, "open file error"));
        }
    }
}
