use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Calculator;

#[derive(Deserialize)]
pub struct CalculatorArgs {
    pub operation: String,
    pub a: f64,
    pub b: f64,
}

#[derive(Debug, thiserror::Error)]
pub enum CalculatorError {
    #[error("Calculation error: {0}")]
    Calculation(String),
}

impl Tool for Calculator {
    const NAME: &'static str = "calculator";
    type Error = CalculatorError;
    type Args = CalculatorArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: "calculator".to_string(),
            description: "Perform basic arithmetic operations: add, subtract, multiply, divide".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "enum": ["add", "subtract", "multiply", "divide"],
                        "description": "The arithmetic operation to perform"
                    },
                    "a": { "type": "number", "description": "The first operand" },
                    "b": { "type": "number", "description": "The second operand" }
                },
                "required": ["operation", "a", "b"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = match args.operation.as_str() {
            "add" => args.a + args.b,
            "subtract" => args.a - args.b,
            "multiply" => args.a * args.b,
            "divide" => {
                if args.b == 0.0 {
                    return Err(CalculatorError::Calculation("Division by zero error".to_string()));
                }
                args.a / args.b
            }
            _ => return Err(CalculatorError::Calculation(format!("Unsupported operation: {}", args.operation))),
        };
        Ok(result.to_string())
    }
}
