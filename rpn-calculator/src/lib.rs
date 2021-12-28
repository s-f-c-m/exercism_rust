#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    //    unimplemented!(
    // 	"Given the inputs: {:?}, evaluate them as though they were a Reverse Polish notation expression",
    // 	inputs,
    // );
    // let mut vec = vec![inputs];
    let mut stack: Vec<i32> = Vec::new();
    for el in inputs.iter() {
        match el {
            CalculatorInput::Value(v) => stack.push(*v),
            CalculatorInput::Add => {
                let n2 = stack.pop()?;
                let n1 = stack.pop()?;
                stack.push(n1 + n2);
            }
            CalculatorInput::Subtract => {
                let n2 = stack.pop()?;
                let n1 = stack.pop()?;
                stack.push(n1 - n2);
            }
            CalculatorInput::Multiply => {
                let n2 = stack.pop()?;
                let n1 = stack.pop()?;
                stack.push(n1 * n2);
            }
            CalculatorInput::Divide => {
                let n2 = stack.pop()?;
                let n1 = stack.pop()?;
                stack.push(n1 / n2);
            }
        }
    }
    if stack.len() > 1 {
        return None;
    }
    stack.pop()
}
