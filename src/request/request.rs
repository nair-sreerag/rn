use std::io::{BufRead, BufReader, Read};

use super::Request;

use regex::Regex;

#[derive(Debug)]
pub struct CoreRequest {
    pub headers: Vec<String>,
    pub url: String,
    // pub host: String,
    pub host_base: String,
    pub host_port: u32,

    // TODO: this is dicey;
    // what exactly should be its type?
    // it can be string, json or binary (??) (in case of file uploads)
    pub body: String,
    pub method: String,
}

struct RegexStruct<'a> {
    name: &'a str,
    regex: &'a str,
    keys: Vec<&'a str>,
}

impl CoreRequest {
    pub fn new(mut stream: &std::net::TcpStream) -> Self {
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
            RegexStruct {
                name: "h",
                regex: r"Host:\s+(?<host_base_url>.*):(?<host_port>.*)",
                keys: vec!["host_base_url", "host_port"],
            },
        ];

        let mut collector = Vec::<String>::new();
        let mut line_collector = String::new();
        let mut content_length_bytes: u32 = 0;
        let mut start_collecting_body: bool = false;
        let mut body_byte_counter: u32 = 0;
        let mut body_collector = String::new();
        let mut found_next_line_symbol: bool = false;
        let mut slice_length_diff: usize = 2;

        // THESE ARE VALUES EXTRACTED FROM REGEXPS
        let mut request_method = String::new();
        let mut request_url = String::new();
        let mut destination_base = String::new();
        let mut destination_port: u32 = 0;

        let character_buffer = Self::parse_request_stream_in_chunks(&stream, Some(50));

        println!("ccc {:?}", character_buffer);

        for character in character_buffer.chars() {
            if start_collecting_body {
                body_collector.push(character);

                body_byte_counter += 1;

                if body_byte_counter == content_length_bytes {
                    // println!("body collected {:?} and  {:?}", collector, body_collector);
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

                            collector.push(String::from(
                                &line_collector[0..line_collector.len() - slice_length_diff],
                            ));

                            for r in &all_regexes {
                                let expr = Regex::new(r.regex).unwrap();

                                match r.name {
                                    "cl" => {
                                        for c in &collector {
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
                                            match expr.captures(c) {
                                                Some(some_capture) => {
                                                    request_method = some_capture["http_method"]
                                                        .parse()
                                                        .unwrap();
                                                    request_url =
                                                        some_capture["http_url"].parse().unwrap();
                                                }

                                                // do nothing
                                                None => (),
                                            }
                                        }
                                    }

                                    "h" => {
                                        for c in &collector {
                                            match expr.captures(c) {
                                                Some(some_capture) => {
                                                    destination_base = some_capture
                                                        ["host_base_url"]
                                                        .parse()
                                                        .unwrap();

                                                    destination_port =
                                                        some_capture["host_port"].parse().unwrap();
                                                }

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
                    _ => {
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

        CoreRequest {
            headers: collector,
            body: body_collector,
            method: request_method,
            url: request_url,
            host_base: destination_base,
            host_port: destination_port,
        }
    }

    pub fn parse_request_stream_in_chunks(
        mut stream: &std::net::TcpStream,
        chunk_size: Option<usize>,
    ) -> String {
        let mut parsed_stream: String = String::new();

        loop {
            let mut buffer = vec![
                0;
                match chunk_size {
                    Some(size) => size,
                    _ => 1024,
                }
            ];
            stream.read(&mut buffer).unwrap();

            let x = String::from_utf8_lossy(&buffer);

            let last_char_in_the_stream = x.chars().last().unwrap();

            if last_char_in_the_stream == '\0' {
                let parsed_string: String =
                    x.chars().take_while(|c| *c != '\0').collect::<String>();

                parsed_stream.push_str(&parsed_string);

                println!("breaking");
                break;
            } else {
                let cow_to_string = x.into_owned();

                parsed_stream.push_str(&cow_to_string);
            }
        }

        parsed_stream
    }
}

impl Request for CoreRequest {
    fn add_headers(request_array: Vec<String>, header_string: Vec<String>) -> Vec<String> {
        // TODO: implement this

        Vec::<String>::new()
    }

    // FIXME: DEPR!!
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
