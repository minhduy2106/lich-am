//! JS bindings for `lunar.js`: generated from the signatures in `port.rs` by rust-anything.
//! Regenerate with `rust-anything refresh` after changing them.

use wasm_bindgen::prelude::*;

pub mod js;
pub mod port;
pub mod wire;

#[allow(unused_imports)]
use serde_json::Value;
pub mod utf16;

/// `jdFromDate`.
fn jd_from_date_inner(dd: String, mm: String, yy: String) -> Result<String, String> {
    let dd = crate::js::from_json(&dd)?;
    let mm = crate::js::from_json(&mm)?;
    let yy = crate::js::from_json(&yy)?;
    let out = port::jd_from_date(dd, mm, yy)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "jdFromDate")]
pub fn jd_from_date(dd: String, mm: String, yy: String) -> Result<String, JsError> {
    jd_from_date_inner(dd, mm, yy).map_err(|e| JsError::new(&e))
}

/// `jdToDate`.
fn jd_to_date_inner(jd: String) -> Result<String, String> {
    let jd = crate::js::from_json(&jd)?;
    let out = port::jd_to_date(jd)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "jdToDate")]
pub fn jd_to_date(jd: String) -> Result<String, JsError> {
    jd_to_date_inner(jd).map_err(|e| JsError::new(&e))
}

/// `convertSolar2Lunar`.
fn convert_solar2_lunar_inner(
    dd: String,
    mm: String,
    yy: String,
    tz: String,
) -> Result<String, String> {
    let dd = crate::js::from_json(&dd)?;
    let mm = crate::js::from_json(&mm)?;
    let yy = crate::js::from_json(&yy)?;
    let tz = crate::js::from_json(&tz)?;
    let out = port::convert_solar2_lunar(dd, mm, yy, tz)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "convertSolar2Lunar")]
pub fn convert_solar2_lunar(
    dd: String,
    mm: String,
    yy: String,
    tz: String,
) -> Result<String, JsError> {
    convert_solar2_lunar_inner(dd, mm, yy, tz).map_err(|e| JsError::new(&e))
}

/// `buildEvents`.
fn build_events_inner(
    from_jd: String,
    days: String,
    gios: String,
    opts: String,
) -> Result<String, String> {
    let from_jd = crate::js::from_json(&from_jd)?;
    let days = crate::js::from_json(&days)?;
    let gios = crate::js::from_json(&gios)?;
    let opts = crate::js::from_json(&opts)?;
    let out = port::build_events(from_jd, days, gios, opts)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "buildEvents")]
pub fn build_events(
    from_jd: String,
    days: String,
    gios: String,
    opts: String,
) -> Result<String, JsError> {
    build_events_inner(from_jd, days, gios, opts).map_err(|e| JsError::new(&e))
}

/// `toIcs`.
fn to_ics_inner(events: String, now: String) -> Result<String, String> {
    let events = crate::js::from_json(&events)?;
    let now = crate::js::from_json(&now)?;
    let out = port::to_ics(events, now)?;
    crate::js::to_json(&out)
}

#[wasm_bindgen(js_name = "toIcs")]
pub fn to_ics(events: String, now: String) -> Result<String, JsError> {
    to_ics_inner(events, now).map_err(|e| JsError::new(&e))
}

// ---- the binary wire ------------------------------------------------------------------------

thread_local! {
    static WIRE_IN: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
    static WIRE_OUT: std::cell::RefCell<Vec<u8>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// This module's memory, where `index.cjs` reads and writes the wire buffers.
#[wasm_bindgen(js_name = "rustAnythingWireMemory")]
pub fn rust_anything_wire_memory() -> JsValue {
    wasm_bindgen::memory()
}

/// The address of an argument buffer of `len` bytes.
#[wasm_bindgen(js_name = "rustAnythingWireIn")]
pub fn rust_anything_wire_in(len: u32) -> u32 {
    WIRE_IN.with(|b| {
        let mut b = b.borrow_mut();
        b.resize(len as usize, 0);
        b.as_mut_ptr() as usize as u32
    })
}

/// The address of the last result.
#[wasm_bindgen(js_name = "rustAnythingWireOut")]
pub fn rust_anything_wire_out() -> u32 {
    WIRE_OUT.with(|b| b.borrow().as_ptr() as usize as u32)
}

/// Reads `len` argument bytes, runs `f`, and keeps its result for `rustAnythingWireOut`.
fn wire_io(
    len: u32,
    f: impl FnOnce(&mut wire::Reader, &mut wire::Writer) -> Result<(), String>,
) -> Result<u32, JsError> {
    let input = WIRE_IN.with(|b| std::mem::take(&mut *b.borrow_mut()));
    let mut w = wire::Writer::default();
    let result = f(
        &mut wire::Reader::new(&input[..(len as usize).min(input.len())]),
        &mut w,
    );
    WIRE_IN.with(|b| *b.borrow_mut() = input);
    match result {
        // A result of another shape than the TS type: JS runs the call through JSON.
        Err(e) if e.starts_with(wire::MISMATCH) => return Ok(u32::MAX),
        other => other.map_err(|e| JsError::new(&e))?,
    }
    let n = w.0.len();
    WIRE_OUT.with(|b| *b.borrow_mut() = w.0);
    Ok(u32::try_from(n).unwrap_or(u32::MAX))
}

static S_JD_FROM_DATE_0: wire::Schema = wire::Schema::Dyn;
static S_JD_FROM_DATE_1: wire::Schema = wire::Schema::Dyn;
static S_JD_FROM_DATE_2: wire::Schema = wire::Schema::Dyn;
static S_JD_FROM_DATE_OUT: wire::Schema = wire::Schema::Dyn;

/// `jdFromDate` over the binary wire.
#[wasm_bindgen(js_name = "jdFromDateWire")]
pub fn jd_from_date_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let dd: serde_json::Value = wire::read_value(r, &S_JD_FROM_DATE_0)?;
        let mm: serde_json::Value = wire::read_value(r, &S_JD_FROM_DATE_1)?;
        let yy: serde_json::Value = wire::read_value(r, &S_JD_FROM_DATE_2)?;
        let out = &port::jd_from_date(dd, mm, yy)?;
        wire::write_value(w, out, &S_JD_FROM_DATE_OUT)?;
        Ok(())
    })
}

static S_JD_TO_DATE_0: wire::Schema = wire::Schema::Dyn;
static S_JD_TO_DATE_OUT: wire::Schema = wire::Schema::Dyn;

/// `jdToDate` over the binary wire.
#[wasm_bindgen(js_name = "jdToDateWire")]
pub fn jd_to_date_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let jd: serde_json::Value = wire::read_value(r, &S_JD_TO_DATE_0)?;
        let out = &port::jd_to_date(jd)?;
        wire::write_value(w, out, &S_JD_TO_DATE_OUT)?;
        Ok(())
    })
}

static S_CONVERT_SOLAR2_LUNAR_0: wire::Schema = wire::Schema::Dyn;
static S_CONVERT_SOLAR2_LUNAR_1: wire::Schema = wire::Schema::Dyn;
static S_CONVERT_SOLAR2_LUNAR_2: wire::Schema = wire::Schema::Dyn;
static S_CONVERT_SOLAR2_LUNAR_OUT: wire::Schema = wire::Schema::Dyn;

/// `convertSolar2Lunar` over the binary wire.
#[wasm_bindgen(js_name = "convertSolar2LunarWire")]
pub fn convert_solar2_lunar_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let dd: serde_json::Value = wire::read_value(r, &S_CONVERT_SOLAR2_LUNAR_0)?;
        let mm: serde_json::Value = wire::read_value(r, &S_CONVERT_SOLAR2_LUNAR_1)?;
        let yy: serde_json::Value = wire::read_value(r, &S_CONVERT_SOLAR2_LUNAR_2)?;
        let tz: Option<serde_json::Value> =
            serde_json::from_value(wire::read_dyn(r)?).map_err(|e| format!("TypeError: {e}"))?;
        let out = &port::convert_solar2_lunar(dd, mm, yy, tz)?;
        wire::write_value(w, out, &S_CONVERT_SOLAR2_LUNAR_OUT)?;
        Ok(())
    })
}

static S_BUILD_EVENTS_0: wire::Schema = wire::Schema::Dyn;
static S_BUILD_EVENTS_1: wire::Schema = wire::Schema::Dyn;
static S_BUILD_EVENTS_2: wire::Schema = wire::Schema::Dyn;
static S_BUILD_EVENTS_3: wire::Schema = wire::Schema::Dyn;
static S_BUILD_EVENTS_OUT: wire::Schema = wire::Schema::Dyn;

/// `buildEvents` over the binary wire.
#[wasm_bindgen(js_name = "buildEventsWire")]
pub fn build_events_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let from_jd: serde_json::Value = wire::read_value(r, &S_BUILD_EVENTS_0)?;
        let days: serde_json::Value = wire::read_value(r, &S_BUILD_EVENTS_1)?;
        let gios: serde_json::Value = wire::read_value(r, &S_BUILD_EVENTS_2)?;
        let opts: serde_json::Value = wire::read_value(r, &S_BUILD_EVENTS_3)?;
        let out = &port::build_events(from_jd, days, gios, opts)?;
        wire::write_value(w, out, &S_BUILD_EVENTS_OUT)?;
        Ok(())
    })
}

static S_TO_ICS_0: wire::Schema = wire::Schema::Dyn;
static S_TO_ICS_1: wire::Schema = wire::Schema::Dyn;
static S_TO_ICS_OUT: wire::Schema = wire::Schema::Dyn;

/// `toIcs` over the binary wire.
#[wasm_bindgen(js_name = "toIcsWire")]
pub fn to_ics_wire(len: u32) -> Result<u32, JsError> {
    wire_io(len, |r, w| {
        let events: serde_json::Value = wire::read_value(r, &S_TO_ICS_0)?;
        let now: serde_json::Value = wire::read_value(r, &S_TO_ICS_1)?;
        let out = &port::to_ics(events, now)?;
        wire::write_value(w, out, &S_TO_ICS_OUT)?;
        Ok(())
    })
}
