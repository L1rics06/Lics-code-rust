use crate::{
    gaia::evaluator::CalculatorArgs,
    tools::{calculator::execute::calculate, tool::Tool},
};
use schemars::schema_for;
use serde_json::Value;

pub struct CalculatorTool;

#[async_trait::async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Perform basic arithmetic operations."
    }

    fn parameters(&self) -> Value {
        serde_json::to_value(schema_for!(CalculatorArgs))
            .expect("Failed to generate JSON schema for CalculatorArgs")
    }

    async fn execute(&self, args_json: &str) -> anyhow::Result<String> {
        let args: CalculatorArgs = serde_json::from_str(args_json)?;
        let result = calculate(&args.operator, args.first_number, args.second_number);
        match result {
            Ok(value) => Ok(value.to_string()),
            Err(err) => Err(anyhow::anyhow!("Error: {}", err)),
        }
    }
}
