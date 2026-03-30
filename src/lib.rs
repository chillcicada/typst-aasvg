use aasvg::RenderOptions;
use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn render_with_options(input: &[u8], options: &[u8]) -> Result<Vec<u8>, String> {
    let input = std::str::from_utf8(input).expect("non-utf8 string");
    let options: RenderOptions = ciborium::from_reader(options).expect("invalid options");
    let svg = aasvg::render_with_options(input, &options.into());
    Ok(svg.into_bytes())
}
