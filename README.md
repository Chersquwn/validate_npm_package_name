# validate_npm_package_name

A crate to validate npm package name

## Install

```rust
cargo add validate_npm_package_name
```

## Usage

### Valid Names

```rust
use validate_npm_package_name::validate;

validate(&String::from("some-package"));
validate(&String::from("@npm/abc"));
```

Validate Result

```rust
validate_npm_package_name::ValidateResult {
    valid_for_new_packages: true,
    valid_for_old_packages: true,
    warnings: None,
    errors: None
}
```

### Invalid Names

```rust
use validate_npm_package_name::validate;

validate(&String::from("s/l/a/s/h/e/s"));
```

Validate Result

```rust
validate_npm_package_name::ValidateResult {
    valid_for_new_packages: false,
    valid_for_old_packages: false,
    warnings: None,
    errors: Some(vec![String::from(
        "name can only contain URL-friendly characters"
    )])
}
```