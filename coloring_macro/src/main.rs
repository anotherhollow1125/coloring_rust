use coloring_macro::repeat_for_types;
use num_traits::Num;

fn add<T: Num>(a: T, b: T) -> T {
    a + b
}

#[allow(unused)]
trait Hoge {}

fn main() {
    repeat_for_types!(for T in [u32, i32, usize] {
        impl Hoge for T {}

        let res = add::<T>(1, 2);
        println!("{}: {}", stringify!(T), res);

        fn hello_~T() {
            println!("Hello, {}!", stringify!(T));
        }
    });

    hello_usize();
    hello_i32();
    hello_u32();

    #[allow(unused)]
    struct Fuga;

    repeat_for_types!(for T in [Fuga] {
        #[allow(non_snake_case)]
        fn hello_~T() {
            println!("Hello, {}!", stringify!(T));
        }
    });

    hello_Fuga();

    #[allow(unused)]
    enum Eennumm {
        A,
        B,
        C,
    }

    let enm = Eennumm::A;

    repeat_for_types!(for T in [A, B, C] {
        match enm {
            #(
                Eennumm::T => println!("{} is matched!", stringify!(T)),
            )*
        }
    })
}
