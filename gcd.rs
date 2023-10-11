// Function to find GCD using iterative method
fn gcd_iterative(mut num1: u32, mut num2: u32) -> u32 {
    while num2 != 0 {
        let temp = num2;
        num2 = num1 % num2;
        num1 = temp;
    }
    num1
}

// Function to find GCD using recursive method
fn gcd_recursive(num1: u32, num2: u32) -> u32 {
    if num2 == 0 {
        num1
    } else {
        gcd_recursive(num2, num1 % num2)
    }
}

fn main() {
    let num1 = 48;
    let num2 = 18;

    let gcd_iter = gcd_iterative(num1, num2);
    let gcd_rec = gcd_recursive(num1, num2);

    println!("GCD using iterative method: {}", gcd_iter);
    println!("GCD using recursive method: {}", gcd_rec);
}