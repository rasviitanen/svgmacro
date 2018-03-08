# svgmacro
A Rust library for writing SVGs, using zero dependencies and does not require a nightly build.

Handles variables and function calls by wrapping them in a {} closure.

Can create any valid XML-element.

Snippet
```
svg!(&mut out,
    svg (xmlns="http://www.w3.org/2000/svg" width={get_width()} height={height} viewBox="0 0 20 20") [
        g [
            {create_circle_group()}               
            circle(cx="10" cy="10" r="10")                
        ]
    ]
);
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
