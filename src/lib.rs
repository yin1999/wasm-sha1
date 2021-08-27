use sha1::Sha1;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    pub fn finalize(&mut self) -> String {
        self.hasher.digest().to_string()
    }
}
