// module.
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() { macro_rules! my_macro {
     () => {};
 }}
