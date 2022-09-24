use std::fmt::write;

use sha1::{Sha1, Digest};
use wasm_bindgen::{prelude::*, throw_str};
use js_sys::Uint8Array;
use js_sys::Array;

#[wasm_bindgen]
pub struct Sha1Digest {
    hasher: Sha1,
}

#[wasm_bindgen]
impl Sha1Digest {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Sha1Digest {
        let hasher = Sha1::new();

        Sha1Digest { hasher: hasher }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }
    
    pub fn finalize(self) -> String {
        let out = self.hasher.finalize();
        let mut s = String::new();
        for &byte in out.iter() {
            match write(&mut s, format_args!("{:02x}", byte)) {
                Err(e) => {
                    throw_str(&e.to_string());
                },
                _ => (),
            }
        }
        s
    }

    pub fn finalize_bytes(self) -> Uint8Array {
        let ans = self.hasher.finalize();
        let array: Array = ans.iter().map(|&x| JsValue::from(x as u8)).collect();
        Uint8Array::new(&array)
    }
}
