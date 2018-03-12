//!
//! ## Introduction to svgmacro
//! A macro for writing SVGs.
//! Can write any valid XML-element. The result may be written to any file-like object.
//! Handle variables and function calls by wrapping them in a {} closure, expressions begin with a @-symbol.
//! ## Examples
//!
//! ```
//! use std::fmt::Write;
//! let mut out = String::new();
//!
//! let width = 320;
//! svg!(&mut out,
//!    svg (xmlns="http://www.w3.org/2000/svg" width={width} height="200") [
//!        g [
//!            g (id="paragraph" size="12")["This is the content of a group"]
//!            circle(cx="10" cy="10" r="10")
//!            @ for i in 0..3 {
//!                svg!(&mut out, circle(cx="10" cy="10" r="10"));
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
            write!($w, " {}:{}={}", stringify!($name), stringify!($subname), stringify!($param))
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};
    
    ($w:expr, $name:ident-$subname:ident = {$param:expr} $($rest:tt)*) => {{
            write!($w, " {}:{}={}", stringify!($name), stringify!($subname), $param)
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($rest)*);
    }};
}

#[macro_export]
macro_rules! svg {
    ($w:expr, ) => (());

    ($w:expr, @ $inner:expr; $($rest:tt)*) => {{
        $inner;
        svg!($w, $($rest)*);
    }};    
    
    ($w:expr, {$e:expr} $($rest:tt)*) => {{
        write!($w, "{}", $e)
            .expect("Error occurred while trying to write in String");
        svg!($w, $($rest)*);
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
            svg!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            svg!($w, $($rest)*);
        }
    };
    
    ($w:expr, $tag:ident [ $($inner:tt)* ] $($rest:tt)*) => {
        {
            write!($w, "<{}>", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            svg!($w, $($inner)*);
            write!($w, "</{}>", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            svg!($w, $($rest)*);
        }
    };

    ($w:expr, $tag:ident ($( $attr:tt )*) $($rest:tt)*) => {
        {
            write!($w, "<{}", stringify!($tag))
                .expect("Error occurred while trying to write in String");
            _parse_args!($w, $($attr)*);
            write!($w, "/>")
                .expect("Error occurred while trying to write in String");
            svg!($w, $($rest)*);
        }
    };
    
}


#[cfg(test)]
mod tests {
    
    #[test]    
    fn test_flat() {
        use std::fmt::Write;
        let mut out = String::new();
        svg!(&mut out,
            svg () []
        );
        assert_eq!(out, "<svg></svg>");
    }

    #[test]    
    fn test_deep() {
        use std::fmt::Write;
        let mut out = String::new();
        svg!(&mut out,
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
        svg!(&mut out,
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
        svg!(&mut out,
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
        svg!(&mut out,
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
        svg!(&mut out,
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
        svg!(&mut out,
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
        svg!(&mut out,
            svg(width={my_str}) [
                circle(width={my_str})
            ]
        );
        assert_eq!(out, "<svg width=\"200\"><circle width=\"200\"/></svg>");
    }

}
