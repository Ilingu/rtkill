use anyhow::{anyhow, Result};
use rand::{thread_rng, Rng};
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
