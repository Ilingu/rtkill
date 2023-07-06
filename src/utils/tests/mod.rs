#[cfg(test)]
mod utils_tests {
    use std::{mem::ManuallyDrop, sync::Arc, thread};

    use rand::{thread_rng, Rng};
    use tui::style::Color;

    use crate::utils::{bytes_len_to_string_prefix, sharable_state::SharableState, FromHex};

    #[test]
    fn test_format_size() {
        assert_eq!(bytes_len_to_string_prefix(2_u64.pow(0)), "1B");
        assert_eq!(bytes_len_to_string_prefix(2_u64.pow(10)), "1.0 KiB");
        assert_eq!(bytes_len_to_string_prefix(2_u64.pow(20)), "1.0 MiB");
        assert_eq!(bytes_len_to_string_prefix(2_u64.pow(30)), "1.0 GiB");
    }

    #[test]
    fn test_from_hex() {
        // tests colors in app
        assert!(Color::from_hex("#2ecc71").is_ok());
        assert!(Color::from_hex("#3498db").is_ok());
        assert!(Color::from_hex("#f1c90f").is_ok());
        assert!(Color::from_hex("#e74c3c").is_ok());
        assert!(Color::from_hex("e74c3c").is_ok());
        assert!(Color::from_hex(" #e74c3c ").is_ok());

        // tests fake colors
        assert!(Color::from_hex("#e74cc").is_err());
        assert!(Color::from_hex("e74ccx9").is_err());
        assert!(Color::from_hex("$e74c3c").is_err());
        assert!(Color::from_hex("#w74c3c").is_err());

        // run 1000 random tests
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let hex = format!(
                "{:02x}{:02x}{:02x}",
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255)
            );
            assert!(Color::from_hex(&hex).is_ok(), "{hex}");
        }
    }

    #[test]
    fn test_sharable_state() {
        let basic_state = Arc::new(SharableState::new(vec![]));

        let threaded_state = Arc::clone(&basic_state);
        thread::spawn(move || {
            for i in 0..1000 {
                threaded_state.mutate(|counter| counter.push(i))
            }
        });

        loop {
            let data = ManuallyDrop::<Vec<usize>>::into_inner(basic_state.read().clone());
            if data.len() == 1000 {
                assert_eq!(data, (0..1000).collect::<Vec<_>>());
                break;
            }
        }
    }
}
