//!
//! ## Introduction to svgmacro
//! A macro for writing SVGs from Rust.
//! Can write any valid XML-element.
//! Handle variables and returning functions by wrapping them in a {} closure.
//! Expressions begin with a @-symbol, such as if statements, for-loops or void-functions.
//! ## Examples
//!
//! ```
//! use std::fmt::Write;
//! let mut out = String::new();
//!
//! let width = 320;
//! SVG!(&mut out,
//!    svg (xmlns="http://www.w3.org/2000/svg" width={width} height="200") [
//!        g [
//!            g (id="paragraph_1" fill="white")[text["This is an example"]]
//!            
//!            circle(cx="10" cy="10" r="10")
//!
//!            @ for i in 0..3 {
//!                 // Need to return to the SVG! macro
//!                 SVG!(&mut out, 
//!                     circle(cx="10" cy="10" r="10")
//!                 );
//!            };                
//!        ]
//!     ]    
//! );
//! ```

#[macro_export]
macro_rules! _parse_args {
    ($w:expr, ) => (());

    ($w:expr, $name:ident = {$param:expr} $($rest:tt)*) => {{
            write!($w, " {}=\"{}\"", stringify!($name), $param)
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};

    ($w:expr, {$name:ident} = $param:tt $($rest:tt)*) => {{
            write!($w, " {}={}", $name, stringify!($param))
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};

    ($w:expr, {$name:ident} = {$param:expr} $($rest:tt)*) => {{
            write!($w, " {}=\"{}\"", $name, $param)
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};

    ($w:expr, {$attribute:expr} $($rest:tt)*) => {{
            write!($w, " {}", $attribute)
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};

    ($w:expr, $name:ident = $param:tt $($rest:tt)*) => {{
            write!($w, " {}={}", stringify!($name), stringify!($param))
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};
    
    ($w:expr, $name:ident:$subname:ident = $param:tt $($rest:tt)*) => {{
            write!($w, " {}:{}={}", stringify!($name), stringify!($subname), stringify!($param))
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};
    
    ($w:expr, $name:ident:$subname:ident = {$param:expr} $($rest:tt)*) => {{
            write!($w, " {}:{}={}", stringify!($name), stringify!($subname), $param)
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};

    ($w:expr, $name:ident-$subname:ident = $param:tt $($rest:tt)*) => {{
            write!($w, " {}-{}={}", stringify!($name), stringify!($subname), stringify!($param))
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};
    
    ($w:expr, $name:ident-$subname:ident = {$param:expr} $($rest:tt)*) => {{
            write!($w, " {}-{}={}", stringify!($name), stringify!($subname), $param)
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};
}

#[macro_export]
macro_rules! SVG {
    ($w:expr, ) => (());

    ($w:expr, @ $inner:expr; $($rest:tt)*) => {{
        $inner;
        SVG!($w, $($rest)*);
    }};   

    ($w:expr, {$tag:expr} ($( $attr:tt )*) [ $($inner:tt)* ] $($rest:tt)*) => {
        {
            write!($w, "<{}", $tag)
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($attr)*);
            write!($w, ">")
                .expect("Error occurred while trying to write in String");
            SVG!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            SVG!($w, $($rest)*);
        }
    };   

    ($w:expr, {$tag:expr} ($( $attr:tt )*) $($rest:tt)*) => {
        {
            write!($w, "<{}", $tag)
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($attr)*);
            write!($w, "/>")
                .expect("Error occurred while trying to write in String");
            SVG!($w, $($rest)*);
        }
    };  

    ($w:expr, {$tag:expr} [ $($inner:tt)* ] $($rest:tt)*) => {
        {
            write!($w, "<{}>", $tag)
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($attr)*);
            SVG!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            SVG!($w, $($rest)*);
        }
    };
    
    ($w:expr, {$e:expr} $($rest:tt)*) => {{
        write!($w, "{}", $e)
            .expect("Error occurred while trying to write in String");
        SVG!($w, $($rest)*);
    }};    

    ($w:expr, $e:tt) => (write!($w, "{}", $e)
            .expect("Error occurred while trying to write in String"));
    
    ($w:expr, $tag:ident ($( $attr:tt )*) [ $($inner:tt)* ] $($rest:tt)*) => {
        {
            write!($w, "<{}", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($attr)*);
            write!($w, ">")
                .expect("Error occurred while trying to write in String");
            SVG!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            SVG!($w, $($rest)*);
        }
    };
    
    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {
        {
            write!($w, "<{}>", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            SVG!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            SVG!($w, $($rest)*);
        }
    };

    ($w:expr, $tag:ident ($( $attr:tt )*) $($rest:tt)*) => {
        {
            write!($w, "<{}", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($attr)*);
            write!($w, "/>")
                .expect("Error occurred while trying to write in String");
            SVG!($w, $($rest)*);
        }
    };
    
}


#[cfg(test)]
mod tests {
    fn attribute_fn() -> String {
        use std::fmt::Write;
        let mut out = String::new();
        write!(out, "200").unwrap();
        out
    }

    fn content_fn() -> String {
        use std::fmt::Write;
        let mut out = String::new();
        write!(out, "Hello, this is an example").unwrap();
        out
    }

    fn my_function(out: &mut String) {
        use std::fmt::Write;
        SVG!(out,
            circle (cx="100" cy="100" r="10")        
        );
    }

    #[test]    
    fn test_special_fn() {
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            svg (width={attribute_fn()}) [
                {content_fn()} 
                circle(fill="red")
                {content_fn()} 
                @ my_function(&mut out);
                {content_fn()}        
            ]
        );
        assert_eq!(out, "<svg width=\"200\">Hello, this is an example<circle fill=\"red\"/>Hello, this is an example<circle cx=\"100\" cy=\"100\" r=\"10\"/>Hello, this is an example</svg>");
    }
    
    #[test]    
    fn test_flat() {
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            svg () []
        );
        assert_eq!(out, "<svg></svg>");
    }

    #[test]    
    fn test_regular_svg() {
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            "<svg><circle row=\"100\"></circle></svg>"
        );
        assert_eq!(out, "<svg><circle row=\"100\"></circle></svg>");
    }

    #[test]    
    fn test_deep() {
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            svg() [
                g() [
                    g () [
                    ]               
                ]            
            ]
        );
        assert_eq!(out, "<svg><g><g></g></g></svg>");
    }
    
    #[test]    
    fn test_attributes() {
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            svg (width="200" height="200") [
                g() []            
            ]
        );
        assert_eq!(out, "<svg width=\"200\" height=\"200\"><g></g></svg>");
    }

    #[test]    
    fn test_content() {
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            svg() [
                g() ["Hello"]            
            ]
        );
        assert_eq!(out, "<svg><g>Hello</g></svg>");
    }

    #[test]    
    fn test_absence() {
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            svg [
                circle()
                g[]
            ]
        );
        assert_eq!(out, "<svg><circle/><g></g></svg>");
    }

    #[test]    
    fn test_variable() {
        use std::fmt::Write;
        let mut out = String::new();
        let width = 200;
        SVG!(&mut out,
            svg(width={width}) [
                circle(width={width})
            ]
        );
        assert_eq!(out, "<svg width=\"200\"><circle width=\"200\"/></svg>");
    }

    #[test]    
    fn test_function() {
        fn my_width() -> i32{
            let width = 200;
            width
        }
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            svg(width={my_width()}) [
                circle(width={my_width()})
                {my_width()}
            ]
        );
        assert_eq!(out, "<svg width=\"200\"><circle width=\"200\"/>200</svg>");
    }

    #[test]    
    fn test_str_as_variable() {
        let my_str: &str = "200";
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            svg(width={my_str}) [
                circle(width={my_str})
            ]
        );
        assert_eq!(out, "<svg width=\"200\"><circle width=\"200\"/></svg>");
    }

    #[test]    
    fn test_dash_attribs() {
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            text (x="0" y="35" fill="red" font-family="Verdana" font-size="35")[
                "Hello, out there"
            ]
        );
        assert_eq!(out, "<text x=\"0\" y=\"35\" fill=\"red\" font-family=\"Verdana\" font-size=\"35\">Hello, out there</text>");
    }

    #[test]    
    fn test_expression() {
        use std::fmt::Write;
        let mut out = String::new();
        SVG!(&mut out,
            @ for _i in 0..2 {
                SVG!(&mut out, circle(dy="20"));            
            };
        );
        assert_eq!(out, "<circle dy=\"20\"/><circle dy=\"20\"/>");
    }

    #[test]    
    fn test_variable_as_element() {
        use std::fmt::Write;
        let mut out = String::new();
        let circle_element = "circle".to_string();
        SVG!(&mut out,
            {circle_element}()
        );
        assert_eq!(out, "<circle/>");
    }

    #[test]    
    fn test_variable_as_element_with_attibutes() {
        use std::fmt::Write;
        let mut out = String::new();
        let circle_element = "circle".to_string();
        SVG!(&mut out,
            {circle_element} (cx="20" cy="20")
        );
        assert_eq!(out, "<circle cx=\"20\" cy=\"20\"/>");
    }

    #[test]    
    fn test_variable_as_attribute() {
        use std::fmt::Write;
        let mut out = String::new();
        let circle_element = "circle".to_string();
        let circle_cx = "cx=\"20\"".to_string();
        SVG!(&mut out,
            {circle_element} ({circle_cx} cy="20")
        );
        assert_eq!(out, "<circle cx=\"20\" cy=\"20\"/>");
    }

    #[test]    
    fn test_variable_as_attribute_element() {
        use std::fmt::Write;
        let mut out = String::new();
        let circle_element = "circle".to_string();
        let cx = "cx".to_string();
        SVG!(&mut out,
            {circle_element} ({cx}="20")
        );
        assert_eq!(out, "<circle cx=\"20\"/>");
    }

}
