use std::error::Error;

pub fn run(input: &String) -> Result<u32, Box<dyn Error>> {
    let mut stack: Vec<u32> = Vec::new();

    for token in input.chars() {

        // If the value is an integer
        if token.is_digit(10) {

            // Push to the vector
            stack.push(token.to_digit(10).unwrap());

        } else if !token.is_whitespace() {

            // Otherwise evaluate the expression provided the current token is not a whitespace character
            let rhs:u32 = stack.pop().unwrap();
            let lhs:u32 = stack.pop().unwrap();

            // ... and pop the result back to the vector
            match token {
                '+' => stack.push(lhs + rhs),
                '-' => stack.push(lhs - rhs),
                '*' => stack.push(lhs * rhs),
                '/' => stack.push(lhs / rhs),
                '%' => stack.push(lhs % rhs),
                _   => panic!("An error occurred")
            }
        }
    }

    let result: u32 = stack.pop().unwrap();
    println!("Result of calulation on {} is {}", input, result);
    Ok(result)
}