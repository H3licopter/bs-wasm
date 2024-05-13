use wasm_bindgen::prelude::*;
use math_parse::MathParse;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn parse_int_expr(input: &str) -> i64 {
    let parsed = MathParse::parse(input);
    match parsed {
        Ok(x) => {
            match x.solve_int(None) {
                Ok(result) => return result,
                Err(e) =>  {
                    log(&format!("Error: {:?}", e));
                }
            }
        },
        Err(e) => {
            log(&format!("Error: {:?}", e));
        }
    }

    // Zero value to return if error
    return 0
}

#[wasm_bindgen]
pub fn parse_float_expr(input: &str) -> f64 {
    let parsed = MathParse::parse(input);
    match parsed {
        Ok(x) => {
            match x.solve_float(None) {
                Ok(result) => return result,
                Err(e) =>  {
                    log(&format!("Error: {:?}", e));
                }
            }
        },
        Err(e) => {
            log(&format!("Error: {:?}", e));
        }
    }

    // Zero value to return if error
    return 0.0
}