use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/vendor_prefix.js")]
extern "C" {
    fn import_me();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(vendor_prefix = webkit)]
    type MySpecialApi;
    #[wasm_bindgen(constructor)]
    fn new() -> MySpecialApi;
    #[wasm_bindgen(method)]
    fn foo(this: &MySpecialApi) -> u32;

    #[wasm_bindgen(vendor_prefix = webkit)]
    type MySpecialApi2;
    #[wasm_bindgen(constructor)]
    fn new() -> MySpecialApi2;
    #[wasm_bindgen(method)]
    fn foo(this: &MySpecialApi2) -> u32;

    #[wasm_bindgen(vendor_prefix = a, vendor_prefix = b)]
    type MySpecialApi3;
    #[wasm_bindgen(constructor)]
    fn new() -> MySpecialApi3;
    #[wasm_bindgen(method)]
    fn foo(this: &MySpecialApi3) -> u32;
}

#[wasm_bindgen_test]
pub fn polyfill_works() {
    import_me();

    assert_eq!(MySpecialApi::new().foo(), 123);
    assert_eq!(MySpecialApi2::new().foo(), 124);
    assert_eq!(MySpecialApi3::new().foo(), 125);
}
