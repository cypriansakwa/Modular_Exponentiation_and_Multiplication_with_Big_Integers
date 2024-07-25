use num::bigint::BigInt;
use num::traits::Pow;

fn main() {
    // Initialize BigInt values for base, exponent, and modulus
    let base1 = BigInt::from(19);
    let exp1 = 4_u32;
    let modulus1 = BigInt::from(5);

    let base2 = BigInt::from(288);
    let exp2 = 8_u32;
    let modulus2 = BigInt::from(5);

    // Calculate (base1^exp1) % modulus1
    let result1 = base1.pow(exp1 as usize) % &modulus1;

    // Calculate (base2^exp2) % modulus2
    let result2 = base2.pow(exp2 as usize) % &modulus2;

    // Multiply the results and take the modulus again
    let final_result = (result1 * result2) % modulus1.clone(); // Since modulus1 == modulus2

    println!("The result is: {}", final_result);
}




