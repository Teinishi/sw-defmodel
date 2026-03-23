# sw-defmodel

**THIS IS WIP**

Stormworks save files, asset files, and definitions are stored in XML and proprietary binary formats, which can be cumbersome to manipulate directly.
This library provides a safe and ergonomic wrapper for working with Stormworks data structures programmatically.

It offers typed access to Stormworks XML files while preserving unknown elements and attributes so as not to break data if future Stormworks updates add new attributes or elements. It is suitable for tooling, editors, and automated modifications.

## Features

- Typed API for Stormworks XML structures
- Round-trip safe editing (unknown elements and attributes preserved)
- Ergonomic access to lists and items
- Designed for tooling and automation
- Architecture prepared for Stormworks binary formats

## Development Note

### Testing

Create a `test_data` directory and put test data in it. It is listed in `.gitignore`.

- Put the XML files from `Stormworks/rom/data/definitions` into `test_data/vanilla_definitions`.

### Generating Schema From XML Files

Run following command, then you get code in `tmp` directory.

```sh
cargo run --bin xml_analyzer --features="tool"
```
