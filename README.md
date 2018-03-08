# svgmacro
A RUST SVG library for writing SVGs.

Snippet
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
Which returns
```
<svg xmlns="http://www.w3.org/2000/svg" width="1920" height="1080" viewBox="0 0 20 20">
    <g>
        <g>
            <circle cx="100" cy="100" r="100"/>
        </g>
        <circle cx="10" cy="10" r="10"/>
    </g>
</svg>
```
