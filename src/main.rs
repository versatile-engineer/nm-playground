use nmrs::{models::Network, NetworkManager};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> nmrs::Result<()> {
    // Initialize NetworkManager
    let nm = NetworkManager::new().await?;

    println!("WiFi Network Scanner");
    println!("===================\n");

    // Scan loop
    loop {
        // Clear screen (Unix/Linux)
        print!("\x1B[2J\x1B[1;1H");

        // Get networks
        let mut networks = nm.list_networks().await?;

        // Sort by signal strength (strongest first)
        networks.sort_by(|a, b| b.strength.unwrap_or(0).cmp(&a.strength.unwrap_or(0)));

        // Display header
        println!("WiFi Network Scanner - {} networks found\n", networks.len());
        println!(
            "{:<30} {:>10} {:>8} {:<20}",
            "SSID", "Signal", "Band", "Security"
        );
        println!("{}", "-".repeat(70));

        // Display each network
        for net in networks {
            print_network(&net);
        }

        println!("\n{}", "-".repeat(70));
        println!("Press Ctrl+C to exit");

        // Wait before next scan
        sleep(Duration::from_secs(5)).await;
    }
}

fn print_network(net: &Network) {
    let signal = net.strength.unwrap_or(0);
    let signal_bar = signal_strength_bar(signal);

    let band = match net.frequency {
        Some(freq) if freq > 5000 => "5GHz",
        Some(_) => "2.4GHz",
        None => "Unknown",
    };

    // let security = match &net.security {
    //     nmrs::WifiSecurity::Open => "Open",
    //     nmrs::WifiSecurity::WpaPsk { .. } => "WPA-PSK",
    //     nmrs::WifiSecurity::WpaEap { .. } => "WPA-EAP",
    // };

    println!(
        "{:<30} {:>3}% {} {:>8}",
        truncate_ssid(&net.ssid, 30),
        signal,
        signal_bar,
        band,
    );
}

fn signal_strength_bar(strength: u8) -> String {
    let bars = match strength {
        80..=100 => "▂▄▆█",
        60..=79 => "▂▄▆▁",
        40..=59 => "▂▄▁▁",
        20..=39 => "▂▁▁▁",
        _ => "▁▁▁▁",
    };

    let color = match strength {
        70..=100 => "\x1b[32m", // Green
        40..=69 => "\x1b[33m",  // Yellow
        _ => "\x1b[31m",        // Red
    };

    format!("{}{}\x1b[0m", color, bars)
}

fn truncate_ssid(ssid: &str, max_len: usize) -> String {
    if ssid.len() <= max_len {
        ssid.to_string()
    } else {
        format!("{}...", &ssid[..max_len - 3])
    }
}
