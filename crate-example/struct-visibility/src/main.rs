mod my {
    pub struct OpenBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents
            }
        }

        pub fn contents(&self) -> T {
            self.contents
        }
    }
}



fn main() {
    let open_box = my::OpenBox { contents: "public information" };

    println!("the open box contains:{}", open_box.contents);

    // let closed_box = my::ClosedBox { contents: "classified information" };

    let _closed_box = my::ClosedBox::new("classified information");

    println!("the closed box containts: {}", _closed_box.contents());
}
