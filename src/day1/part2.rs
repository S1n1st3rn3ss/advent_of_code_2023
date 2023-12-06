pub fn run(input: &str) -> i32 {
    let input: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let mut sum: i32 = 0;

    for i in &input {
        let mut temp_arr: Vec<i32> = vec![];
        for c in 0..i.len() {
            let j = &i[c..];
            if j.starts_with('1') || j.starts_with("one") { temp_arr.push(1); }
            if j.starts_with('2') || j.starts_with("two") { temp_arr.push(2); }
            if j.starts_with('3') || j.starts_with("three") { temp_arr.push(3); }
            if j.starts_with('4') || j.starts_with("four") { temp_arr.push(4); }
            if j.starts_with('5') || j.starts_with("five") { temp_arr.push(5); }
            if j.starts_with('6') || j.starts_with("six") { temp_arr.push(6); }
            if j.starts_with('7') || j.starts_with("seven") { temp_arr.push(7); }
            if j.starts_with('8') || j.starts_with("eight") { temp_arr.push(8); }
            if j.starts_with('9') || j.starts_with("nine") { temp_arr.push(9); }
        }
        sum += temp_arr[0] * 10 + &temp_arr[temp_arr.len() - 1];
    }
    sum
}