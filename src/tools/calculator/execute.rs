pub fn calculate(operator: &str, first_number: f64, second_number: f64) -> Result<f64, String> {
    match operator {
        "add" => Ok(first_number + second_number),
        "subtract" => Ok(first_number - second_number),
        "multiply" => Ok(first_number * second_number),
        "divide" => {
            if second_number == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(first_number / second_number)
            }
        }
        _ => Err(format!("Unsupported operator: {}", operator)),
    }
}
