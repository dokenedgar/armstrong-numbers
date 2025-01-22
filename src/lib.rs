pub fn is_armstrong_number(num: u32) -> bool {
    let mut total = 0;
    let length = num.to_string().len() as u32;
    for item in num.to_string().chars() {
        total += item.to_digit(10).unwrap().pow(length);
    }
    total == num
}
