# JSON Parser in Rust

A simple, handwritten JSON parser implemented in Rust using `cargo`.
This project demonstrates how to parse JSON without relying on external libraries, using a recursive descent parsing approach.

---

## Features

* Supports all standard JSON types:

  * `null`
  * `boolean`
  * `number` (floating point)
  * `string` (with escape sequences and Unicode support)
  * `array`
  * `object`
* Proper whitespace handling
* Clear error messages for invalid input
* Written using safe, idiomatic Rust
* No external dependencies

---

## Project Structure

```
.
├── src
│   └── main.rs
├── Cargo.toml
└── README.md
```

---

## JSON Representation

Parsed JSON values are represented using the `JsonValue` enum:

```rust
pub enum JsonValue {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}
```

---

## How It Works

The parser:

* Iterates over characters using `Peekable<Chars>`
* Skips unnecessary whitespace
* Recursively parses JSON values
* Converts input into structured Rust data types

Each JSON type has its own parsing function:

* `parse_object`
* `parse_array`
* `parse_string`
* `parse_number`
* `parse_bool`
* `parse_null`

---

## Running the Project

### 1. Build the project

```bash
cargo build
```

### 2. Run the program

```bash
cargo run
```

### Example Output

Input JSON:

```json
{ "key": "value", "list": [1, 2, 3] }
```

Output:

```
Success!
{"key": String("value"), "list": Array([Number(1.0), Number(2.0), Number(3.0)])}
```

---

## Example Usage (from `main.rs`)

```rust
fn main() {
    let json = r#" { "key": "value", "list": [1, 2, 3] } "#;
    let mut parser = Parser::new(json);

    let result = match parser.parse() {
        Ok(JsonValue::Object(map)) => {
            println!("Success!");
            map
        }
        Ok(_) => {
            println!("Parsed valid JSON, but it wasn't an Object.");
            HashMap::new()
        }
        Err(e) => {
            println!("Error: {}", e);
            HashMap::new()
        }
    };

    println!("{:?}", result);
}
```

---

## Limitations

* Does not preserve key order (`HashMap`)
* Floating-point numbers only (`f64`)
* No streaming or incremental parsing
* No error position tracking (line/column)

---

## Possible Improvements

* Add line/column error reporting
* Support for large numbers or decimals
* Replace `HashMap` with `IndexMap` for deterministic ordering
* Implement a tokenizer (lexer) layer
* Add unit tests

---

## License

This project is provided for educational purposes.
You may use, modify, and distribute it freely.
