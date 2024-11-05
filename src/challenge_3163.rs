pub fn run() {
    println!("{}", Solution::compressed_string("abcde".to_string()));
}

struct Solution {}

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut char_iter = word.chars();
        let max_repeat = 9;

        let mut elem = char_iter.next();
        let mut result: Vec<char> = Vec::new();

        while elem.is_some() {
            let c = elem.unwrap();
            let mut count: u8 = 1;

            loop {
                elem = char_iter.next();
                if let Some(k) = elem {
                    if k == c && count < max_repeat {
                        count += 1;
                    } else {
                        result.push((b'0' + count) as char);
                        result.push(c);
                        break;
                    }
                } else {
                    result.push((b'0' + count) as char);
                    result.push(c);
                    break;
                }
            }
        }

        result.iter().collect()
    }
}
