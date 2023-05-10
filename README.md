# structured-output Rust crate

`StructuredOutput` is a Rust library that provides a struct for handling structured output with both a string and a JSON value.

## Installation

To use this library in your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
structured_output = "0.1.0"
```

## Usage

You can then create a new `StructuredOutput` instance using the `new` method, which takes a string and a JSON value as arguments:

```rust
let lines = "Hello world!";
let json = serde_json::json!({ "message": lines });
let output = StructuredOutput::new(lines, json.clone());
```

You can retrieve the string and JSON value separately using the `lines` and `json` methods:

```rust
assert_eq!(output.lines(), "Hello world!");
assert_eq!(output.json(), &json);
```

You can also get the JSON value as a JSON string or a pretty-printed JSON string using the `to_json_string` and `to_json_pretty` methods:

```rust
assert_eq!(output.to_json_string(), "{\"message\":\"Hello world!\"}");
assert_eq!(output.to_json_pretty(), "{\n  \"message\": \"Hello world!\"\n}");
```

The `StructuredOutput` implementation of the `Display` trait defaults to the string value. You can print the string value directly with the `println!` macro:

```rust
println!("{}", output);
```

## License

This library is licensed under the Apache 2.0 License.