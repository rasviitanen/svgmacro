macro_rules! parse_args {
    ($w:expr, ) => (());

    ($w:expr, $name:ident = {$param:expr} $($rest:tt)*) => {{
            write!($w, " {}=\"{}\"", stringify!($name), $param)
                .expect("Error occurred while trying to write in String");
            parse_args!($w, $($rest)*);
    }};

    ($w:expr, $name:ident = $param:tt $($rest:tt)*) => {{
            write!($w, " {}={}", stringify!($name), stringify!($param))
                .expect("Error occurred while trying to write in String");
            parse_args!($w, $($rest)*);
    }};
    
    ($w:expr, $name:ident:$subname:ident = $param:tt $($rest:tt)*) => {{
            write!($w, " {}:{}={}", stringify!($name), stringify!($subname), stringify!($param))
                .expect("Error occurred while trying to write in String");
            parse_args!($w, $($rest)*);
    }};
    
    ($w:expr, $name:ident:$subname:ident = {$param:expr} $($rest:tt)*) => {{
            write!($w, " {}:{}={}", stringify!($name), stringify!($subname), $param)
                .expect("Error occurred while trying to write in String");
            parse_args!($w, $($rest)*);
    }};
}

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
            parse_args!($w, $($attr)*);
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
            parse_args!($w, $($attr)*);
            write!($w, "/>")
                .expect("Error occurred while trying to write in String");
            svg!($w, $($rest)*);
        }
    };
    
}


#[cfg(test)]
mod tests {

    fn save(data: String) {
        use std::fs::File;
        use std::io::Write;

        let mut f = File::create("foo.svg").expect("Unable to create file");
        f.write_all(data.as_bytes()).expect("Unable to write data");
    
    }
    
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
    fn save_svg() {
        fn content() -> String {
            use std::fmt::Write;
            let mut out = String::new();
            svg!(&mut out,
                svg() [
                    g() ["Hello"]            
                ]
            );
            out
        }
        use std::fs::File;
        use std::io::Write;
        let data = "Some data!";
        let mut f = File::create("test.svg").expect("Unable to create file");
        f.write_all(content().as_bytes()).expect("Unable to write data");
        assert_eq!(content(), "<svg><g>Hello</g></svg>");
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
    
    #[test]    
    fn test_loop() {
        use std::fmt::Write;
        let mut out = String::new();
        svg!(&mut out,
            svg(version="1.1"
                id="Layer_1"
                xmlns="http://www.w3.org/2000/svg"
                xmlns:xlink="http://www.w3.org/1999/xlink"
                x="0px" 
                y="0px"
                width="500px" 
                height="200px" 
                viewBox="0 0 50 10"
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
        save(out);
        //assert_eq!(out, "<svg><circle width=\"100\"/><rect width=\"200\"/></svg>");
    }

}
