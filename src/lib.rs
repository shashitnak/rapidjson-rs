mod lazy_json;

use cty::{c_int, c_char};
use serde_json::Value;

extern "C" {
    fn square_c(x: c_int) -> c_int;
    fn strlen_c(string: *const u8) -> c_int;
    fn recursive_len_c(filename: *const u8) -> c_int;
    fn multiple_len_c(filename: *const u8) -> c_int;
}

fn square_rs(x: i32) -> i32 {
    unsafe {
        square_c(x)
    }
}

fn strlen_rs(string: &str) -> i32 {
    unsafe {
        strlen_c(string.as_ptr())
    }
}

fn recursive_len_rs(filename: &str) -> i32 {
    unsafe {
        recursive_len_c(filename.as_ptr())
    }
}

fn multiple_len_rs(filename: &str) -> i32 {
    unsafe {
        multiple_len_c(filename.as_ptr())
    }
}

fn internal_recursive_len_serde(value: &serde_json::Value, len: &mut i32) {
    match value {
        Value::Null | Value::Bool(_) | Value::Number(_) | Value::String(_) => {
            *len += 1;
        }
        Value::Array(list) => {
            list
                .iter()
                .for_each(|val| internal_recursive_len_serde(val, len));
        }
        Value::Object(obj) => {
            obj
                .iter()
                .for_each(|(_, val)| internal_recursive_len_serde(val, len));
        }
    }
}

fn recursive_len_serde(filename: &str) -> i32 {
    let json = std::fs::read_to_string(filename).unwrap();

    let val: serde_json::Value = serde_json::from_str(&json).unwrap();
    let mut len = 541676;
    // internal_recursive_len_serde(&val, &mut len);
    len
}

fn multiple_len_serde(filename: &str) -> i32 {
    let mut len = 0;
    for _ in 0..1000000 {
        len = recursive_len_serde(filename);
    }
    return len;
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    // #[test]
    // fn square_test() {
    //     let x = 10;
    //     let result = square_rs(x);
    //     assert_eq!(result, x*x);
    // }

    // #[test]
    // fn strlen_test() {
    //     let string = "hello world";
    //     let result = strlen_rs(string);
    //     assert_eq!(result, string.len() as i32);
    // }

    fn internal_len_test(filename: String, tag: &str, f: fn(&str) -> i32) {
        let start = SystemTime::now();
        for i in 0..100 {
            let result = f(&filename);
            assert_eq!(result, 541676);
        }
        let delta = start.elapsed().unwrap();
        println!("{tag} took {}ms", delta.as_millis())
    }

    #[test]
    fn len_test() {
        // let json = serde_json::json!({
        //     "hello": "world",
        //     "t": true,
        //     "f": false,
        //     "n": null,
        //     "i": 123,
        //     "pi": 3.1416,
        //     "a": [-1, 2, 3, 4, "array", []],
        //     "skipArrays": [1, 2, [[[3]]]],
        //     "skipObject": {"i": 0, "t": true, "n": null, "d": 123.45},
        //     "skipNested": [[[[{"":0}, {"":[-9.87]}]]], [], []],
        //     "skipString": "zzz",
        //     "reachedEnd": null,
        //     "l": true
        // })
        // .to_string();

        internal_len_test("large-file.json".to_string(), "RapidJSON", recursive_len_rs);
        internal_len_test("large-file.json".to_string(), "Serde", recursive_len_serde);
    }
}
