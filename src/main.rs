use crossbeam_channel::bounded;
use crossbeam_utils::thread;
use simdjson::borrowed::Value;
use std::io::BufReader;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum Data<'e> {
    Raw(Value<'e>),
    None,
}

fn main() {
    thread::scope(|t| {
        let (s1, r1) = bounded(5);
        let (s2, r2) = bounded(5);
        t.spawn(|_| {
            for data in r2 {
                println!("{:?}", data);
            }
        });

        let handle = t.spawn(|_| {
            for data in r1 {
                s2.send(data);
            }
        });

        t.spawn(|_| {
            let mut buffer = BufReader::new(io::stdin());

            for line in buffer.lines() {
                unsafe {
                    let mut content = line.unwrap();
                    let value = simdjson::to_borrowed_value(content.as_bytes_mut()).unwrap();

                    s1.send((Data::Raw(value), content));
                }
            }
        });
    });
}