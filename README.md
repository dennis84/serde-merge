# Serde Merge

A very simple merge function for serde_json.

```rust 
let mut a = json!({"a": 1});
let b = json!({"b": 2});

merge(&mut a, &b);
```
