
pub struct Converter {}
impl Converter {

    pub fn to_int(string: &String) -> Option<i64> {
        let prefix: String = string.chars()
                            .take(2)
                            .collect();

        let typ = match prefix.as_str() {
            "0b" => 2,
            "0o" => 8,
            "0x" => 16,
            _ => 10
        };

        let valid_chars = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
        let subset = &valid_chars[0..typ];

        let mut chars = string.chars();

        let first_char = chars.next();

        if !(first_char == Some('-')) {
            chars = string.chars();
        }

        if typ != 10 {
            chars.next();
            chars.next();
        }

        for char in chars {
            if !subset.contains(&char) {
                return None;
            }
        }

        Some(Converter::string_to_integer(string, typ as i64))
    }

    fn string_to_integer(str: &String, radix: i64) -> i64 {
        let equivalents = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

        let mut chars = str.chars();
        let mut multiplier: i64 = -1;
        let mut value: i64 = 0;

        if !(chars.next() == Some('-')) {
            multiplier *= -1;
            chars = str.chars();
        }

        let mut d = 1;
        loop {
            match chars.next_back() {
                Some(v) => {
                    match equivalents.binary_search(&v) {
                        Ok(idx) => {
                            value += idx as i64 * d;
                            d *= radix;
                        }
                        Err(_) => break
                    }
                }
                None => break,
            }
        }

        value * multiplier
    }
}