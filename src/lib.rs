use js_sys::Function;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::{from_value, to_value};

#[wasm_bindgen]
pub fn sort_numbers(numbers: &JsValue, ascending: bool) -> JsValue {
    let mut nums: Vec<f64> = from_value(numbers.clone()).unwrap_or_default();
    if ascending {
        nums.sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    } else {
        nums.sort_by(|a: &f64, b: &f64| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    }
    to_value(&nums).unwrap()
}

#[wasm_bindgen]
pub fn sort_strings(strings: &JsValue, ascending: bool) -> JsValue {
    let mut strs: Vec<String> = from_value(strings.clone()).unwrap_or_default();
    if ascending {
        strs.sort();
    } else {
        strs.sort_by(|a: &String, b: &String| b.cmp(a));
    }
    to_value(&strs).unwrap()
}

#[wasm_bindgen]
pub fn sort_objects_by_column(objects: &JsValue, column: &str, ascending: bool) -> JsValue {
    let mut objs: Vec<Value> = from_value(objects.clone()).unwrap_or_default();
    objs.sort_by(|a: &Value, b: &Value| {
        let va = a.get(column);
        let vb = b.get(column);
        match (va, vb) {
            (Some(va), Some(vb)) => {
                if let (Some(na), Some(nb)) = (va.as_f64(), vb.as_f64()) {
                    if ascending {
                        na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal)
                    } else {
                        nb.partial_cmp(&na).unwrap_or(std::cmp::Ordering::Equal)
                    }
                } else if let (Some(sa), Some(sb)) = (va.as_str(), vb.as_str()) {
                    if ascending {
                        sa.cmp(sb)
                    } else {
                        sb.cmp(sa)
                    }
                } else {
                    std::cmp::Ordering::Equal
                }
            }
            _ => std::cmp::Ordering::Equal,
        }
    });
    to_value(&objs).unwrap()
}

#[wasm_bindgen]
pub fn sort_objects_by_key_fn(objects: &JsValue, key_fn: &Function, ascending: bool) -> JsValue {
    let objs: Vec<Value> = from_value(objects.clone()).unwrap_or_default();
    let mut objs = objs;
    objs.sort_by(|a: &Value, b: &Value| {
        let this = JsValue::NULL;
        let va = key_fn.call1(&this, &to_value(a).unwrap()).unwrap_or(JsValue::UNDEFINED);
        let vb = key_fn.call1(&this, &to_value(b).unwrap()).unwrap_or(JsValue::UNDEFINED);
        // Try to compare as numbers first
        let na = va.as_f64();
        let nb = vb.as_f64();
        if let (Some(na), Some(nb)) = (na, nb) {
            if ascending {
                na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal)
            } else {
                nb.partial_cmp(&na).unwrap_or(std::cmp::Ordering::Equal)
            }
        } else {
            let sa = va.as_string();
            let sb = vb.as_string();
            if let (Some(sa), Some(sb)) = (sa, sb) {
                if ascending {
                    sa.cmp(&sb)
                } else {
                    sb.cmp(&sa)
                }
            } else {
                std::cmp::Ordering::Equal
            }
        }
    });
    to_value(&objs).unwrap()
}

// For advanced dynamic sorting, you can add more functions or accept custom comparator logic via JS callback.
