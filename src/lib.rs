use models::Commons::Coodinate;
use models::Mino::T;
// use wasm_bindgen::{Clamped, JsCast};
// use web_sys::ImageData;
// use models::Mino::getMino;
// use models::Mino::Mino;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/**
 * for demo_test_01
 */
use std::cell::Cell;
use std::rc::Rc;

mod models;
mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t) *).to_string()));
}

/**
 * demo from https://rustwasm.github.io/docs/wasm-bindgen/examples/paint.html
 */
#[wasm_bindgen(start)]
pub fn demo_test_01() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            context.begin_path();
            context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
                context.line_to(event.offset_x() as f64, event.offset_y() as f64);
                context.stroke();
                context.begin_path();
                context.move_to(event.offset_x() as f64, event.offset_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = context.clone();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
            context.line_to(event.offset_x() as f64, event.offset_y() as f64);
            context.stroke();
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

/**
 * demo from https://rustwasm.github.io/wasm-bindgen/examples/2d-canvas.html
 */
#[wasm_bindgen]
pub fn demo_test_02() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>() //式中のジェネリックな型、関数、メソッドへの引数を指定
        .map_err(|_| ()) //https://doc.rust-jp.rs/book-ja/ch13-01-closures.html
        .unwrap(); //毎回unwrapって何してるん？

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::canvasrenderingcontext2d>()
        .unwrap();

    context.begin_path();

    // draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::pi * 2.0)
        .unwrap();

    // draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::pi).unwrap();

    // draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::pi * 1.0)
        .unwrap();

    // draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::pi * 2.0)
        .unwrap();

    context.stroke();

    Ok(())
}

pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(640);
    canvas.set_height(480);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    OK(())
}

#[wasm_bindgen]
pub fn test() {
    println!("test!");
}

// #[wasm_bindgen]
// pub fn named_struct_by_value(x: &Mino) {}

// #[wasm_bindgen]
// pub fn return_named_struct() -> Mino {
//     return T;
// }
