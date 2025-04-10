# rtest
A simple and expressive unit testing framework for Rust.

## Features

- **Flexible Test Suites**: Easily organize tests into suites for better structure.
- **Customizable Hooks**: Implement `before_all`, `before_each`, `after_all`, and `after_each` hooks to set up and tear down test environments.
- **Multiple Reporters**: Choose from various reporters like `spec`, `minimal`, `json`, `json-pretty`, `rust`, `dot`, `tap`, and `list` to display test results.
- **State Management**: Share state across tests within a suite to maintain context.
- **Performance Metrics**: Measure and report test durations in nanoseconds, microseconds, milliseconds, or seconds.
- **Custom Assertions**: Integrate with custom assertion libraries to tailor test validations.
- **Test Exclusion**: Exclude specific tests or suites from running as needed.
- **Nested Suites**: Create nested test suites to mirror complex module structures.
- **Retry Mechanism**: Automatically retry failed tests to handle flaky scenarios.
- **Panic Testing**: Support for testing functions that are expected to panic.
- **Console Highlighting**: Enhanced console output with color highlighting for better readability.
- **Dynamic Testing**: Generate tests dynamically based on runtime conditions.
- **Slow Test Highlighting**: Identify and highlight tests that exceed a specified duration.
- **Macro-Free**: Write tests without relying on complex macros.
- **Human-Readable Output**: Test results are formatted for clarity and ease of understanding.

## Installation

Add `rtest` to your `Cargo.toml` under `[dev-dependencies]`:

```toml
[dev-dependencies]
rtest = "0.1.0"
```

## Usage

Here's a basic example demonstrating how to use `rtest` to test a simple function:

```rust
fn add_one(n: u64) -> u64 {
    n + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use rtest::{describe, it, expect, LabResult, NullState};

    #[test]
    fn suite() -> LabResult {
        describe("add_one()", |suite| {
            suite.it("should return 1 when passed 0", |_| {
                expect(add_one(0)).to_equal(1)
            })
            .it("should return 2 when passed 1", |_| {
                expect(add_one(1)).to_equal(2)
            });
        })
        .state(NullState)
        .milis()
        .run()
    }
}
```

To run the tests:

```sh
$ cargo test -- --nocapture
```

## License

This project is licensed under the MIT License. 
