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
<img src="data:image/svg+xml;base64,PHN2ZyB2ZXJzaW9uPSIxLjEiIGlkPSJMYXllcl8xIiB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHhtbG5zOnhsaW5rPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5L3hsaW5rIiB4PSIwcHgiIHk9IjBweCIgd2lkdGg9IjUwMHB4IiBoZWlnaHQ9IjEwMHB4IiB2aWV3Qm94PSIwIDAgNTAgMzAiIHhtbDpzcGFjZT0icHJlc2VydmUiPjxyZWN0IHg9IjAiIHk9IjEwIiB3aWR0aD0iNCIgaGVpZ2h0PSIxMCIgZmlsbD0iIzMzMyIgb3BhY2l0eT0iMC4yIj48YW5pbWF0ZSBhdHRyaWJ1dGVOYW1lPSJ5IiBhdHRyaWJ1dGVUeXBlPSJYTUwiIHZhbHVlcz0iMTA7IDU7IDEwIiBiZWdpbj0iMHMiIGR1cj0iMC42cyIgcmVwZWF0Q291bnQ9ImluZGVmaW5pdGUiLz48YW5pbWF0ZSBhdHRyaWJ1dGVOYW1lPSJvcGFjaXR5IiBhdHRyaWJ1dGVUeXBlPSJYTUwiIHZhbHVlcz0iMC4yOyAxOyAuMiIgYmVnaW49IjBzIiBkdXI9IjAuNnMiIHJlcGVhdENvdW50PSJpbmRlZmluaXRlIi8+PGFuaW1hdGUgYXR0cmlidXRlTmFtZT0iaGVpZ2h0IiBhdHRyaWJ1dGVUeXBlPSJYTUwiIHZhbHVlcz0iMTA7IDIwOyAxMCIgYmVnaW49IjBzIiBkdXI9IjAuNnMiIHJlcGVhdENvdW50PSJpbmRlZmluaXRlIi8+PC9yZWN0PjxyZWN0IHg9IjEwIiB5PSIxMCIgd2lkdGg9IjQiIGhlaWdodD0iMTAiIGZpbGw9IiMzMzMiIG9wYWNpdHk9IjAuMiI+PGFuaW1hdGUgYXR0cmlidXRlTmFtZT0ieSIgYXR0cmlidXRlVHlwZT0iWE1MIiB2YWx1ZXM9IjEwOyA1OyAxMCIgYmVnaW49IjBzIiBkdXI9IjAuNnMiIHJlcGVhdENvdW50PSJpbmRlZmluaXRlIi8+PGFuaW1hdGUgYXR0cmlidXRlTmFtZT0ib3BhY2l0eSIgYXR0cmlidXRlVHlwZT0iWE1MIiB2YWx1ZXM9IjAuMjsgMTsgLjIiIGJlZ2luPSIwcyIgZHVyPSIwLjZzIiByZXBlYXRDb3VudD0iaW5kZWZpbml0ZSIvPjxhbmltYXRlIGF0dHJpYnV0ZU5hbWU9ImhlaWdodCIgYXR0cmlidXRlVHlwZT0iWE1MIiB2YWx1ZXM9IjEwOyAyMDsgMTAiIGJlZ2luPSIwcyIgZHVyPSIwLjZzIiByZXBlYXRDb3VudD0iaW5kZWZpbml0ZSIvPjwvcmVjdD48cmVjdCB4PSIyMCIgeT0iMTAiIHdpZHRoPSI0IiBoZWlnaHQ9IjEwIiBmaWxsPSIjMzMzIiBvcGFjaXR5PSIwLjIiPjxhbmltYXRlIGF0dHJpYnV0ZU5hbWU9InkiIGF0dHJpYnV0ZVR5cGU9IlhNTCIgdmFsdWVzPSIxMDsgNTsgMTAiIGJlZ2luPSIwcyIgZHVyPSIwLjZzIiByZXBlYXRDb3VudD0iaW5kZWZpbml0ZSIvPjxhbmltYXRlIGF0dHJpYnV0ZU5hbWU9Im9wYWNpdHkiIGF0dHJpYnV0ZVR5cGU9IlhNTCIgdmFsdWVzPSIwLjI7IDE7IC4yIiBiZWdpbj0iMHMiIGR1cj0iMC42cyIgcmVwZWF0Q291bnQ9ImluZGVmaW5pdGUiLz48YW5pbWF0ZSBhdHRyaWJ1dGVOYW1lPSJoZWlnaHQiIGF0dHJpYnV0ZVR5cGU9IlhNTCIgdmFsdWVzPSIxMDsgMjA7IDEwIiBiZWdpbj0iMHMiIGR1cj0iMC42cyIgcmVwZWF0Q291bnQ9ImluZGVmaW5pdGUiLz48L3JlY3Q+PHJlY3QgeD0iMzAiIHk9IjEwIiB3aWR0aD0iNCIgaGVpZ2h0PSIxMCIgZmlsbD0iIzMzMyIgb3BhY2l0eT0iMC4yIj48YW5pbWF0ZSBhdHRyaWJ1dGVOYW1lPSJ5IiBhdHRyaWJ1dGVUeXBlPSJYTUwiIHZhbHVlcz0iMTA7IDU7IDEwIiBiZWdpbj0iMHMiIGR1cj0iMC42cyIgcmVwZWF0Q291bnQ9ImluZGVmaW5pdGUiLz48YW5pbWF0ZSBhdHRyaWJ1dGVOYW1lPSJvcGFjaXR5IiBhdHRyaWJ1dGVUeXBlPSJYTUwiIHZhbHVlcz0iMC4yOyAxOyAuMiIgYmVnaW49IjBzIiBkdXI9IjAuNnMiIHJlcGVhdENvdW50PSJpbmRlZmluaXRlIi8+PGFuaW1hdGUgYXR0cmlidXRlTmFtZT0iaGVpZ2h0IiBhdHRyaWJ1dGVUeXBlPSJYTUwiIHZhbHVlcz0iMTA7IDIwOyAxMCIgYmVnaW49IjBzIiBkdXI9IjAuNnMiIHJlcGVhdENvdW50PSJpbmRlZmluaXRlIi8+PC9yZWN0PjxyZWN0IHg9IjQwIiB5PSIxMCIgd2lkdGg9IjQiIGhlaWdodD0iMTAiIGZpbGw9IiMzMzMiIG9wYWNpdHk9IjAuMiI+PGFuaW1hdGUgYXR0cmlidXRlTmFtZT0ieSIgYXR0cmlidXRlVHlwZT0iWE1MIiB2YWx1ZXM9IjEwOyA1OyAxMCIgYmVnaW49IjBzIiBkdXI9IjAuNnMiIHJlcGVhdENvdW50PSJpbmRlZmluaXRlIi8+PGFuaW1hdGUgYXR0cmlidXRlTmFtZT0ib3BhY2l0eSIgYXR0cmlidXRlVHlwZT0iWE1MIiB2YWx1ZXM9IjAuMjsgMTsgLjIiIGJlZ2luPSIwcyIgZHVyPSIwLjZzIiByZXBlYXRDb3VudD0iaW5kZWZpbml0ZSIvPjxhbmltYXRlIGF0dHJpYnV0ZU5hbWU9ImhlaWdodCIgYXR0cmlidXRlVHlwZT0iWE1MIiB2YWx1ZXM9IjEwOyAyMDsgMTAiIGJlZ2luPSIwcyIgZHVyPSIwLjZzIiByZXBlYXRDb3VudD0iaW5kZWZpbml0ZSIvPjwvcmVjdD48L3N2Zz4=">
