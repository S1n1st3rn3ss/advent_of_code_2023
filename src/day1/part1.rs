// pub fn run(input: &str) -> u32 {
//     let input: Vec<&str> = input.lines().collect();
//     for i in &input {
//         let mut temp_arr: Vec<u32> = vec![];
//         for c in 0..i.len() {
//             let j: char = i.bytes()[c];
//             if j.is_numeric()  {
//                 temp_arr.push(j.to_digit(10).expect("is digit"));
//             }
//         }
//         return temp_arr[0] * 10 + &temp_arr[temp_arr.len() - 1];
//     }
//     0
// }