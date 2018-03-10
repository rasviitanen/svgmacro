# svgmacro
[![Cargo](https://img.shields.io/crates/v/svgmacro.svg)](https://crates.io/crates/svgmacro) 
[![Documentation](https://docs.rs/svgmacro/badge.svg)](https://docs.rs/svgmacro/)
<br>
A Rust library for writing SVGs. Can write any valid SVG-element.
The result may be written to any file-like object.
Function calls and variables are available.

Add the following to your Cargo.toml
```
svgmacro = "0.1.3"
```

To use the crate in your module, simply add:
```
#[macro_use]
extern crate svgmacro;
```

## Examples
To use the macro, define a filelike object to store the result, and call the svg! macro.
```
use std::fmt::Write;
let mut out = String::new()
svg!(&mut out,
    svg (width=200 height=100) [
        "Hello world!"
    ]
);
```
Define elements by their XML-tag, their attributes in a () and their contents in a [].

```
svg () []
svg (width="200" height="100") ["Hello world!"]

g () []
g (size="20") ["Hello world!"]
```
### Parantheses and brackets
Parentheses and brackets are optional, these are all valid syntax.
```
svg () []
g []
circle ()
circle 
```
### Nested elements and multiple attributes
Nest elements by putting new elements inside the []-brackets of their parent.
Add multiple arguments by delmimiting them with a " ".

```
svg [
    g(size="12" color="red") ["This is a group"]
    g(color="red") ["This is a group"]
]
```
### Communication with Rust expressions, functions and variables
Handle variables and function calls by wrapping them in a {} closure, expressions with a @-symbol.
```
// Define your variables
let width = "200";
let height = "100";

fn create_cool_shape() -> String {
    // Create a cool shape
}

svg!(&mut out,
    svg [
        g(width={width} height={height}) [
            "This is a group"
        ]
        g [
            {create_cool_shape(&mut out)}        
        ]
        g [
        @ for i in 0..3 {
                svg!(&mut out, circle(cx="10" cy="10" r="10"));
            }; 
        ]
    ]
);
```

Simple svg, showing variables, functioncalls and a for loop.
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
