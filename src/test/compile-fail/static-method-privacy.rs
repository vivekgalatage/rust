mod a {
    pub struct S;
    impl S {
        fn new() -> S { S }
    }
}

fn main() {
    let _ = a::S::new();    //~ ERROR function `new` is private
}

