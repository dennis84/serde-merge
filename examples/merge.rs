#[macro_use]
extern crate serde_json;
extern crate serde_merge;

use serde_merge::merge;

fn main() {
    let mut a = json!({"a": 1});
    let b = json!({"b": 2});
    println!("{}", merge(&mut a, &b));

    let mut a = json!({"a": {"b": [1, 2]}});
    let b = json!({"a": {"b": [3, 4], "c": 1}});
    println!("{}", merge(&mut a, &b));
}
