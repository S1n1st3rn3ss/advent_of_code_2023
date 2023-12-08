pub fn run(input: &str) -> u32 {
    let lines: _ = input.trim().lines();
    let mut answer: u32 = 0;
    let mut digits: Vec<u8> = vec![];
    for line in lines {
        for byte in line.bytes() {
            // b'0' = ASCII 0
            if byte - b'0' < 10 {
                &digits.push(byte - b'0');
            }
        }
        answer += (&digits[0] * 10 + &digits[&digits.len() - 1]) as u32;
        digits.clear()
    }
    answer
}