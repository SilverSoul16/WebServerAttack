extern crate ftp;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::time::Duration;
use web_server::ThreadPool;
use std::thread;
use ftp::FtpStream;

/*
struct Database {
    data: Vec<json::JsonValue>
}

impl Database{
    fn init() -> Database {
        let data = Vec::new();
        let info = json::object!{
            title: "test.txt",
            contents: fs::read_to_string("test.txt").unwrap()
        };
        data.push(info)
        
        Database { data }
    }

    fn push(value: json:JsonValue) {
        
    }
}*/
static mut database: Vec<serde_json::Value> = Vec::new();
//static mut database: Vec<json::JsonValue> = Vec::new();

pub fn init(args: Vec<String>) {
    let mut max = args.len() - 1;
    let mut counter = 1;
    let mut root: &str = "127.0.0.1";
    let mut port: &str = "7878";
    let mut process_n: usize = 1;

    while max > 0 {
        let flag = &args[counter];
        let flag_value = &args[counter + 1];
        
        if flag == "-n" {
            process_n = flag_value.parse().unwrap();
        } else if flag == "-w" {
            root = flag_value;
        } else if flag == "-p"{
            port = flag_value;
        } else {
            println!("ERROR: invalid flag. Valid options: -n -w -p");
            break;
        }
        max -= 2;
        counter += 2;
    }

    // initializes the server
    server_init(root, port, process_n);
}

fn server_init(root: &str, port: &str, n: usize) {
    let http_url = format!("{}:{}", root, port);
    println!("{}", http_url);
    //let mut ftp_stream = FtpStream::connect(http_url).unwrap_or_else(|err|
    //    panic!("{}", err)
    //);
    let listener = TcpListener::bind(http_url).unwrap();        // returns a new tcp listener, returns Result<T, E>

    let pool = ThreadPool::new(n);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //sends each request to a different thread
        pool.execute(|| {
            handle_requests(stream);
        });
    }
}

fn handle_requests(mut stream: TcpStream){
    let mut buffer = [0; 1024];         // holds the data as its read on, 1024 bytes
    stream.read(&mut buffer).unwrap();

    //let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let buffer_vector: Vec<String> = std::str::from_utf8(&buffer).unwrap().split(' ').map(|s| s.to_string()).collect(); // creates a vector from args

    if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        get("HTTP/1.1 200 OK", "hi.html", stream);
    } else if buffer.starts_with(b"GET") || buffer.starts_with(b"POST") {
        get("HTTP/1.1 200 OK", "hi.html", stream);
    } else if buffer.starts_with(b"PUT") {
        put("HTTP/1.1 200 OK", buffer_vector[1].trim_start_matches("/"), stream); // /bin/zip ERR stream did not contain valid UTF-8
    } else if buffer.starts_with(b"DELETE") {
        put("HTTP/1.1 200 OK", "test.txt", stream);
    } else if buffer.starts_with(b"HEAD") {
        put("HTTP/1.1 200 OK", "hi.html", stream);
    } else {
        println!("ERROR: Invalid command.");
    }
}


fn get(status_line: &str, filename: &str, mut stream: TcpStream){
    unsafe {
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{:#?}",
            status_line,
            database.len(),
            database
        );

        match stream.write(response.as_bytes()) {
            Ok(_) => println!("Response: {}", response),
            Err(e) => println!("ERROR: {}", e),
        }
        stream.flush().unwrap();
    }
}

fn put(status_line: &str, filename: &str, mut stream: TcpStream) {
    let data = serde_json::json!({
        "title": filename,
        "contents": fs::read_to_string(filename).unwrap()
    });

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        serde_json::to_string(&data).unwrap().len(),
        serde_json::to_string_pretty(&data).unwrap()
    );

    unsafe {
        database.push(data);
    }

    match stream.write(response.as_bytes()) {
        Ok(_) => println!("Response: {}", response),
        Err(e) => println!("ERROR: {}", e),
    }
    stream.flush().unwrap();
}