use std::{
    fs::{self, OpenOptions},
    thread,
};

use anyhow::{anyhow, Result};
use number_prefix::NumberPrefix;
use rand::{thread_rng, Rng};
use std::io::Write;
use tui::style::Color;

pub mod sharable_state;

pub fn generate_random_color() -> Color {
    let mut rng = thread_rng();
    Color::Rgb(
        rng.gen_range(128..=255),
        rng.gen_range(128..=255),
        rng.gen_range(128..=255),
    )
}

/// e.g:
/// ```
/// assert_eq!(bytes_len_to_string_prefix(1_073_741_824), "1 GiB")
/// ```
pub fn bytes_len_to_string_prefix(bin_size: u64) -> String {
    match NumberPrefix::binary(bin_size as f64) {
        NumberPrefix::Standalone(bytes) => format!("{bytes}B"),
        NumberPrefix::Prefixed(prefix, n) => format!("{:.1} {}B", n, prefix),
    }
}

pub fn log_print(log: String) {
    thread::spawn(move || {
        dotenv::dotenv().unwrap();
        let path_str = format!("/home/{}/.cache/rtkill", dotenv::var("USER").unwrap());
        if fs::create_dir_all(path_str.as_str()).is_err() {
            return;
        }

        let log_file_path = format!("{path_str}/dev_logs.log");
        let mut file = match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_file_path)
        {
            Ok(f) => f,
            Err(_) => return,
        };
        let _ = writeln!(file, "{log}");
    });
}

pub trait FromHex {
    fn from_hex(hex: &str) -> Result<Color>;
}

impl FromHex for Color {
    fn from_hex(hex: &str) -> Result<Color> {
        let safe_hex = hex.trim().trim_start_matches('#').trim_start_matches("0x");

        let hex_val = safe_hex.chars().collect::<Vec<_>>();
        if hex_val.len() != 6 {
            return Err(anyhow!("wrong length, expected 3 crumb"));
        }

        let mut hex_crumb = vec![]; // fun fact: a crumb is a group of 2 bits: https://en.wikipedia.org/wiki/Units_of_information

        let mut i = 0;
        while i < hex_val.len() - 1 {
            let str = &hex_val[i..=i + 1]
                .iter()
                .map(|c| c.to_string())
                .collect::<String>();
            hex_crumb.push(u8::from_str_radix(str, 16)?);
            i += 2;
        }
        if hex_crumb.len() != 3 {
            return Err(anyhow!("wrong length, expected 3 crumb"));
        }

        Ok(Color::Rgb(hex_crumb[0], hex_crumb[1], hex_crumb[2]))
    }
}
