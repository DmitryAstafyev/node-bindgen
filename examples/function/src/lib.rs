
use node_bindgen::derive::node_bindgen;


#[node_bindgen]
fn hello(count: i32) -> String {        
    format!("hello world {}", count)
}

