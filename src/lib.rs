//! A library for creating structured output for command line applications.
//!
//!
//! # Example
//!
//! ```
//! use structured_output::StructuredOutput;
//!
//! let lines = "Hello world!";
//! let json = serde_json::json!({ "message": lines });
//! let output = StructuredOutput::new(lines, json.clone());
//! assert_eq!(output.lines(), "Hello world!");
//! assert_eq!(output.json(), &json);
//! ```
//!

#[derive(Debug, Default, Clone, PartialEq)]
/// A struct that contains both a string and a json value.
pub struct StructuredOutput {
    pub lines: String,
    pub json: serde_json::Value,
}

impl StructuredOutput {
    /// Create a new [StructuredOutput] that includes both a string and a json value.
    ///
    pub fn new(lines: impl Into<String>, json: impl Into<serde_json::Value>) -> StructuredOutput {
        StructuredOutput {
            lines: lines.into(),
            json: json.into(),
        }
    }

    /// Get the string value of the output.
    ///
    pub fn lines(&self) -> &str {
        &self.lines
    }

    /// Get the json value of the output.
    pub fn json(&self) -> &serde_json::Value {
        &self.json
    }

    /// Get the json value of the output as a JSON string.
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self.json).unwrap()
    }

    /// Get the json value of the output as a prettified JSON string.
    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(&self.json).unwrap()
    }
}

impl std::fmt::Display for StructuredOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let lines = "Hello world!";
        let json = serde_json::json!({ "message": lines });
        let output = StructuredOutput::new(lines, json.clone());
        assert_eq!(output.lines(), "Hello world!");
        assert_eq!(output.json(), &json);
    }
}
