use std::io::{BufRead, BufReader, Read};

use super::Request;

use regex::Regex;

#[derive(Debug)]
pub struct CoreRequest {
    headers: Vec<String>,

    // TODO: this is dicey;
    // what exactly should be its type?
    body: String,
    method: String,
}

struct RegexStruct<'a> {
    name: &'a str,
    regex: &'a str,
    keys: Vec<&'a str>,
}

impl CoreRequest {
    pub fn new(mut stream: &std::net::TcpStream) -> Self {
        let content_length_regex = Regex::new(r"content-length\s*:\s*(?<name>\d{1,})").unwrap();

        let method_extractor_regex = Regex::new(r"(?<http_method>GET|PUT|POST|DELETE|PATCH|HEAD|OPTIONS|TRACE|CONNECT)\s+(?<http_url>.*)\s+HTTP\/(?<http_version>[0-9.]{3,})").unwrap();

        let all_regexes: Vec<RegexStruct> = vec![
            RegexStruct {
                name: "cl",
                regex: r"content-length\s*:\s*(?<name>\d{1,})",
                keys: vec!["name"],
            },
            RegexStruct {
                name: "cm",
                regex: r"(?<http_method>GET|PUT|POST|DELETE|PATCH|HEAD|OPTIONS|TRACE|CONNECT)\s+(?<http_url>.*)\s+HTTP\/(?<http_version>[0-9.]{3,})",
                keys: vec!["http_method", "http_url", "http_version"],
            },
        ];

        let mut continue_loop = true;
        let mut collector = Vec::<String>::new();
        let mut line_collector = String::new();
        let mut content_length_bytes: u32 = 0;
        let mut start_collecting_body: bool = false;
        let mut body_byte_counter: u32 = 0;
        let mut body_collector = String::new();
        let mut found_next_line_symbol: bool = false;
        let mut slice_length_diff: usize = 2;
        let mut request_method = String::new();

        while continue_loop {
            let mut buffer = [0; 1];

            stream.read(&mut buffer);

            let character_buffer = String::from_utf8_lossy(&buffer);

            for character in character_buffer.chars() {
                if start_collecting_body {
                    body_collector.push(character);

                    body_byte_counter += 1;

                    if body_byte_counter == content_length_bytes {
                        println!("body collected {:?} and  {:?}", collector, body_collector);

                        continue_loop = false;
                    }
                } else {
                    match character {
                        '\r' => {
                            line_collector.push(character);

                            if found_next_line_symbol == true {
                                slice_length_diff = 4;
                            }
                        }
                        '\n' => {
                            println!("got \\n");

                            line_collector.push(character);

                            if found_next_line_symbol == true {
                                start_collecting_body = true;
                                println!(
                                    "pushing to global collector and flushing the local collector"
                                );

                                // match content_length_regex.captures(&line_collector.to_lowercase())
                                // {
                                //     // println!("not found yet");
                                //     // return ;
                                //     Some(g) => {
                                //         println!("----------------------- got g {:?}", g);

                                //         content_length_bytes = g["name"].parse().unwrap();
                                //     }
                                //     None => println!("got none in regex"),
                                // };

                                collector.push(String::from(
                                    &line_collector[0..line_collector.len() - slice_length_diff],
                                ));

                                for r in &all_regexes {
                                    println!("getting in the loop for {}", r.regex);
                                    let expr = Regex::new(r.regex).unwrap();

                                    match r.name {
                                        "cl" => {
                                            for c in &collector {
                                                println!(
                                                    "in cl {} and expr {}",
                                                    &c[..].to_lowercase(),
                                                    expr
                                                );
                                                match expr.captures(&c[..].to_lowercase()) {
                                                    Some(some_capture) => {
                                                        content_length_bytes =
                                                            some_capture["name"].parse().unwrap();
                                                    }

                                                    // do nothing
                                                    None => (),
                                                }
                                            }
                                        }

                                        "cm" => {
                                            for c in &collector {
                                                println!("in cm {}", c);

                                                match expr.captures(c) {
                                                    Some(some_capture) => {
                                                        request_method = some_capture
                                                            ["http_method"]
                                                            .parse()
                                                            .unwrap();
                                                    }

                                                    // do nothing
                                                    None => (),
                                                }
                                            }
                                        }

                                        _ => panic!("This call will never occur"),
                                    }
                                }

                                // extract the required fields from here itself

                                line_collector = String::new();
                                slice_length_diff = 2;

                                found_next_line_symbol = false;
                            } else {
                                found_next_line_symbol = true;
                            }
                        }
                        char => {
                            if found_next_line_symbol == true {
                                // match content_length_regex.captures(&line_collector.to_lowercase())
                                // {
                                //     // println!("not found yet");
                                //     // return ;
                                //     Some(g) => {
                                //         println!("----------------------- got g {:?}", g);

                                //         content_length_bytes = g["name"].parse().unwrap();
                                //     }
                                //     None => println!("got none in regex"),
                                // };

                                collector.push(String::from(
                                    &line_collector[0..line_collector.len() - slice_length_diff],
                                ));

                                line_collector = String::new();
                                line_collector.push(character);

                                slice_length_diff = 2;

                                found_next_line_symbol = false;
                            } else {
                                line_collector.push(character);
                            }
                        }
                    }
                }
            }
        }

        println!("here it is");

        CoreRequest {
            headers: collector,
            body: body_collector,

            // TODO: fix this
            method: String::from("GET"),
        }
    }
}

impl Request for CoreRequest {
    fn add_headers(request_array: Vec<String>, header_string: Vec<String>) -> Vec<String> {
        // TODO: implement this

        Vec::<String>::new()
    }

    fn parse_request(mut stream: std::net::TcpStream) -> Vec<String> {
        let buf_reader = BufReader::new(&mut stream);

        // this should handle different use cases
        //  for example when the POST request has a body,
        // u need to parse that separately after the perceived
        // terminator \r\n\r\n of a normal get request.
        //PS.  it marks the end of the headers section of any request

        // use the content length to find out how much the size of data is

        buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect()
    }
}
