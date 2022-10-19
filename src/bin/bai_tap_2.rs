//Bài tập 2 : Cho 1 chuỗi ký tự, nhập 1 ký tự từ bàn phím trả về số lần xuất hiện của từ đó trong chuỗi đã cho, và chuỗi không chứa ký tự nhập từ bàn phím. Lưu ý: khong phân biệt viết hoa, viết thường
//Ví dụ: let input = “adbcdaDd”.
//Nhập s = ‘a’ => in ra kết quả : 2, “dbcdDd”
//Nhập s = ‘d’ => in ra kết quả : 4, “abca”

use std::io::stdin;

fn bai_tap_2_result(input_string: &mut String) {
    let mut input_char: String = String::new();
    stdin().read_line(&mut input_char).unwrap();
    input_char.truncate(input_char.len() - 1);
    let mut count: i32 = 0;
    for elem in input_string.chars() {
        if elem.to_string().eq(&input_char) {
            count = count + 1;
            elem.to_lowercase();
        }
    }

    let s2 = str::replace(&input_string, &input_char, "");
    println!("{}, {:?}", count, s2);
}

fn main() {
    let mut input_string: String = String::from("adbcdaDd");
    bai_tap_2_result(&mut input_string);
}
