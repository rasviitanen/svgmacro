# svgmacro
A Rust library for writing SVGs, using zero dependencies and does not require a nightly build. Can write any valid XML-element.
The result may be written to any file-like object.

Handle variables and function calls by wrapping them in a {} closure.


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
Handle expreessions by assigning a @ and terminationg with a ;.
For example.

```
svg!(&mut out,
    svg (xmlns="http://www.w3.org/2000/svg" width={get_width()} height={height} viewBox="0 0 20 20") [
        @ for i in 1..2 {
            let width = "100";
            svg!(&mut out, circle(width={width}));
        };

        @ if 1 > 0 {
            svg!(&mut out, rect(width="200"));            
        };
    ]
);
```
