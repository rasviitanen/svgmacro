# svgmacro
A Rust library for writing SVGs using zero dependencies and does not require a nightly build. Can write any valid XML-element.
The result may be written to any file-like object.

Handle variables and function calls by wrapping them in a {} closure, expressions with a @-symbol.


Example:
```
svg!(&mut out,
    svg (xmlns="http://www.w3.org/2000/svg" width={get_width()} height={height} viewBox="0 0 20 20") [
        g [
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

A more complicated SVG can be written like this:
```
svg!(&mut out,
    svg(version="1.1"
        id="Layer_1"
        xmlns="http://www.w3.org/2000/svg"
        xmlns:xlink="http://www.w3.org/1999/xlink"
        x="0px" 
        y="0px"
        width="500px" 
        height="100px" 
        viewBox="0 0 50 30"
        xml:space="preserve"
        ) 
    [
        @ for i in 0..5 {
            svg!(&mut out,
                rect(x={i*10} y="10" width="4" height="10" fill="#333" opacity="0.2") [
                animate(attributeName="y" attributeType="XML" values="10; 5; 10" begin="0s" dur="0.6s" repeatCount="indefinite")                        
                animate(attributeName="opacity" attributeType="XML" values="0.2; 1; .2" begin="0s" dur="0.6s" repeatCount="indefinite")
                animate(attributeName="height" attributeType="XML" values="10; 20; 10" begin="0s" dur="0.6s" repeatCount="indefinite")
            ]);
        };
    ]
);
```
Click for animation:
<img src="./foo.svg">
