#[allow(clippy::all)]
#[allow(warnings)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
}
pub use bindings::*;
