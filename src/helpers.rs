/*
 * Use this file if you want to extract helpers from your solutions.
 * Example import from this file: `use advent_of_code::helpers::example_fn;`.
 */

#[macro_export]
macro_rules! debug {
    ($var:expr) => {
        println!("{:?} = {:?}", stringify!($var), $var);
    };
}