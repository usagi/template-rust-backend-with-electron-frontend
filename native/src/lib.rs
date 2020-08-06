use {{crate_name}}_backend as backend;
use node_bindgen::derive::node_bindgen;

#[node_bindgen]
fn example_add(a: f64, b: f64) -> f64 {
 backend::example_add(a as f32, b as f32) as f64
}

#[node_bindgen]
fn example_concat(a: String, b: String) -> String {
 backend::example_concat(&a, &b)
}
