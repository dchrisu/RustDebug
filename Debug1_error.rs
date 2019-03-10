fn main() {
    println!("Printing multiples all numbers divisible between 1 and 1000 of 3 and 5");
    let mut numbers: Vec<u128> = Vec::new();
    
    for number in 1..1000 {
        if (number % 5 == 0) || (number % 3 == 0) {
            numbers.push(number);
        }
    }

    let result: u128 = 0;
    for number in &numbers {
        result = result * number;
    }
    println!("Result: {}", result);
}
