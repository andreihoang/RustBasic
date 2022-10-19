// Bai Tap 1:
// Cho 2 mảng, kiểm tra mảng này có phải là mảng con của mảng kia không ? (yêu cầu đúng thứ tự của các phần tử)
// Ví dụ : let org_arr = [1, 2,3,5,6,8, 10, 11];
//         let sub_arr = [6,8,10];
fn check_is_sub_array(org_arr: &mut [i32], sub_arr: &mut [i32]) -> bool {
    let org_arr_length = org_arr.len();
    let sub_arr_length = sub_arr.len();
    let mut i = 0;
    let mut j = 0;
    while i < org_arr_length && j < sub_arr_length {
        if org_arr[i] == sub_arr[j] {
            i = i + 1;
            j = j + 1;
            if j == sub_arr_length {
                return true;
            }
        } else {
            i = i - j + 1;
            j = 0;
        }
    }
    return false;
}

fn main() {
    let mut org_arr: [i32; 8] = [1, 2, 3, 5, 6, 8, 10, 11];
    let mut sub_arr: [i32; 3] = [6, 8, 10];
    let result: bool = check_is_sub_array(&mut org_arr, &mut sub_arr);
    if result {
        println!("True! It is a subarray!");
    } else {
        println!("False! It's not a subarray");
    }
}
