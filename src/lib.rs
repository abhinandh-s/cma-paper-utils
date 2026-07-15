use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = paperIdToLevel)]
pub fn paper_id_to_level(id: &str) -> String {
    match id {
        "p1" | "p2" | "p3" | "p4" => "foundation".to_string(),
        "p5" | "p6" | "p7" | "p8" | "p9" | "p10" | "p11" | "p12" => "intermediate".to_string(),
        "p13" | "p14" | "p15" | "p16" | "p17" | "p18" | "p19" | "p20" | "p20A" | "p20B" | "p20C" => "final".to_string(),
        _ => "unknown".to_string(),
    }
}

