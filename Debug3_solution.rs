/*!
    This is the solution of our problem. We simply had
    to replace our operator from a multiplication (*) sign
    to an addition (+) sign. Now we add all of our 
    modulus 3 and 5 elements from our vector together 
    in our for loop.
*/

fn main() {
    println!("Printing multiples all numbers divisible between 1 and 1000 of 3 and 5");
    let mut numbers: Vec<u128> = Vec::new();
    
    for number in 1..1000 {
        if (number % 5 == 0) || (number % 3 == 0) {
            numbers.push(number);
        }
    }

    let mut result: u128 = 0;
    for number in &numbers {
        result = result + number;
    }
    println!("Result: {}", result);
}
