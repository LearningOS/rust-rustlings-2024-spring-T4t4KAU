// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

    // std::env::set_var("TEST_FOO", timestamp.to_string());
    println!("cargo:rustc-env=TEST_FOO={}", timestamp.to_string());
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

// #[cfg(test)]
// mod tests {

//     #[test]
//     fn test_success() {
//         #[cfg(feature = "pass")]
//         return;

//         panic!("no cfg set");
//     }
// }
