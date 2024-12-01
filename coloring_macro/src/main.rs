use coloring_macro::repeat_for_types;
use num_traits::Num;

fn add<T: Num>(a: T, b: T) -> T {
    a + b
}

trait Hoge {}

fn main() {
    repeat_for_types!(for T in [u32, i32, usize] {
        impl Hoge for T {}

        let res = add::<T>(1, 2);
        println!("{}: {}", stringify!(T), res);
    });
}
