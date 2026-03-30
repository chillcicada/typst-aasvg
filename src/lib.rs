use wasm_minimal_protocol::*;

initiate_protocol!();

#[derive(serde::Deserialize)]
struct RenderOptions {
    backdrop: bool,
    disable_text: bool,
    spaces: u32,
    stretch: bool,
}

impl From<RenderOptions> for aasvg::RenderOptions {
    fn from(opts: RenderOptions) -> Self {
        aasvg::RenderOptions {
            backdrop: opts.backdrop,
            disable_text: opts.disable_text,
            spaces: opts.spaces,
            stretch: opts.stretch,
        }
    }
}

#[wasm_func]
pub fn render_with_options(input: &[u8], options: &[u8]) -> Result<Vec<u8>, String> {
    let input = std::str::from_utf8(input).expect("non-utf8 string");
    let options: RenderOptions = ciborium::from_reader(options).expect("invalid options");
    let svg = aasvg::render_with_options(input, &options.into());
    Ok(svg.into_bytes())
}
