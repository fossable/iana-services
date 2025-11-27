//! IANA Service Names and Port Numbers Registry
//!
//! This crate provides access to service name and port number mappings from either
//! the IANA registry (when built with the `embed` feature) or the system's `/etc/services`
//! file (default).
//!
//! # Features
//!
//! - **default**: Parse `/etc/services` at runtime (no build-time dependencies, ~125 KB)
//! - **embed**: Fetch and embed the complete IANA registry at compile time (~6 MB, requires internet during build)
//! - **optional-info**: Include description and extended metadata fields with embed mode (~15 MB total)
//! - **lookup-by-name**: Enable the `lookup_by_name` function and associated data (reduces size when only port lookups are needed)
//!
//! # Examples
//!
//! ```toml
//! # Minimal: Runtime parsing
//! iana-services = "0.1.0"
//!
//! # Embedded: Core fields only
//! iana-services = { version = "0.1.0", features = ["embed"] }
//!
//! # Embedded with name lookups
//! iana-services = { version = "0.1.0", features = ["embed", "lookup-by-name"] }
//!
//! # Full: With extended metadata
//! iana-services = { version = "0.1.0", features = ["embed", "optional-info", "lookup-by-name"] }
//! ```
//!
//! ```
//! use iana_services::lookup_by_port;
//! #[cfg(feature = "lookup-by-name")]
//! use iana_services::lookup_by_name;
//!
//! // Look up services by port number
//! if let Some(services) = lookup_by_port(80) {
//!     for service in &services {
//!         println!("Port 80: {} ({:?})", service.name, service.protocol);
//!     }
//! }
//!
//! // Look up services by name (requires lookup-by-name feature)
//! #[cfg(feature = "lookup-by-name")]
//! if let Some(services) = lookup_by_name("http") {
//!     for service in &services {
//!         println!("HTTP runs on port {}", service.port);
//!     }
//! }
//! ```

/// Transport protocol for a service
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransportProtocol {
    /// Transmission Control Protocol
    Tcp,
    /// User Datagram Protocol
    Udp,
}

/// A service record
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceRecord {
    /// Service name (may be empty for reserved/unassigned ports)
    #[cfg(feature = "embed")]
    pub name: &'static str,
    #[cfg(not(feature = "embed"))]
    pub name: String,

    /// Port number
    pub port: u16,

    /// Transport protocol (TCP or UDP)
    pub protocol: TransportProtocol,

    /// Description of the service
    #[cfg(all(feature = "embed", feature = "optional-info"))]
    pub description: &'static str,
    #[cfg(all(not(feature = "embed"), feature = "optional-info"))]
    pub description: String,

    /// Organization or person to whom the port is assigned
    #[cfg(all(feature = "embed", feature = "optional-info"))]
    pub assignee: Option<&'static str>,
    #[cfg(all(not(feature = "embed"), feature = "optional-info"))]
    pub assignee: Option<String>,

    /// Contact information for the assignee
    #[cfg(all(feature = "embed", feature = "optional-info"))]
    pub contact: Option<&'static str>,
    #[cfg(all(not(feature = "embed"), feature = "optional-info"))]
    pub contact: Option<String>,

    /// Date the service was registered
    #[cfg(all(feature = "embed", feature = "optional-info"))]
    pub registration_date: Option<&'static str>,
    #[cfg(all(not(feature = "embed"), feature = "optional-info"))]
    pub registration_date: Option<String>,

    /// Date the service record was last modified
    #[cfg(all(feature = "embed", feature = "optional-info"))]
    pub modification_date: Option<&'static str>,
    #[cfg(all(not(feature = "embed"), feature = "optional-info"))]
    pub modification_date: Option<String>,

    /// Reference documentation (usually RFC numbers)
    #[cfg(all(feature = "embed", feature = "optional-info"))]
    pub reference: Option<&'static str>,
    #[cfg(all(not(feature = "embed"), feature = "optional-info"))]
    pub reference: Option<String>,

    /// Service code
    #[cfg(all(feature = "embed", feature = "optional-info"))]
    pub service_code: Option<&'static str>,
    #[cfg(all(not(feature = "embed"), feature = "optional-info"))]
    pub service_code: Option<String>,

    /// Whether unauthorized use has been reported
    #[cfg(all(feature = "embed", feature = "optional-info"))]
    pub unauthorized_use: Option<&'static str>,
    #[cfg(all(not(feature = "embed"), feature = "optional-info"))]
    pub unauthorized_use: Option<String>,

    /// Additional notes about the assignment
    #[cfg(all(feature = "embed", feature = "optional-info"))]
    pub assignment_notes: Option<&'static str>,
    #[cfg(all(not(feature = "embed"), feature = "optional-info"))]
    pub assignment_notes: Option<String>,
}

#[cfg(feature = "embed")]
mod embedded {
    use super::*;

    include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

    pub fn lookup_by_port_impl(port: u16) -> Option<Vec<ServiceRecord>> {
        BY_PORT.get(&port).map(|(start, end)| {
            SERVICE_RECORDS[*start..*end].to_vec()
        })
    }

    #[cfg(feature = "lookup-by-name")]
    pub fn lookup_by_name_impl(name: &str) -> Option<Vec<ServiceRecord>> {
        BY_NAME.get(name).map(|indices| {
            indices.iter().map(|&idx| SERVICE_RECORDS[idx].clone()).collect()
        })
    }
}

#[cfg(not(feature = "embed"))]
mod runtime {
    use super::*;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn parse_services_file<F>(mut callback: F) -> std::io::Result<()>
    where
        F: FnMut(String, u16, TransportProtocol, String) -> bool,
    {
        let file = File::open("/etc/services")?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();

            // Skip comments and empty lines
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse line format: service port/protocol [aliases] [#comment]
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let name = parts[0].to_string();

            // Parse port/protocol
            let port_proto: Vec<&str> = parts[1].split('/').collect();
            if port_proto.len() != 2 {
                continue;
            }

            let port: u16 = match port_proto[0].parse() {
                Ok(p) => p,
                Err(_) => continue,
            };

            let protocol = match port_proto[1].to_lowercase().as_str() {
                "tcp" => TransportProtocol::Tcp,
                "udp" => TransportProtocol::Udp,
                _ => continue,
            };

            // Extract description (everything after first # or remaining parts as aliases)
            let description = if let Some(comment_pos) = line.find('#') {
                line[comment_pos + 1..].trim().to_string()
            } else if parts.len() > 2 {
                parts[2..].join(" ")
            } else {
                String::new()
            };

            // Call callback, if it returns false, stop iteration
            if !callback(name, port, protocol, description) {
                break;
            }
        }

        Ok(())
    }

    pub fn lookup_by_port_impl(target_port: u16) -> Option<Vec<ServiceRecord>> {
        let mut results = Vec::new();

        let _ = parse_services_file(|name, port, protocol, _description| {
            if port == target_port {
                results.push(ServiceRecord {
                    name,
                    port,
                    protocol,
                    #[cfg(feature = "optional-info")]
                    description,
                    #[cfg(feature = "optional-info")]
                    assignee: None,
                    #[cfg(feature = "optional-info")]
                    contact: None,
                    #[cfg(feature = "optional-info")]
                    registration_date: None,
                    #[cfg(feature = "optional-info")]
                    modification_date: None,
                    #[cfg(feature = "optional-info")]
                    reference: None,
                    #[cfg(feature = "optional-info")]
                    service_code: None,
                    #[cfg(feature = "optional-info")]
                    unauthorized_use: None,
                    #[cfg(feature = "optional-info")]
                    assignment_notes: None,
                });
            }
            true // Continue searching
        });

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }

    #[cfg(feature = "lookup-by-name")]
    pub fn lookup_by_name_impl(target_name: &str) -> Option<Vec<ServiceRecord>> {
        let mut results = Vec::new();

        let _ = parse_services_file(|name, port, protocol, _description| {
            if name == target_name {
                results.push(ServiceRecord {
                    name,
                    port,
                    protocol,
                    #[cfg(feature = "optional-info")]
                    description,
                    #[cfg(feature = "optional-info")]
                    assignee: None,
                    #[cfg(feature = "optional-info")]
                    contact: None,
                    #[cfg(feature = "optional-info")]
                    registration_date: None,
                    #[cfg(feature = "optional-info")]
                    modification_date: None,
                    #[cfg(feature = "optional-info")]
                    reference: None,
                    #[cfg(feature = "optional-info")]
                    service_code: None,
                    #[cfg(feature = "optional-info")]
                    unauthorized_use: None,
                    #[cfg(feature = "optional-info")]
                    assignment_notes: None,
                });
            }
            true // Continue searching
        });

        if results.is_empty() {
            None
        } else {
            Some(results)
        }
    }
}

/// Look up services by port number
///
/// Returns all service records (both TCP and UDP) associated with the given port number.
///
/// # Examples
///
/// ```
/// use iana_services::lookup_by_port;
///
/// if let Some(services) = lookup_by_port(22) {
///     for service in &services {
///         println!("Port 22: {} over {:?}", service.name, service.protocol);
///     }
/// }
/// ```
pub fn lookup_by_port(port: u16) -> Option<Vec<ServiceRecord>> {
    #[cfg(feature = "embed")]
    return embedded::lookup_by_port_impl(port);

    #[cfg(not(feature = "embed"))]
    return runtime::lookup_by_port_impl(port);
}

/// Look up services by service name
///
/// Returns all service records (across all protocols and ports) with the given name.
///
/// The returned vector contains service records. Note that service names
/// may map to multiple ports and protocols.
///
/// # Examples
///
/// ```
/// use iana_services::lookup_by_name;
///
/// if let Some(services) = lookup_by_name("ssh") {
///     for service in &services {
///         println!("SSH: port {} over {:?}", service.port, service.protocol);
///     }
/// }
/// ```
#[cfg(feature = "lookup-by-name")]
pub fn lookup_by_name(name: &str) -> Option<Vec<ServiceRecord>> {
    #[cfg(feature = "embed")]
    return embedded::lookup_by_name_impl(name);

    #[cfg(not(feature = "embed"))]
    return runtime::lookup_by_name_impl(name);
}
