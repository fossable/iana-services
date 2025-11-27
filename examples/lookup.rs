use iana_services::lookup_by_port;
#[cfg(feature = "lookup-by-name")]
use iana_services::lookup_by_name;

fn main() {
    #[cfg(feature = "embed")]
    println!("Running with EMBED mode (IANA registry)\n");

    #[cfg(not(feature = "embed"))]
    println!("Running with DEFAULT mode (/etc/services)\n");
    // Test port lookups
    println!("=== Port Lookups ===");

    if let Some(services) = lookup_by_port(80) {
        println!("\nPort 80:");
        for service in &services {
            #[cfg(feature = "optional-info")]
            println!("  {} ({:?}): {}", service.name, service.protocol, service.description);

            #[cfg(not(feature = "optional-info"))]
            println!("  {} ({:?})", service.name, service.protocol);
        }
    }

    if let Some(services) = lookup_by_port(443) {
        println!("\nPort 443:");
        for service in &services {
            #[cfg(feature = "optional-info")]
            println!("  {} ({:?}): {}", service.name, service.protocol, service.description);

            #[cfg(not(feature = "optional-info"))]
            println!("  {} ({:?})", service.name, service.protocol);
        }
    }

    if let Some(services) = lookup_by_port(22) {
        println!("\nPort 22:");
        for service in &services {
            #[cfg(feature = "optional-info")]
            println!("  {} ({:?}): {}", service.name, service.protocol, service.description);

            #[cfg(not(feature = "optional-info"))]
            println!("  {} ({:?})", service.name, service.protocol);
        }
    }

    // Test name lookups
    #[cfg(feature = "lookup-by-name")]
    {
        println!("\n=== Name Lookups ===");

        if let Some(services) = lookup_by_name("http") {
            println!("\nService 'http':");
            for service in &services {
                #[cfg(feature = "optional-info")]
                println!("  Port {} ({:?}): {}", service.port, service.protocol, service.description);

                #[cfg(not(feature = "optional-info"))]
                println!("  Port {} ({:?})", service.port, service.protocol);
            }
        }

        if let Some(services) = lookup_by_name("ssh") {
            println!("\nService 'ssh':");
            for service in &services {
                #[cfg(feature = "optional-info")]
                println!("  Port {} ({:?}): {}", service.port, service.protocol, service.description);

                #[cfg(not(feature = "optional-info"))]
                println!("  Port {} ({:?})", service.port, service.protocol);
            }
        }
    }

    // Test edge cases
    println!("\n=== Edge Cases ===");

    match lookup_by_port(65000) {
        Some(_) => println!("\nPort 65000: Found (unexpected)"),
        None => println!("\nPort 65000: Not found (expected)"),
    }

    #[cfg(feature = "lookup-by-name")]
    match lookup_by_name("nonexistent-service") {
        Some(_) => println!("Service 'nonexistent-service': Found (unexpected)"),
        None => println!("Service 'nonexistent-service': Not found (expected)"),
    }
}
