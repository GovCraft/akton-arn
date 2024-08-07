# Acton ERN (Entity Resource Name)

[![Crates.io](https://img.shields.io/crates/v/acton-ern.svg)](https://crates.io/crates/acton-ern)
[![Documentation](https://docs.rs/acton-ern/badge.svg)](https://docs.rs/acton-ern)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Overview

The `acton-ern` crate provides a robust, type-safe implementation for handling Acton Entity Resource Names (ERNs) within the Acton distributed actor framework. ERNs are structured identifiers used to uniquely identify and manage hierarchical resources across different services and partitions in Acton-based cloud-native solutions.

## Table of Contents

- [Installation](#installation)
- [ERN Structure](#ern-structure)
- [Basic Usage](#basic-usage)
- [Advanced Usage](#advanced-usage)
    - [Building ERNs](#building-erns)
    - [Parsing ERNs](#parsing-erns)
    - [Manipulating ERNs](#manipulating-erns)
- [ERN Components](#ern-components)
- [ID Types](#id-types)
- [Error Handling](#error-handling)
- [Best Practices](#best-practices)
- [Contributing](#contributing)
- [License](#license)

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
acton-ern = "2.0.0-alpha"
```

## ERN Structure

An Acton ERN follows this structure:

```
ern:domain:category:account:root/path/to/resource
```

- `ern`: Prefix indicating an Acton Entity Resource Name
- `domain`: Classifies the resource (e.g., "acton-internal", "acton-external")
- `category`: Specifies the service within the Acton system
- `account`: Identifies the owner or account responsible for the resource
- `root`: A unique identifier for the root of the resource hierarchy
- `path`: Optional path-like structure showing the resource's position within the hierarchy

## Basic Usage

Here's a simple example of creating and using an ERN:

```rust
use acton_ern::prelude::*;

fn main() -> Result<(), ErnError> {
    // Create an ERN
    let ern: Ern<UnixTime> = ErnBuilder::new()
        .with::<Domain>("acton-internal")?
        .with::<Category>("hr")?
        .with::<Account>("company123")?
        .with::<Root>("employees")?
        .with::<Part>("department_a")?
        .with::<Part>("team1")?
        .build()?;

    // Convert ERN to string
    println!("ERN: {}", ern);

    // Parse an ERN string
    let ern_str = "ern:acton-internal:hr:company123:employees/department_a/team1";
    let parsed_ern: Ern<UnixTime> = ErnParser::new(ern_str).parse()?;

    assert_eq!(ern, parsed_ern);

    Ok(())
}
```

## Advanced Usage

### Building ERNs

The `ErnBuilder` provides a fluent interface for constructing ERNs:

```rust
use acton_ern::prelude::*;

fn create_ern() -> Result<Ern<UnixTime>, ErnError> {
    ErnBuilder::new()
        .with::<Domain>("acton-external")?
        .with::<Category>("iot")?
        .with::<Account>("device_manufacturer")?
        .with::<Root>("sensors")?
        .with::<Part>("region1")?
        .with::<Part>("building5")?
        .with::<Part>("floor3")?
        .with::<Part>("device42")?
        .build()
}
```

### Parsing ERNs

Use `ErnParser` to parse ERN strings:

```rust
use acton_ern::prelude::*;

fn parse_ern(ern_str: &str) -> Result<Ern<UnixTime>, ErnError> {
    ErnParser::new(ern_str).parse()
}
```

### Manipulating ERNs

ERNs can be manipulated after creation:

```rust
use acton_ern::prelude::*;

fn manipulate_ern(ern: &Ern<UnixTime>) -> Result<Ern<UnixTime>, ErnError> {
    // Add a new part
    let new_ern = ern.add_part("new_subsystem")?;

    // Change the root
    let new_root_ern = ern.with_new_root("new_root")?;

    // Combine ERNs
    let combined_ern = ern.clone() + new_ern;

    Ok(combined_ern)
}
```

## ERN Components

The `acton-ern` crate provides separate types for each ERN component:

- `Domain`: Represents the domain of the resource
- `Category`: Specifies the service category
- `Account`: Identifies the account or owner
- `Root<T>`: Represents the root of the resource hierarchy
- `Part`: Represents a single part in the resource path
- `Parts`: A collection of `Part`s

Each component can be created and manipulated individually:

```rust
use acton_ern::prelude::*;

fn work_with_components() -> Result<(), ErnError> {
    let domain = Domain::new("acton-internal")?;
    let category = Category::new("finance");
    let account = Account::new("acme_corp");
    let root: Root<UnixTime> = Root::new("transactions")?;
    let part = Part::new("2023")?;
    let parts = Parts::new(vec![part]);

    Ok(())
}
```

## ID Types

The `acton-ern` crate supports different ID types for the `Root` component:

- `Random`: Uses UUID v4 (random)
- `SHA1Name`: Uses UUID v5 (SHA1 hash)
- `Timestamp`: Uses UUID v6 (timestamp-based)
- `UnixTime`: Uses UUID v7 (Unix timestamp-based)
- `UserDefined`: Uses UUID v8 (user-defined)

Choose the appropriate ID type based on your requirements:

```rust
use acton_ern::prelude::*;

fn create_erns_with_different_id_types() -> Result<(), ErnError> {
    let random_ern: Ern<Random> = Ern::with_root("random_root")?;
    let sha1_ern: Ern<SHA1Name> = Ern::with_root("sha1_root")?;
    let timestamp_ern: Ern<Timestamp> = Ern::with_root("timestamp_root")?;
    let unix_time_ern: Ern<UnixTime> = Ern::with_root("unix_time_root")?;
    let user_defined_ern: Ern<UserDefined> = Ern::with_root("user_defined_root")?;

    Ok(())
}
```

## Error Handling

The crate uses a custom `ErnError` type for error handling. Always check for and handle potential errors when working with ERNs:

```rust
use acton_ern::prelude::*;

fn handle_ern_errors() {
    match ErnBuilder::new().with::<Domain>("").build() {
        Ok(ern) => println!("Created ERN: {}", ern),
        Err(ErnError::ParseFailure(component, msg)) => {
            eprintln!("Failed to parse {}: {}", component, msg);
        }
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
```

## Best Practices

1. Use the builder pattern (`ErnBuilder`) for creating new ERNs.
2. Parse ERN strings using `ErnParser` to ensure validity.
3. Choose appropriate ID types based on your use case (e.g., `UnixTime` for timestamp-based IDs).
4. Handle all potential errors using the `ErnError` type.
5. Use the provided component types (`Domain`, `Category`, etc.) for type safety.
6. Leverage the `is_child_of` and `parent` methods for working with hierarchical ERNs.

## Contributing

Contributions to `acton-ern` are welcome! Please refer to the [Acton Framework GitHub repository](https://github.com/GovCraft/acton-framework) for contribution guidelines.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.