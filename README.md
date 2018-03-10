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
use std::fmt::Write; // The write function to use, must be called Write (use lib::someWrite as Write;)
let mut out = String::new(); // The object to write to

svg!(&mut out,
    svg (width="100" height="100") [
        circle (cx="50" cy="50" r="30")
    ]
);
```
Result written to out:
```
<svg width="100" heigh="100">
    <circle cx="50" cy="50" r="30"/>
</svg>
```
### Elements, parantheses and brackets
Define elements by their XML-tag, their attributes in a () and their children in a [].
Parentheses and brackets are optional, these are all valid syntax.
```
svg () []
g []
circle ()
circle 

svg (width="200" height="100") ["Hello world!"]
g (size="20") ["Hello world!"]
circle (cx="10" cy="10" r="10")
```
### Nested elements and multiple attributes
Nest elements by putting new elements inside the []-brackets of their parent.
Add multiple attributes by delmimiting them with a space.

```
svg!(&mut out,
    svg [
        g(size="12" color="red") ["This is a group"]
        g(color="red") ["This is a group"]
    ]
);
```
### Rust expressions, functions and variables in SVG
Handle variables and function calls by wrapping them in a {} closure, expressions with a @-symbol.
```
use std::fmt::Write;
let mut out = String::new()

// Define your variables
let width = "200";
let height = "100";

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
                let radius = 15;
                svg!(&mut out, circle(cx="10" cy="10" r={radius}));
            }; 
        ]
    ]
);
```
