pub mod container_parser;
#[allow(clippy::all)]
#[allow(dead_code)]
#[rustfmt::skip]
mod bindings {
    include!("model.rs");
}
pub use bindings::*;
