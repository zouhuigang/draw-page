extern crate serde_json;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
use web_sys::console::log_1;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


fn log(s: &String) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
pub fn output_log(s: &str) {
    log(&format!("Hello {}", s));
}
#[derive(Serialize, Deserialize,Debug)]
struct ShieldFontData {
    #[serde(rename = "fontFile")]
    font_file: String,
    #[serde(rename = "canvasWidth")]
    canvas_width: i32,
    #[serde(rename = "canvasHeight")]
    canvas_height: i32,
    #[serde(rename = "canvasFontSize")]
    canvas_font_size: i32,
    #[serde(rename = "canvasFontColor")]
    canvas_font_color: String,
    #[serde(rename = "canvasClass")]
    canvas_class: String,
    font: String,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // https://docs.rs/web-dom/0.0.23/x86_64-apple-darwin/web_dom/document/fn.get_elements_by_class_name.html
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    // 获取 document.body let body = document.body().unwrap();
    let shield_fonts = document.get_elements_by_class_name("bx-shield-font");
    //取出所有.bx-shield-font的html内容
    for index in 0..shield_fonts.length() {
        let shield_font = shield_fonts.item(index).unwrap();
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        // 获取文本
       let user_text = shield_font.inner_html();
        let shield_font_data_str = shield_font.get_attribute("data-shield").unwrap();
        log(&shield_font_data_str);
        let shield_font_data:ShieldFontData = serde_json::from_str(&shield_font_data_str).expect("unable to deserialize JSON");

        canvas.set_inner_text("你的浏览器不支持canvas");
        shield_font.set_inner_html("");
        shield_font.append_child(&canvas)?;

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        canvas.set_width(shield_font_data.canvas_width as u32);
        canvas.set_height(shield_font_data.canvas_height as u32);
        //https://www.kancloud.cn/dennis/canvas/340115
        context.set_font(&shield_font_data.font);
        context.set_text_align("start");
        context.set_fill_style(&shield_font_data.canvas_font_color.into());
        context.fill_text( &*user_text,0.0,shield_font_data.canvas_font_size as f64).unwrap();
        //设置样式
        //canvas.style().set_property("border", "solid")?;
        //设置canvas class
        canvas.set_class_name(&*shield_font_data.canvas_class);
    }
    Ok(())
}
