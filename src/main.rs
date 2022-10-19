// fn findSubArray(org_arr: &mut [i32], sub_arr: &mut [i32]) -> bool {
//     let sub_arr_first_item = sub_arr[0];
//     let mut p1 = 0;
//     for (i, elem) in org_arr.iter().enumerate() {
//         if org_arr[i] == sub_arr_first_item {
//             p1 = i;
//         }
//     }
//     let mut p2 = 0;
//     let mut is_sub = true;
//     while p2 < sub_arr.len() {
//         if org_arr[p1] != sub_arr[p2] {
//             is_sub = false;
//         }
//         p1 = p1 + 1;
//         p2 = p2 + 1;
//     }

//     return is_sub;
// }
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
        println!("True");
    } else {
        println!("False");
    }
}
