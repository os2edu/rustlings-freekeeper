// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    /* 方式一 */
    data = data.to_uppercase();
    
    /* 方式二 */
    // let data = &data.to_uppercase();
    
    println!("{}", data);
}

/* 方式三
fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&mut data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: &mut String) {
    *data = data.to_uppercase();
    println!("{}", data);
}
*/
