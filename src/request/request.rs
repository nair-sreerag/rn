use std::{
    io::Error,
    io::{BufRead, BufReader, Read},
};

use super::{Header, Request};

use regex::Regex;

#[derive(Debug)]
pub struct CoreRequestParser {
    pub headers: Vec<String>,
    pub host_path: String,
    // pub host: String,
    pub host_base: String,
    pub host_port: u32,

    // TODO: this is dicey;
    // what exactly should be its type?
    // it can be string,
    // json or binary (??) (in case of file uploads)
    // Me thinks it should be Vec<bytes> or string
    pub body: String,
    pub method: String,

    pub content_type: String,

    pub auth_type: String,
    pub cookie: String,
    pub auth_token: String,
    pub encoding: String,

    pub source_address: String,
}

struct RegexStruct<'a> {
    name: RegexExtractors,
    regex: &'a str,
    keys: Vec<&'a str>,
}

enum RegexExtractors {
    ContentLength,
    MethodAndUrl,
    HostData,
    ContentType,
    Authorization,
    Cookie,
    AcceptEncoding,
}

impl CoreRequestParser {
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

            let buffer_to_cow = String::from_utf8_lossy(&buffer);

            let last_char_in_the_stream = buffer_to_cow.chars().last().unwrap();

            if last_char_in_the_stream == '\0' {
                // '\0' marks the end of the request as a whole
                let parsed_string: String = buffer_to_cow
                    .chars()
                    .take_while(|c| *c != '\0')
                    .collect::<String>();

                parsed_stream.push_str(&parsed_string);

                // println!("breaking");
                break;
            } else {
                let cow_to_string = buffer_to_cow.into_owned();

                parsed_stream.push_str(&cow_to_string);
            }
        }

        parsed_stream
    }

    pub fn new(mut stream: &std::net::TcpStream) -> Self {
        let source_address = stream.peer_addr().unwrap();

        println!("ssss ->>> {:?}", source_address.to_string());

        let all_regexes: Vec<RegexStruct> = vec![
            RegexStruct {
                name: RegexExtractors::ContentLength,
                regex: r"content-length\s*:\s*(?<name>\d{1,})",
                keys: vec!["name"],
            },
            RegexStruct {
                name: RegexExtractors::MethodAndUrl,
                regex: r"(?<http_method>GET|PUT|POST|DELETE|PATCH|HEAD|OPTIONS|TRACE|CONNECT)\s+(?<http_url>.*)\s+HTTP\/(?<http_version>[0-9.]{3,})",
                keys: vec!["http_method", "http_url", "http_version"],
            },
            RegexStruct {
                name: RegexExtractors::HostData,
                regex: r"Host:\s+(?<host_base_url>.*):(?<host_port>.*)",
                keys: vec!["host_base_url", "host_port"],
            },
            RegexStruct {
                name: RegexExtractors::ContentType,
                regex: r"Content-Type:\s*(?<content_type>.*)\s*;*",
                keys: vec!["content_type"],
            },
            RegexStruct {
                name: RegexExtractors::Authorization,
                regex: r"Authorization:\s*(?<auth_type>.*)\s+(?<auth_token>.*)", // COMEBACK TO THIS FOR DIFF AUTH TYPES
                keys: vec!["auth_type", "auth_token"],
            },
            RegexStruct {
                name: RegexExtractors::Cookie,
                regex: r"Cookie:\s*(?<cookie>.*)",
                keys: vec!["cookie"],
            },
            RegexStruct {
                name: RegexExtractors::AcceptEncoding,
                regex: r"Accept-Encoding:\s*(?<encoding>.*)", //Accept-Encoding: gzip, deflate
                keys: vec!["encoding"],
            },
        ];

        let mut collector = Vec::<String>::new();
        let mut single_line_collector = String::new();
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
        let mut content_type = String::new();
        let mut auth_type = String::new();
        let mut auth_token = String::new();
        let mut cookie = String::new();
        let mut encoding = String::new();

        let character_buffer = Self::parse_request_stream_in_chunks(&stream, Some(500));

        // println!("ccc {:?}", character_buffer);

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
                        single_line_collector.push(character);

                        if found_next_line_symbol == true {
                            slice_length_diff = 4;
                        }
                    }
                    '\n' => {
                        // println!("got \\n");

                        single_line_collector.push(character);

                        // again, this will be true if \n is encountered
                        // the second time. it also means the next bytes will be the
                        // content (body) that is being passed in the request

                        if found_next_line_symbol == true {
                            start_collecting_body = true;
                            // println!(
                            //     "pushing to global collector and flushing the local collector"
                            // );

                            collector.push(String::from(
                                &single_line_collector
                                    [0..single_line_collector.len() - slice_length_diff],
                            ));

                            for r in &all_regexes {
                                let expr = Regex::new(r.regex).unwrap();

                                match r.name {
                                    RegexExtractors::ContentLength => {
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

                                    RegexExtractors::MethodAndUrl => {
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

                                    RegexExtractors::HostData => {
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

                                    RegexExtractors::ContentType => {
                                        for c in &collector {
                                            match expr.captures(c) {
                                                Some(some_capture) => {
                                                    content_type = some_capture["content_type"]
                                                        .parse()
                                                        .unwrap()
                                                }
                                                None => (),
                                            }
                                        }
                                    }

                                    RegexExtractors::Authorization => {
                                        for c in &collector {
                                            match expr.captures(c) {
                                                Some(some_capture) => {
                                                    auth_type =
                                                        some_capture["auth_type"].parse().unwrap();

                                                    auth_token =
                                                        some_capture["auth_token"].parse().unwrap();

                                                    println!(
                                                        "zzz ->>> {} and {}",
                                                        auth_type, auth_token
                                                    );
                                                }
                                                None => (),
                                            }
                                        }
                                    }

                                    RegexExtractors::Cookie => {
                                        for c in &collector {
                                            match expr.captures(c) {
                                                Some(some_capture) => {
                                                    cookie =
                                                        some_capture["cookie"].parse().unwrap();
                                                }
                                                None => (),
                                            }
                                        }
                                    }

                                    RegexExtractors::AcceptEncoding => {
                                        for c in &collector {
                                            match expr.captures(c) {
                                                Some(some_capture) => {
                                                    encoding =
                                                        some_capture["encoding"].parse().unwrap();
                                                }
                                                None => (),
                                            }
                                        }
                                    }

                                    _ => panic!("This call will never occur"),
                                }
                            }

                            single_line_collector = String::new();
                            slice_length_diff = 2;

                            found_next_line_symbol = false;
                        } else {
                            found_next_line_symbol = true;
                        }
                    }
                    _ => {
                        if found_next_line_symbol == true {
                            collector.push(String::from(
                                &single_line_collector
                                    [0..single_line_collector.len() - slice_length_diff],
                            ));

                            single_line_collector = String::new();
                            single_line_collector.push(character);

                            slice_length_diff = 2;

                            found_next_line_symbol = false;
                        } else {
                            single_line_collector.push(character);
                        }
                    }
                }
            }
        }

        CoreRequestParser {
            headers: collector,
            body: body_collector,
            method: request_method,
            host_path: request_url,
            host_base: destination_base,
            host_port: destination_port,
            content_type,
            auth_type,
            cookie,
            auth_token,
            encoding,
            source_address: source_address.to_string(),
        }
    }
}

impl Request for CoreRequestParser {
    fn add_header(&mut self, header_to_push: String) -> Result<bool, Error> {
        self.headers.push(header_to_push);

        Ok(true) // should always be the case
    }

    // fn delete_header(&mut self) -> Result<bool, Error> {}

    fn replace_destination_uri(&mut self, new_uri: String) -> Result<bool, Error> {
        self.headers.pop().unwrap();

        Ok(true)
    }
}

// impl Request for CoreRequestParser {
// fn add_headers(&mut self, headers_to_add: Vec<Header>) {
//     // TODO: implement this

//     for header in headers_to_add {
//         self.headers
//             .push(format!("{}: {}", header.key, header.value));
//     }

//     // self
// }

// fn replace_headers(
//     mut header: &str,
//     expression: &str,
//     replacement_values: Vec<super::ReplacementStruct>,
// ) -> String {
//     let regexp = Regex::new(expression).unwrap();

//     header = regexp.replace_all(&header, |captures| {
//         for r in replacement_values {

//             if let Some(word) = captures[&r.key[..]] {

//             }

//         }
//     });

//     String::from(header)
// }

// // FIXME: DEPR!!
// fn parse_request(mut stream: std::net::TcpStream) -> Vec<String> {
//     let buf_reader = BufReader::new(&mut stream);

//     // this should handle different use cases
//     //  for example when the POST request has a body,
//     // u need to parse that separately after the perceived
//     // terminator \r\n\r\n of a normal get request.
//     //PS.  it marks the end of the headers section of any request

//     // use the content length to find out how much the size of data is

//     buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect()
// }
// }

#[cfg(test)]
mod tests {

    use super::*;

    // WILL NEED TO SIMULATE AN HTTP REQUEST

    // check if basic parsing works or not
    // check if add_headers() works or not
}
