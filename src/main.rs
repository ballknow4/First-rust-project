
use sysinfo::System;

// Converts bytes into gigabytes (GiB)
fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

fn main() {
    // Initialize system information
    let mut sys = System::new_all().expect("Failed to initialize system information");

    // Refresh system information
    sys.refresh_all();

    // CPU information
    let cpu_name = sys.cpus()
        .first()
        .map(|cpu| cpu.brand())
        .unwrap_or("Unknown");

    let cpu_processor_count = sys.cpus().len();

    // RAM information
    let total_ram = bytes_to_gb(sys.total_memory());
    let available_ram = bytes_to_gb(sys.available_memory());

    // Operating system information
    let os_name = System::name()
        .unwrap_or_else(|_| "Unknown".to_string());

    let os_version = System::os_version()
        .unwrap_or_else(|_| "Unknown".to_string());

    let hostname = System::host_name().unwrap_or_else(|_| "Unknown".to_string());

    // Display information
    println!("\n========== SYSTEM INFORMATION ==========\n");

    println!("OPERATING SYSTEM");
    println!("OS Name: {}", os_name);
    println!("OS Version: {}", os_version);
    println!("Hostname: {}", hostname);

    println!("\nPROCESSOR");
    println!("CPU Name: {}", cpu_name);
    println!("Logical Processors: {}", cpu_processor_count);

    println!("\nMEMORY");
    println!("Total RAM: {:.2} GB", total_ram);
    println!("Available RAM: {:.2} GB", available_ram);

    println!("\n========================================");
}
