#![cfg_attr(feature = "no_std", no_std)]

#[allow(dead_code)]

pub mod json_parser{
    #[cfg(not(feature = "no_std"))]    
    use std::collections::BTreeMap;
    #[cfg(not(feature = "no_std"))]    
    use std::vec::Vec;
    #[cfg(not(feature = "no_std"))]
    use std::string::String;
    #[cfg(not(feature = "no_std"))]
    use std::ops::Index;

    #[cfg(feature = "no_std")]
    extern crate alloc;
    #[cfg(feature = "no_std")]
    use alloc::string::String;
    #[cfg(feature = "no_std")]
    use alloc::vec::Vec;
    #[cfg(feature = "no_std")]
    use alloc::collections::BTreeMap;
    #[cfg(feature = "no_std")]
    use core::ops::Index;
    pub enum Jobject {
        Jmap(Jmap),
        Jvec(Jvec),
        Jval(Jval),
    }

    pub struct Jmap(BTreeMap<String, Jobject>);
    impl Index<String> for Jmap {
        type Output = Jobject;
        fn index(&self, key: String) ->&Jobject {
            &self.0[&key]
        }
    }
    pub struct Jvec(Vec<Jobject>);
    impl Index<usize> for Jvec {
        type Output = Jobject;
        fn index(&self, i: usize) -> &Jobject {
            &self.0[i]
        }
    }
    pub struct Jpair(String, Jobject);
    pub enum Jval {
        String(String),
        Number(f64),
        Bool(bool),
        Null
    }

    fn is_blank_char(c: char) -> bool {
        let blank_chars = [' ', '\t', '\n', '\r'];
        let mut flag = false;
        for bc in blank_chars.iter() {
            if *bc == c {
                flag = true;
                break;
            }
        }
        flag
    }

    pub fn parse(js: &mut String) -> Jobject {
        let idx: &mut usize = &mut 0;
        let json_str = js.chars().collect();
        return parse_object(&json_str, idx);
    }

    fn parse_object(json_str: &Vec<char>, idx: &mut usize) -> Jobject {
        while is_blank_char(json_str[*idx]) {
            *idx += 1;
        }
        match json_str[*idx] {
            '{' => Jobject::Jmap(parse_map(json_str, idx)),
            '[' => Jobject::Jvec(parse_vec(json_str, idx)),
            _ => Jobject::Jval(parse_val(json_str, idx))
        }
    }
    
    fn parse_map(json_str: &Vec<char>, idx: &mut usize) -> Jmap {
        let mut jpairs = BTreeMap::<String, Jobject>::new();
        if json_str[*idx] == '{' {
            *idx += 1;
        }
        else {
            panic!("需要一个'{'")
        }
        loop {
            while is_blank_char(json_str[*idx]) {
                if *idx >= json_str.len() {
                    panic!("需要一'}'")
                }
                *idx += 1;
            }

            if json_str[*idx] == '}' {
                break;
            }

            let jpair = parse_pair(json_str, idx);

            jpairs.insert(jpair.0, jpair.1);
            while is_blank_char(json_str[*idx]) {
                *idx += 1;
            }
            if json_str[*idx] == ',' {
                *idx += 1;
            }
            else {
                if json_str[*idx] == '}' {
                    continue;
                }
                panic!("需要一个','")
            }
        }
        *idx += 1;
        let jmap = Jmap (jpairs);
        jmap
    }
    #[test]
    fn test_map() {
        let raw_json_map = "{\"aaa\":12345, \"bbb\": false, \"ccc\": \"ssss\"}";
        let json_map = raw_json_map.chars().collect();
        let idx = &mut 0;
        let map = parse_map(&json_map, idx.into());
        let pairs = map.0;
        if let Jobject::Jval(Jval::Number(num)) = &pairs["aaa"] {
            assert_eq!(num, &12345.0);
        }
        if let Jobject::Jval(Jval::Bool(b)) = &pairs["bbb"] {
            assert_eq!(b, &false)
        }
        if let Jobject::Jval(Jval::String(s)) = &pairs["ccc"] {
            assert_eq!(s, "ssss")
        }
        assert_eq!(*idx, raw_json_map.len());
    }

    fn parse_pair(json_str: &Vec<char>, idx: &mut usize) -> Jpair {
        let jk = parse_string(json_str, idx);
        if json_str[*idx] != ':' {
            panic!("需要一个':'")
        }
        else {
            *idx += 1;
        }
        let jv = parse_object(json_str, idx);

        let jpair = Jpair (jk, jv);
        jpair
    }

    #[test]
    fn test_pair() {
        // let raw_json_pair = "\"asdf\":12345";
        // let json_pair = raw_json_pair.chars().collect();
        // let idx = &mut 0;
        // let pair = parse_pair(&json_pair, idx.into());
        // assert_eq!(pair.0, "asdf");
        // if let Jobject::Jval(Jval::Number(num)) = pair.1 {
        //     assert_eq!(num, 12345.0)
        // }
        // assert_eq!(*idx, raw_json_pair.len());

        // let raw_json_pair = "\"asdf\":123.45";
        // let json_pair = raw_json_pair.chars().collect();
        // let idx = &mut 0;
        // let pair = parse_pair(&json_pair, idx.into());
        // assert_eq!(pair.0, "asdf");
        // if let Jobject::Jval(Jval::Number(num)) = pair.1 {
        //     assert_eq!(num, 123.45)
        // }
        // assert_eq!(*idx, raw_json_pair.len());

        // let raw_json_pair = "\"asdf\": false";
        // let json_pair = raw_json_pair.chars().collect();
        // let idx = &mut 0;
        // let pair = parse_pair(&json_pair, idx.into());
        // assert_eq!(pair.0, "asdf");
        // if let Jobject::Jval(Jval::Bool(b)) = pair.1 {
        //     assert_eq!(b, false)
        // }
        // assert_eq!(*idx, raw_json_pair.len());
    }

    fn parse_vec(json_str: &Vec<char>, idx: &mut usize) -> Jvec {
        let mut jobjects = Vec::<Jobject>::new();
        if json_str[*idx] == '[' {
            *idx += 1;
        }
        else {
            panic!("需要一个'['")
        }
        loop {
            while is_blank_char(json_str[*idx]) {
                if *idx >= json_str.len() {
                    panic!("需要一']'")
                }
                *idx += 1;
            }
            if json_str[*idx] == ']' {
                *idx += 1;
                break;
            }
            let obj = parse_object(json_str, idx);
            jobjects.push(obj);
            while is_blank_char(json_str[*idx]) {
                *idx += 1;
            }
            if json_str[*idx] == ',' {
                *idx += 1;
            }
            else {
                if json_str[*idx] == ']' {
                    *idx += 1;
                    break;
                }
                panic!("需要一','")
            }
        }
        let jvec = Jvec (jobjects);
        jvec
    }

    #[test]
    fn test_vec() {
        let raw_json_vec = "[12345, \"asdf\", true, false, null, 123.45]";
        let json_vec = raw_json_vec.chars().collect();
        let idx = &mut 0;
        let vec = parse_vec(&json_vec, idx.into());
        let jobjects = vec.0;
        if let Jobject::Jval(Jval::Number(num)) = &jobjects[0] {
            assert_eq!(num, &12345.0)
        }
        else {
            panic!("类型错误，应为number")
        }
        if let Jobject::Jval(Jval::String(s)) = &jobjects[1] {
            assert_eq!(s, "asdf")
        }
        else {
            panic!("类型错误，应为string")
        }
        if let Jobject::Jval(Jval::Bool(b)) = &jobjects[2] {
            assert_eq!(b, &true)
        }
        else {
            panic!("类型错误，应为bool")
        }
        if let Jobject::Jval(Jval::Bool(b)) = &jobjects[3] {
            assert_eq!(b, &false)
        }
        else {
            panic!("类型错误，应为bool")
        }
        if let Jobject::Jval(Jval::Null) = &jobjects[4] {
        }
        else {
            panic!("类型错误，应为null")
        }
        if let Jobject::Jval(Jval::Number(num)) = &jobjects[5] {
            assert_eq!(num, &123.45)
        }
        else {
            panic!("类型错误，应为number")
        }
        assert_eq!(*idx, raw_json_vec.len());
    }


    fn parse_val(json_str: &Vec<char>, idx: &mut usize) -> Jval {
        if json_str[*idx] == '"' {
            return Jval::String(parse_string(json_str, idx));
        }

        if *idx+4 <= json_str.len() {
            let null_str: Vec<char> = "null".chars().collect();
            let mut flag = true;
            for i in 0..4 {
                if json_str[i+*idx] != null_str[i] {
                    flag = false;
                    break;
                } 
            }
            if flag {
                *idx += 4;
                return Jval::Null;
            }
        }

        if *idx+4 <= json_str.len() {
            let true_str: Vec<char> = "true".chars().collect();
            let mut flag = true;
            for i in 0..4 {
                if json_str[i+*idx] != true_str[i] {
                    flag = false;
                    break;
                } 
            }
            if flag {
                *idx += 4;
                return Jval::Bool(true);
            }
        }

        if *idx+5 <= json_str.len() {
            let false_str: Vec<char> = "false".chars().collect();
            let mut flag = true;
            for i in 0..4 {
                if json_str[i+*idx] != false_str[i] {
                    flag = false;
                    break;
                } 
            }
            if flag {
                *idx += 5;
                return Jval::Bool(false);
            }
        }
        Jval::Number(parse_number(json_str, idx))
    }

    #[test]
    fn test_val() {

        // let raw_json_val = "\"adf12sdf1s31fsdf\"";
        // let json_val = raw_json_val.chars().collect();
        // let idx = &mut 0;
        // let val = parse_val(&json_val, idx.into());
        // if let Jval::String(s) = val {
        //     assert_eq!(s, &raw_json_val[1..raw_json_val.len()-1])
        // }
        // else {
        //     panic!("类型错误，应为string")
        // }
        // assert_eq!(*idx, raw_json_val.len());

        // let raw_json_val = "null";
        // let json_val = raw_json_val.chars().collect();
        // let idx = &mut 0;
        // let val = parse_val(&json_val, idx.into());
        // if let Jval::Null = val { }
        // else {
        //     panic!("类型错误，应为Null")
        // }
        // assert_eq!(*idx, raw_json_val.len());

        // let raw_json_val = "true";
        // let json_val = raw_json_val.chars().collect();
        // let idx = &mut 0;
        // let val = parse_val(&json_val, idx.into());
        // if let Jval::Bool(b) = val {
        //     assert_eq!(b, true);
        // }
        // else {
        //     panic!("类型错误，应为bool")
        // }
        // assert_eq!(*idx, raw_json_val.len());

        // let raw_json_val = "false";
        // let json_val = raw_json_val.chars().collect();
        // let idx = &mut 0;
        // let val = parse_val(&json_val, idx.into());
        // if let Jval::Bool(b) = val {
        //     assert_eq!(b, false);
        // }
        // else {
        //     panic!("类型错误，应为bool")
        // }
        // assert_eq!(*idx, raw_json_val.len());

        // let raw_json_val = "12345";
        // let json_val = raw_json_val.chars().collect();
        // let idx = &mut 0;
        // let val = parse_val(&json_val, idx.into());
        // if let Jval::Number(num) = val {
        //     assert_eq!(num, 12345.0);
        // }
        // else {
        //     panic!("类型错误，应为number")
        // }
        // assert_eq!(*idx, raw_json_val.len());

        // let raw_json_val = "123.45";
        // let json_val = raw_json_val.chars().collect();
        // let idx = &mut 0;
        // let val = parse_val(&json_val, idx.into());
        // if let Jval::Number(num) = val {
        //     assert_eq!(num, 123.45);
        // }
        // else {
        //     panic!("类型错误，应为number")
        // }
        // assert_eq!(*idx, raw_json_val.len());
    }

    fn parse_string(json_str: &Vec<char>, idx: &mut usize) -> String {
        let mut str = String::new();
        if json_str[*idx] != '"' {
            panic!("需要一个左'\"'")
        }
        let mut end_idx = *idx + 1;
        let mut flag = false;
        while end_idx < json_str.len() {
            if json_str[end_idx] == '"' {
                flag = true;
                break
            }
            end_idx += 1;
        }
        if !flag {
            panic!("需要一个右'\"'")
        }
        for i in *idx+1..end_idx {
            str.push(json_str[i]);
        }
        *idx = end_idx + 1;
        str
    }

    #[test]
    fn test_string() {

        // let raw_json_string = "\"adf12sdf1s31fsdf\"";
        // let json_string = raw_json_string.chars().collect();
        // let idx = &mut 0;
        // let string = parse_string(&json_string, idx.into());
        // assert_eq!(string, &raw_json_string[1..raw_json_string.len()-1]);
        // assert_eq!(*idx, raw_json_string.len());

    }


    fn parse_number(json_str: &Vec<char>, idx: &mut usize) -> f64 {
        let beg = *idx;
        let mut point_pos = json_str.len() + 1;
        let mut end = *idx;
        while json_str[end] == '.' || (json_str[end] <= '9' && json_str[end] >= '0') {
            if json_str[end] == '.' {
                if point_pos != json_str.len() + 1 {
                    panic!("数字中出现多个'.'")
                }
                else {
                    point_pos = end;
                }
            }
            end += 1;
            if end == json_str.len() {
                break;
            }
            else if end > json_str.len() {
                panic!("字符串数组越界!")
            }
        }
        let mut num = 0.0;
        if point_pos != json_str.len() + 1 {
            let mut int_part = 0.0;
            for i in beg..point_pos {
                int_part *= 10.0;
                int_part += (json_str[i] as i32 - '0' as i32) as f64;
            }
            let mut frac_part = 0.0;
            for i in (point_pos+1..end).rev() {
                frac_part += (json_str[i] as i32 - '0' as i32) as f64;
                frac_part /= 10.0;
            }
            num = int_part + frac_part;
        }
        else {
            for i in beg..end {

                num *= 10.0;
                num += (json_str[i] as i32 - '0' as i32) as f64;
            }
        }
        *idx = end;
        num
    }
    
    #[test]
    fn test_number() {
        
        // let json_number = "12345".chars().collect();
        // let idx = &mut 0;
        // let number = parse_number(&json_number, idx.into());
        // assert_eq!(number, 12345.0);
        // assert_eq!(*idx, 5);

        // let json_number = "123.45".chars().collect();
        // let idx = &mut 0;
        // let number = parse_number(&json_number, idx.into());
        // assert_eq!(number, 123.45);
        // assert_eq!(*idx, 6);
        
        // let json_number = "12;3.45".chars().collect();
        // let idx = &mut 0;
        // let number = parse_number(&json_number, idx.into());
        // assert_eq!(number, 12.0);
        // assert_eq!(*idx, 2 as usize);
    }


}