//! Try to lock from workers

use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;

lazy_static!{
    //static ref MY_NUM: tokio::sync::Mutex<u32> = tokio::sync::Mutex::new(4); // async mutex
    static ref MY_NUM: std::sync::Mutex<u32> = std::sync::Mutex::new(12); // std sync Mutex, also works
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(value: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    //let mut num = MY_NUM.lock().await;
    //tokio::time::sleep().await;
    log(&format!("Hello, {}! thread_id={:?} waiting for lock..", name, std::thread::current().id()));
    let mut num = MY_NUM.lock().unwrap();
    let mut f = 1.111;
    for i in 0..100000000 { 
        f = 1.1111 / 7.77777 * f; 
        if i % 10000000 == 0 {
            log(&format!("Hello, {}! doing work..", name));
        }
    } // do some work
    *num = *num + 1;
    //let num = 0;
    log(&format!("Hello, {}! num={}", name, num));
}
