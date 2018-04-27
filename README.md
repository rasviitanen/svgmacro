# svgmacro
[![Cargo](https://img.shields.io/crates/v/svgmacro.svg)](https://crates.io/crates/svgmacro) 
[![Documentation](https://docs.rs/svgmacro/badge.svg)](https://docs.rs/svgmacro/)
<br>
A Rust library for writing SVGs. Can write any SVG-element.
Function calls and variables defined in Rust can be used in the SVG without trouble.

Questions or requests can be emailed to
rasviitanen@gmail.com.

Add the following to your Cargo.toml
```
svgmacro = "0.2.2"
```

To use the crate in your module, simply add:
```
#[macro_use]
extern crate svgmacro;
```

## Examples
To use the macro, create a new String to store the result, and add "use std::fmt:Write"
to be able to successfully write to it. Other types are also supported.
```
use std::fmt::Write;
let mut out = String::new();
```
Below is a quick example on how to use the macro. SVG elements and attributes are defined by their regular names found in the SVG reference.
```
use std::fmt::Write;
let mut out = String::new();

SVG!(&mut out,
    svg (width="100" height="100") [
        circle (cx="50" cy="50" r="30")
    ]
);
```
Result written to out:
```
<svg width="100" height="100">
    <circle cx="50" cy="50" r="30"/>
</svg>
```
### Elements, parantheses and brackets
Define elements in plain text by their element tag, their attributes in a parenthesis, (), and their children in a bracket, [].
These are all valid syntax (note that you do not have to use both brackets and parantheses).
```
g (fill="red") [circle (cx="10" cy="10" r="10")]
g () [circle (cx="10" cy="10" r="10")]
g (fill="red") []
circle (cx="10" cy="10" r="10")
text ["Test"]
```
### Nested elements and multiple attributes
Nest elements by putting new elements inside the []-brackets of their parent.
Add multiple attributes by delmimiting them with a space.

```
SVG!(&mut out,
    svg [
        g(size="12" color="red") [
            circle(cx="10" cy="10" r="10")
            circle(cx="20" cy="20" r="10")
        ]
        g(color="red") [
            circle(cx="20" cy="20" r="10")
        ]
    ]
);
```
### Rust expressions, functions and variables in SVG
Handle variables and returning function calls by wrapping them in a {} closure, 
expressions such as void functions, for loops and if expressions (any Rust code is ok) with a @-symbol and terminate with a ";".
```
use std::fmt::Write;
let mut out = String::new()

// Define your variables
let width = "200";
let height = "100";

SVG!(&mut out,
    svg (width={width} height={height}) [
        g(fill={get_color()}) [
            @ create_cool_shape(&mut out); 
        ]
        g [
            @ for i in 1..3 {
                let radius = 15*i;
                // It is important to call the macro again, when using it from inside and expression.
                SVG!(&mut out, circle(cx={10*i} cy="10" r={radius}));
            }; 
        ]
    ]
);
```
