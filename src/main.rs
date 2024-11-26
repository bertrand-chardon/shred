use std::collections::HashMap;

use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    let headers: Vec<_> = match lines.next() {
        Some(result) => match result {
            Ok(header_line) => {
                header_line
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect()
            }
            Err(e) => panic!("{}", e),
        },
        None => panic!("bar"),
    };

    let result = lines.fold(Vec::new(), |mut memo, line| match line {
        Ok(l) => {
            let mut processed_line: HashMap<String, String> = HashMap::new();
            let zipped = headers.iter().zip(l.split_whitespace());
            for (header, value) in zipped {
                processed_line.insert(header.to_string(), value.to_string());
            }
            memo.push(processed_line);
            return memo;
        }
        Err(_) => {
            panic!("boo")
        }
    });

    println!("{:?}", result);
}
