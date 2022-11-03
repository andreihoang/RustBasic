/////////////////////////////////////
// Bài 1
// Sửa code để compile thành công liên quan tới vấn đề lifetime
/////////////////////////////////////

use std::io;
fn main() {
    let mut input: Vec<&str>;
    loop {
        let mut input_text = String::new();
        println!("Type instruction in the format Add <name> to <department>:");
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");
        let trimmed_text: String = input_text.trim().to_string();
        input = trim(&trimmed_text);

        if input[0] == "Add" && input[2] == "to" {
            break;
        } else {
            println!("Invalid format.");
        }
    }
    println!("{:?}", input);
}

fn trim<'a>(x: &'a str) -> Vec<&'a str> {
    let vec: Vec<&'a str> = x.split("").collect();
    vec
}
