use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn announce(binding: &str) {
    alert(&format!("Loaded drand-wasm target for {}", binding));
}


#[cfg(test)]
mod tests {
    use crate::announce;

    #[test]
    #[should_panic(expected = "cannot call wasm-bindgen imported functions on non-wasm targets")]
    fn it_loads_bindings() {
        assert_eq!(announce("foo"), ());
    }
}
