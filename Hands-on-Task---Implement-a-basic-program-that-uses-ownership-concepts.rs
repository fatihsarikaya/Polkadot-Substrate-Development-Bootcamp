// Function signature for concatenate_strings
fn concatenate_strings(string1: &str, string2: &str) -> String {
    let mut result = String::new();
    result.push_str(string1);
    result.push_str(string2);
    result
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("Rust!");

    // Call the concatenate_strings function with string slices
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the result to the console
    println!("{}", concatenated_string);
}
