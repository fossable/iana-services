# `iana-services`

A Rust crate that provides access to IANA service name to port number mappings.

## Features

By default, `iana-services` just parses `/etc/services` at runtime and provides
the data as Rust structs. This leads to small impact on binary size, but means
the target system needs to have a `/etc/services` suitable for your needs. Some
systems may have a "reduced" services file to save on storage.

### Compile-Time Embedding

The `embed` feature causes the IANA registry to be included in the program's
binary artifact. This bloats your final binary, but uses perfect hash functions
to ensure `O(1)` lookups, as least.

## Usage

Add one of these to your `Cargo.toml` dependencies:

```toml
# Parses /etc/services at runtime (adds ~125 KB to binary size)
# Has: name, port, protocol
iana-services = "0.1.0"

# Embeds a reduced IANA registry (adds ~6 MB to binary size)
# Has: name, port, protocol
iana-services = { version = "0.1.0", features = ["embed"] }

# Embeds the full IANA registry with all fields (adds ~15 MB to binary size)
# Has: name, port, protocol, description, assignee, contact, dates, references, notes
iana-services = { version = "0.1.0", features = ["embed", "optional-info"] }
```

## Examples

### Look up by port

```rust
use iana_services::lookup_by_port;

if let Some(services) = lookup_by_port(80) {
    for service in services {
        println!("{} ({:?}): {}", service.name, service.protocol, service.description);
    }
}
```

### Look up by name

```rust
use iana_services::lookup_by_name;

if let Some(services) = lookup_by_name("http") {
    for service in &services {
        println!("HTTP runs on port {} over {:?}", service.port, service.protocol);
    }
}
```
