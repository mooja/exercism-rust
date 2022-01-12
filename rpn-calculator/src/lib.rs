#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        match input {
            Value(n) => stack.push(*n),

            Add => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a + b);
            }

            Subtract => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b - a);
            }

            Multiply => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(a * b);
            }

            Divide => {
                let a = stack.pop()?;
                let b = stack.pop()?;
                stack.push(b / a);
            }
        }
    }

    match stack.len() {
        0 => None,
        1 => stack.pop(),
        _ => None,
    }
}
