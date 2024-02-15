use std::{
    any,
    fs::read_to_string,
    io::{BufRead, BufReader, Error, Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
};

use regex::Regex;
// Content-Length: 101

use crate::CoreThreadPool;

use super::{Job, Server};

pub struct CoreServer {
    host: String,
    port: u32,
    s_port: u32,
    thread_count: usize,
    // pool: Option<Vec<Wor>>,
    server_handle: Option<()>,
    sender: mpsc::Sender<Job>,
    // receiver: Arc<Mutex<Receiver<()>>>,
    thread_pool: CoreThreadPool,
}

impl CoreServer {
    pub fn new(host: String, port: u32, s_port: u32, thread_count: usize) -> Self {
        println!("Intializing the server.");

        println!("Thread count is {}", thread_count);

        let (sender, receiver) = mpsc::channel::<Job>();

        let rx = Arc::new(Mutex::new(receiver));

        let thread_pool = CoreThreadPool::new(thread_count, rx);

        Self {
            host,
            port,
            s_port,
            thread_count,
            server_handle: Some(()),
            sender,
            // receiver: rx,
            thread_pool,
        }
    }

    pub fn handle_incoming_request(mut stream: std::net::TcpStream) {
        
        let mut continue_loop = true;

        let mut collector = Vec::<String>::new();

        let mut line_collector = String::new();

        let mut content_length_bytes: u32 = 0;

        let mut start_collecting_body :bool= false;
        let mut body_byte_counter: u32 = 0;

        let mut body_collector = String::new();

        let mut found_next_line_symbol: bool = false;

        let mut slice_length_diff: usize = 2;


        while continue_loop {

            let mut buffer = [0; 1];

            stream.read(&mut buffer);
            
            let z = String::from_utf8_lossy(&buffer);

            
            for character in z.chars() {

                        if start_collecting_body  {


                            body_collector.push(character);

                            body_byte_counter += 1;

                            if body_byte_counter == content_length_bytes {
                                
                                println!("body collected  {:?}", body_collector);


                                continue_loop = false;
                            }

                           
                        } else {

                            if character == '\r' {
                                line_collector.push(character);
    
    
                                if found_next_line_symbol == true {
                                    slice_length_diff = 4;
                                    
                                    } 
                                }
                                else if character == '\n'{
                                    println!("got \\n");
    
                                    line_collector.push(character);

    
                                    if found_next_line_symbol == true  {
    
                                        start_collecting_body = true;
                                        println!("pushing to global collector and flushing the local collector");
                                        

                                        let regular = Regex::new(r"content-length\s*:\s*(?<name>\d{1,})").unwrap();

                                        match regular.captures(&line_collector.to_lowercase())   {
                                            // println!("not found yet");
                                            // return ;
                    
                                            Some(g) => { println!("----------------------- got g {:?}", g);
                                        
                                            content_length_bytes = g["name"].parse().unwrap();
                    
                    
                                         },
                                            None => println!("got none in regex")
                                        };



                                        collector.push(
                                            String::from(
                                                &line_collector[0.. line_collector.len() - slice_length_diff]
                                            )
                                        );
    
    
            
                                        line_collector = String::new();
                                        slice_length_diff = 2;
    
    
                                        found_next_line_symbol = false;
    
                                    } else {
                                        found_next_line_symbol = true;
                                    }
    
                                }
                                else {
    
                                    if found_next_line_symbol == true  {

                                        let regular = Regex::new(r"content-length\s*:\s*(?<name>\d{1,})").unwrap();

                                        match regular.captures(&line_collector.to_lowercase())   {
                                            // println!("not found yet");
                                            // return ;
                    
                                            Some(g) => { println!("----------------------- got g {:?}", g);
                                        
                                            content_length_bytes = g["name"].parse().unwrap();
                    
                    
                                         },
                                            None => println!("got none in regex")
                                        };


                                        collector.push(
                                            String::from(
                                                &line_collector[0.. line_collector.len() - slice_length_diff]
                                            )
                                        );

                                        line_collector = String::new();
                                        line_collector.push(character);

                                        slice_length_diff = 2;
    
    
                                        found_next_line_symbol = false;
                                        

                                    } else {
                                        line_collector.push(character);
                                    }
    
                                }
    
    
                            // println!("slice_length_diff {} and {:?}",slice_length_diff, line_collector);
                        }
            }

           

        }


    //     let post_body_start = res.find("\r\n\r\n").unwrap_or(0) + 4;
    // let post_body = &res[post_body_start..];

    //     println!(" ->>>> {:?}", post_body);


        // let http_request: Vec<_> = buf_reader
        //     .lines()
        //     .map(|result| {
        //         let u  =result.unwrap();
        //         println!("unwrapped result ->> {} ",u );
        //         u
        //     })
        //     .take_while(|line| {

        //         !line.is_empty()
        //     })
        //     .collect();

        // // create a stream to the final destination - fd
        // // forward the entire request to fd and wait for its response
        // // when u get the response, return it back to the calling server

        // // returning the response


        // println!("req uere {:?}", http_request);

            println!("content_length_bytescontent_length_bytescontent_length_bytescontent_length_bytes {}", content_length_bytes);


        let mut local_server_connection = TcpStream::connect("127.0.0.1:35577").unwrap();

        let get_request = vec![
            
    "GET /hello-world HTTP/1.1",
    "Host: 127.0.0.1:35577",
    "User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:99.0) Gecko/20100101 Firefox/99.0",
    "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
    "Accept-Language: en-US,en;q=0.5",
    "Accept-Encoding: gzip, deflate, br",
    "DNT: 1",
    "Connection: keep-alive",
    "Upgrade-Insecure-Requests: 1",
    "Sec-Fetch-Dest: document",
    "Sec-Fetch-Mode: navigate",
    "Sec-Fetch-Site: none",
    "Sec-Fetch-User: ?1",
    "Cache-Control: max-age=0",
]
        ;

        // println!("tolo this ->>> {:?}", http_request);

        // u need to basically
        // modify the above buf_reader and

        let mut response1: Vec<u8> = Vec::<u8>::new();
        local_server_connection
            .write(format!("{}\r\n\r\n",get_request.join("\r\n")).as_bytes())
            .unwrap();

        local_server_connection.read_to_end(&mut response1).unwrap();

        println!("ressss - >>>>> {:?}", std::str::from_utf8(&response1));

        println!("{}", get_request.join("\r\n"));

        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = String::from("zz");
        let length = contents.len();

        stream
            .write_all(
                // format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes(),
            
                &response1
            )
            .unwrap()
    }
}

impl Server for CoreServer {
    fn get_server_handle(&self) -> Result<std::net::TcpListener, std::io::Error> {
        let complete_address: String = format!("{}:{}", self.host, self.port);

        let server_binding = TcpListener::bind(complete_address);

        let server_binding_status = match server_binding {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        };

        server_binding_status
    }

    fn start<F>(&self, server_handle: std::net::TcpListener, executor_function: F)
    where
        F: FnOnce(std::net::TcpStream) + Send + Copy + 'static,
    {
        println!(
            "Server is now listening on http://{}:{}",
            self.host, self.port
        );

        for l in server_handle.incoming() {
            println!("got an incoming",);
            let stream = l.unwrap();

            // self.handle_incoming_request(stream);

            let job = Box::new(move || executor_function(stream));

            self.sender.send(job).unwrap();
        }
    }

    fn get_permissible_limit() -> u32 {
        // TODO: change this

        20
    }

    // fn handle_incoming_request_server(&self, mut stream: std::net::TcpStream) {
    //     let buf_reader = BufReader::new(&mut stream);
    //     let http_request: Vec<_> = buf_reader
    //         .lines()
    //         .map(|result| result.unwrap())
    //         .take_while(|line| !line.is_empty())
    //         .collect();

    //     println!("Request: {:#?}", http_request);
    // }
}
