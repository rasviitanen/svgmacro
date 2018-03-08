# svgmacro
A RUST SVG library for writing SVGs.

```
pub fn test_build() -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let height = 1080;
    svg!(&mut out,
        svg (xmlns="http://www.w3.org/2000/svg" width={get_width()} height={height} viewBox="0 0 20 20") [
            g [
                {create_circle_group()}               
                circle(cx="10" cy="10" r="10")                
            ]
        ]
    );
    out
}
```
