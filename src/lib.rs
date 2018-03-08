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
}


macro_rules! svg {
    ($w:expr, ) => (());

    ($w:expr, {$e:expr} $($rest:tt)*) => {{
        write!($w, "{}", $e)
            .expect("Error occurred while trying to write in String");
        svg!($w, $($rest)*);
    }};

    ($w:expr, $e:tt) => (write!($w, "{}", stringify!($e))
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

pub fn get_width() -> i32 {
    let width = 1920;
    width
}

pub fn create_circle_group() -> String {
    use std::fmt::Write;
    let mut out = String::new();
    let height = 1080;
    svg!(&mut out,
        g [
            circle(cx="100" cy="100" r="100")
        ]
    );
    out
}

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

pub fn save_svg(file_name: &str) {
    use std::io::Write;
    use std::fs::File;
    let mut f = File::create(file_name).expect("Unable to create file");
    f.write_all(test_build().as_bytes()).expect("Unable to write data");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]    
    fn test_svg() {
        save_svg("test.svg");
        assert_eq!(test_build(), "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1920\" height=\"1080\" viewBox=\"0 0 20 20\"><g><circle cx=\"10\" cy=\"10\" r=\"10\"/></g></svg>");
    }
}
