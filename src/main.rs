#![allow(unused_variables, dead_code)]

//! Section 10 — Closures.
//!
//! Starting point: a plain `fn` validator passed nothing but data.
//! Closures come next, when the validation rule itself becomes the argument.

// INFO: -------------------------------------------------
// INFO: FUNCTION POINTERS
// INFO: -------------------------------------------------

/// A person record used as sample data for the closure exercises.
struct User {
    /// Display name. Empty string means "not filled in".
    name: String,
    /// Age in years; `u8` caps at 255, plenty for a human.
    age: u8,
    /// Yearly salary, whole currency units.
    salary: u32,
}

// The `fn` version, kept for comparison with the closure below.
//
// Returns `true` when `name` is not empty. Takes `&str` rather than
// `&String` so it accepts literals too.
//
// fn validate_user(name: &str) -> bool {
//     name.len() != 0
// }

fn is_valid_user(
    name: &str,
    banned_user_name: &str,
    age: u8,
    simple_validator: fn(&str, &str) -> bool,
    advance_validator: fn(u8) -> bool,
) -> bool {
    simple_validator(name, banned_user_name) && advance_validator(age)
}

fn validate_user_simple(name: &str, banned_user_name: &str) -> bool {
    name.len() != 0 && name != banned_user_name
}

fn validate_user_advance(age: u8) -> bool {
    age >= 30
}

/// Builds a `User` and validates its name with a closure.
fn main() {
    let person_1 = User {
        name: String::from("Alice"),
        age: 30,
        salary: 60000,
    };

    // Closure replacing `validate_user`. Same body, but it is a value:
    // it can be stored in a variable, passed to another function, or
    // swapped for a different rule without touching the call site.
    //
    // `name: &str` is annotated because the compiler cannot infer a
    // lifetime for a closure parameter from the body alone.
    // Captures nothing, so it is a plain `Fn`.
    let banned_user = String::from("banned User");
    // let validate_user_simple = move |name: &str| {
    //     let banned_user_name = banned_user;
    //     !name.is_empty() && name != banned_user_name
    // };
    // let validate_user_advance = |age: u8| age >= 30;
    // println!("{banned_user}"); // ERROR: `banned_user` was moved into the closure above.

    // `&person_1.name` is a `&String`; dereference coercion turns it into `&str`.
    println!(
        "User Validity: {}",
        is_valid_user(
            &person_1.name,
            &banned_user,
            person_1.age,
            validate_user_simple,
            validate_user_advance,
        )
    );

    // debug_assert!(validate_user_simple("Alice") && !validate_user_simple(""));
}
