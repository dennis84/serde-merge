#[macro_use]
extern crate serde_json;

use serde_json::{Value, to_value};

pub fn merge(a: &mut Value, b: &Value) -> Value {
    if !a.is_object() || !b.is_object() {
        panic!("Only objects can be merged at the moment.");
    }

    let a = a.as_object_mut().unwrap();
    let b = b.as_object().unwrap();

    for (key, value) in b.iter() {
        if a.get(key).is_some() {
            let existing = a.get_mut(key).unwrap();
            if existing.is_object() && value.is_object() {
                merge(existing, value);
                continue;
            } else if existing.is_array() && value.is_array() {
                let array_a = existing.as_array_mut().unwrap();
                let array_b = value.as_array().unwrap();
                array_a.extend_from_slice(&array_b);
                continue;
            }
        }

        a.insert(key.to_string(), value.clone());
    }

    Value::Object(a.clone())
}

#[test]
fn test_merge_two_objects() {
    let mut a = json!({"a": {"x": 1}});
    let b = json!({"a": {"x": 2}});
    let c = json!({"a": {"x": 2}});
    assert_eq!(merge(&mut a, &b), c);
    assert_eq!(a, c);
}

#[test]
fn test_merge_two_objects_with_missing_key() {
    let mut a = json!({"a": {"x": 1, "y": 2}});
    let b = json!({"a": {"x": 2}});
    let c = json!({"a": {"x": 2, "y": 2}});
    assert_eq!(merge(&mut a, &b), c);
    assert_eq!(a, c);
}

#[test]
fn test_merge_two_objects_with_different_keys() {
    let mut a = json!({"a": {"x": 1, "y": {"z": 3}}});
    let b = json!({"a": {"x": 2, "y": {"v": 4}}});
    let c = json!({"a": {"x": 2, "y": {"z": 3, "v": 4}}});
    assert_eq!(merge(&mut a, &b), c);
    assert_eq!(a, c);
}

#[test]
fn test_merge_objects_in_a_list() {
    let mut a = json!({"a": [{"x": 1}]});
    let b = json!({"a": [{"x": 2}]});
    let c = json!({"a": [{"x": 1}, {"x": 2}]});
    assert_eq!(merge(&mut a, &b), c);
    assert_eq!(a, c);
}
