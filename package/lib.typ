#let aasvg-wasm = plugin("./typst_aasvg.wasm")

#let aasvg(
  str,
  backdrop: false,
  disable_text: false,
  spaces: 2,
  stretch: false,
  ..args,
) = {
  image(
    aasvg-wasm.render_with_options(
      bytes(str),
      cbor.encode((backdrop: backdrop, disable_text: disable_text, spaces: spaces, stretch: stretch)),
    ),
    format: "svg",
    ..args,
  )
}
