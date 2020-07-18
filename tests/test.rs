#[cfg(not(feature="no_std"))]
extern crate simple_json_parser;

use simple_json_parser::json_parser::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


#[test]
fn test_parser() {
    let path = Path::new("./jsonfile/test1.json");
    let display = path.display();
    let mut json_file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };    
    let mut json_str = String::new();
    match json_file.read_to_string(&mut json_str) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, json_str),
    }
    let json_obj = parse(&mut json_str);
    if let Jobject::Jmap(jmap) = json_obj {
        let account_name = &jmap[String::from("accountName")];
        if let Jobject::Jval(Jval::String(account_name_val)) = account_name {
            assert_eq!(account_name_val, "11111111111111111111111111111111111111111111111111")
        }
        else {
            panic!("类型错误，应为string")
        }
        let start_date = &jmap[String::from("startDate")];
        if let Jobject::Jval(Jval::Bool(start_date_val)) = start_date {
            assert_eq!(start_date_val, &true)
        }
        else {
            panic!("类型错误，应为bool")
        }
    }
    else {
        panic!("应该是Jmap类型")
    }
}