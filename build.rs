fn main() {
    #[cfg(feature = "embed")]
    build_embedded();
}

#[cfg(feature = "embed")]
fn build_embedded() {
    use std::collections::HashMap;
    use std::env;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::Path;
    println!("cargo:rerun-if-changed=build.rs");

    // Fetch IANA service names CSV
    let url = "https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.csv";
    let client = reqwest::blocking::Client::builder()
        .user_agent("iana-services-rust-crate/0.1.0")
        .build()
        .expect("Failed to build HTTP client");
    let response = client
        .get(url)
        .send()
        .expect("Failed to fetch IANA services file")
        .text()
        .expect("Failed to read response body");

    // Parse CSV
    let mut csv_reader = csv::Reader::from_reader(response.as_bytes());

    // Group records by port and by name
    let mut by_port: HashMap<u16, Vec<ServiceEntry>> = HashMap::new();
    let mut by_name: HashMap<String, Vec<ServiceEntry>> = HashMap::new();

    for result in csv_reader.records() {
        let record = result.expect("Failed to parse CSV record");

        // Extract fields
        let service_name = record.get(0).unwrap_or("").trim().to_string();
        let port_str = record.get(1).unwrap_or("").trim();
        let protocol_str = record.get(2).unwrap_or("").trim().to_lowercase();
        let description = record.get(3).unwrap_or("").trim().to_string();
        let assignee = record.get(4).and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });
        let contact = record.get(5).and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });
        let registration_date = record.get(6).and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });
        let modification_date = record.get(7).and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });
        let reference = record.get(8).and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });
        let service_code = record.get(9).and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });
        let unauthorized_use = record.get(10).and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });
        let assignment_notes = record.get(11).and_then(|s| {
            if s.trim().is_empty() {
                None
            } else {
                Some(s.trim().to_string())
            }
        });

        // Skip entries without port numbers or with port ranges
        let port: u16 = match port_str.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        // Parse protocol
        let protocol = match protocol_str.as_str() {
            "tcp" => "TransportProtocol::Tcp",
            "udp" => "TransportProtocol::Udp",
            _ => continue, // Skip unknown protocols
        };

        let entry = ServiceEntry {
            name: service_name.clone(),
            port,
            protocol: protocol.to_string(),
            description,
            assignee,
            contact,
            registration_date,
            modification_date,
            reference,
            service_code,
            unauthorized_use,
            assignment_notes,
        };

        // Add to port map
        by_port.entry(port).or_default().push(entry.clone());

        // Add to name map (only if name is not empty)
        if !service_name.is_empty() {
            by_name.entry(service_name).or_default().push(entry);
        }
    }

    // Generate code
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    // Generate service records as static data
    writeln!(&mut file, "static SERVICE_RECORDS: &[ServiceRecord] = &[").unwrap();
    let mut all_entries: Vec<_> = by_port.values().flat_map(|v| v.iter()).collect();
    all_entries.sort_by_key(|e| (e.port, &e.name));

    for entry in &all_entries {
        writeln!(&mut file, "    ServiceRecord {{").unwrap();
        writeln!(&mut file, "        name: {:?},", entry.name).unwrap();
        writeln!(&mut file, "        port: {},", entry.port).unwrap();
        writeln!(&mut file, "        protocol: {},", entry.protocol).unwrap();

        if cfg!(feature = "optional-info") {
            writeln!(&mut file, "        #[cfg(feature = \"optional-info\")]").unwrap();
            writeln!(&mut file, "        description: {:?},", entry.description).unwrap();
            writeln!(&mut file, "        #[cfg(feature = \"optional-info\")]").unwrap();
            writeln!(
                &mut file,
                "        assignee: {},",
                option_to_code(&entry.assignee)
            )
            .unwrap();
            writeln!(&mut file, "        #[cfg(feature = \"optional-info\")]").unwrap();
            writeln!(
                &mut file,
                "        contact: {},",
                option_to_code(&entry.contact)
            )
            .unwrap();
            writeln!(&mut file, "        #[cfg(feature = \"optional-info\")]").unwrap();
            writeln!(
                &mut file,
                "        registration_date: {},",
                option_to_code(&entry.registration_date)
            )
            .unwrap();
            writeln!(&mut file, "        #[cfg(feature = \"optional-info\")]").unwrap();
            writeln!(
                &mut file,
                "        modification_date: {},",
                option_to_code(&entry.modification_date)
            )
            .unwrap();
            writeln!(&mut file, "        #[cfg(feature = \"optional-info\")]").unwrap();
            writeln!(
                &mut file,
                "        reference: {},",
                option_to_code(&entry.reference)
            )
            .unwrap();
            writeln!(&mut file, "        #[cfg(feature = \"optional-info\")]").unwrap();
            writeln!(
                &mut file,
                "        service_code: {},",
                option_to_code(&entry.service_code)
            )
            .unwrap();
            writeln!(&mut file, "        #[cfg(feature = \"optional-info\")]").unwrap();
            writeln!(
                &mut file,
                "        unauthorized_use: {},",
                option_to_code(&entry.unauthorized_use)
            )
            .unwrap();
            writeln!(&mut file, "        #[cfg(feature = \"optional-info\")]").unwrap();
            writeln!(
                &mut file,
                "        assignment_notes: {},",
                option_to_code(&entry.assignment_notes)
            )
            .unwrap();
        }

        writeln!(&mut file, "    }},").unwrap();
    }
    writeln!(&mut file, "];").unwrap();
    writeln!(&mut file).unwrap();

    // Generate PHF map for port lookup
    let mut port_map = phf_codegen::Map::new();
    let mut port_ranges: HashMap<u16, (usize, usize)> = HashMap::new();
    let mut port_values: Vec<String> = Vec::new();
    let mut offset = 0;

    for (port, entries) in &by_port {
        let count = entries.len();
        port_ranges.insert(*port, (offset, offset + count));
        offset += count;
    }

    // Collect formatted strings first to ensure they live long enough
    for (start, end) in port_ranges.values() {
        port_values.push(format!("({}, {})", start, end));
    }

    for ((port, _), value) in port_ranges.iter().zip(port_values.iter()) {
        port_map.entry(*port, value);
    }

    writeln!(
        &mut file,
        "static BY_PORT: phf::Map<u16, (usize, usize)> = {};",
        port_map.build()
    )
    .unwrap();
    writeln!(&mut file).unwrap();

    // Generate name lookup data only if lookup-by-name feature is enabled
    if cfg!(feature = "lookup-by-name") {
        // Generate static arrays for name lookup indices
        let mut name_ranges: HashMap<String, Vec<usize>> = HashMap::new();

        for (idx, entry) in all_entries.iter().enumerate() {
            if !entry.name.is_empty() {
                name_ranges.entry(entry.name.clone()).or_default().push(idx);
            }
        }

        // Generate static arrays for each service name's indices
        for (idx, (_name, indices)) in name_ranges.iter().enumerate() {
            writeln!(&mut file, "#[cfg(feature = \"lookup-by-name\")]").unwrap();
            writeln!(
                &mut file,
                "static NAME_INDICES_{}: &[usize] = &{:?};",
                idx, indices
            )
            .unwrap();
        }
        writeln!(&mut file).unwrap();

        // Generate PHF map for name lookup
        let mut name_map = phf_codegen::Map::new();
        let name_values: Vec<String> = (0..name_ranges.len())
            .map(|idx| format!("NAME_INDICES_{}", idx))
            .collect();

        for ((name, _), value) in name_ranges.iter().zip(name_values.iter()) {
            name_map.entry(name.as_str(), value);
        }

        writeln!(&mut file, "#[cfg(feature = \"lookup-by-name\")]").unwrap();
        writeln!(
            &mut file,
            "static BY_NAME: phf::Map<&'static str, &'static [usize]> = {};",
            name_map.build()
        )
        .unwrap();
    }

    fn option_to_code(opt: &Option<String>) -> String {
        match opt {
            Some(s) => format!("Some({:?})", s),
            None => "None".to_string(),
        }
    }

    #[derive(Clone)]
    struct ServiceEntry {
        name: String,
        port: u16,
        protocol: String,
        description: String,
        assignee: Option<String>,
        contact: Option<String>,
        registration_date: Option<String>,
        modification_date: Option<String>,
        reference: Option<String>,
        service_code: Option<String>,
        unauthorized_use: Option<String>,
        assignment_notes: Option<String>,
    }
}
