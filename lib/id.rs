use chrono::{offset::TimeZone, Datelike, Utc};
use lazy_static::lazy_static;
use rand::random;
use std::convert::TryInto;

///
/// Get next ID for form
/// The ID formatted as following:
///
/// #ABCD EFGH IJKL
///  | |  |      |- Random
///  | |  |--- -- Counter (BASE32 in "0123456789ACDEFGHJKLMNPQRSTUVWXY")
///  | |- Year (last two digits)
///  |- Message Type ("AP" for application, "CM" for contact message)
pub fn next(prefix: &str) -> String {
    let now = Utc::now();
    let epoch = Utc.ymd(now.year(), 1, 1).and_hms(0, 0, 0);

    let duration = now - epoch;

    format!(
        "#{}{}{}{}",
        prefix,
        now.format("%y"),
        into_counter(duration.num_milliseconds().try_into().unwrap()),
        into_random(random::<u8>())
    )
}

fn into_counter(milliseconds: u64) -> String {
    lazy_static! {
        static ref SYMBOLS: [char; 32] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'C', 'D', 'E', 'F', 'G', 'H',
            'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y',
        ];
        static ref RADIX: [u32; 6] = [33554432, 1048576, 32768, 1024, 32, 1,];
    }

    let interval = into_intervals(milliseconds);

    let mut counter = String::with_capacity(6);

    for &radix in RADIX.iter() {
        counter.push(SYMBOLS[into_counter_index(interval, radix)]);
    }

    counter
}

fn into_counter_index(interval: u32, radix: u32) -> usize {
    (interval / radix % 32).try_into().unwrap()
}

fn into_intervals(milliseconds: u64) -> u32 {
    (milliseconds / 50).try_into().unwrap()
}

fn into_random(number: u8) -> String {
    lazy_static! {
        static ref SYMBOLS: [char; 16] =
            ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',];
    }

    let (ih, il) = into_random_index(number);

    format!("{}{}", SYMBOLS[ih], SYMBOLS[il])
}

fn into_random_index(number: u8) -> (usize, usize) {
    (
        (number / 16).try_into().unwrap(),
        (number % 16).try_into().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_counter() {
        assert_eq!(into_counter(0), "000000");
        assert_eq!(into_counter(50), "000001");
        assert_eq!(into_counter(100), "000002");
        assert_eq!(into_counter(1550), "00000Y");
        assert_eq!(into_counter(1600), "000010");
        assert_eq!(into_counter(31_540_000_000), "KSKGM0");
        assert_eq!(into_counter(31_622_400_000), "KU4S00");
        assert_eq!(into_counter(31_708_800_000), "KVSHH0");
    }
}
