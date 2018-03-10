# svgmacro
[![Cargo](https://img.shields.io/crates/v/svgmacro.svg)](https://crates.io/crates/svgmacro) <br>
A Rust library for writing SVGs using zero dependencies and does not require a nightly build. Can write any valid XML-element.
The result may be written to any file-like object.

Handle variables and function calls by wrapping them in a {} closure, expressions with a @-symbol.

Read more at https://crates.io/crates/svgmacro

Add the following to your Cargo.toml
```
svgmacro = "0.1.2"
```

To use the crate in your module, simply add:
```
#[macro_use]
extern crate svgmacro;
```


Example:
```
use std::fmt::Write;
let mut out = String::new()
svg!(&mut out,
    svg (xmlns="http://www.w3.org/2000/svg" width={get_width()} height={height} viewBox="0 0 20 20") [
        g [
            circle(cx="10" cy="10" r="10")
            {create_circle_group()}
            @ for i in 0..3 {
                svg!(&mut out, circle(cx="10" cy="10" r="10"));
            };                
        ]
    ]
);
```
Return:
```
<svg xmlns="http://www.w3.org/2000/svg" width="1920" height="1080" viewBox="0 0 20 20">
    <g>
        <g>
            <circle cx="100" cy="100" r="100"/>
        </g>
        <circle cx="10" cy="10" r="10"/>
        <circle cx="10" cy="10" r="10"/>
        <circle cx="10" cy="10" r="10"/>
    </g>
</svg>
```
