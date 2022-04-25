
pub fn init(args: Vec<String>) {
    let mut max = args.len() - 1;
    let mut counter = 1;
    let mut host_to_connect: &str = "localhost:7878/";
    //let mut stream = TcpStream::connect(host_to_connect).expect("Could not connect to server."); //supposely communicates with the tcp stream

    while max > 0 {
        let flag = &args[counter];
        let flag_value = &args[counter + 1];
        
        if flag == "-h" {
            host_to_connect = flag_value;
            //stream = TcpStream::connect(host_to_connect).expect("Could not connect to server.");
            // se hace un stream.write para escribir al tcp stream.
        } else if flag.starts_with("["){
            let comm_list = &args[counter..];
            validate_commands(comm_list.to_vec());
            break;
        } else {
            println!("ERROR: invalid flag. Valid options: -h");
            break;
        }
        max -= 2;
        counter += 2;
    }

}

fn validate_commands(commands: Vec<String>) {
    //HTTPclient -h localhost:7878/ [GET data="title:hi"]

    let mut max = commands.len();
    let mut counter = 0;
    let mut host_to_connect: &str = "localhost:7878/";

    while max > 0 {
        let method = &commands[counter].trim_start_matches("[");
        let data = &commands[counter + 1];
        
        /* standard GET
        curl url
        or
        curl url -X GET

        to download file:
        curl localhost:7878/file.txt

        to post it
        curl -F 'data=@path' uploadURL  //no funciona?
        curl localhost:7878/ -H "Content-Type: document" --data-binary "@/bin/zip"
        home/gio/Documents/WebServerAttack/web_server/test.txt
        */
        
        if *method == "POST" {
            println!("POST METHOD: {}", method);
            println!("POST DATA: {}", data);
        } else if *method == "GET" {
            println!("GET METHOD: {}", method);
            println!("GET DATA: {}", data);
        } else if *method == "HEAD" {
            println!("HEAD METHOD: {}", method);
            println!("HEAD DATA: {}", data);
        } else if *method == "PUT" {
            println!("PUT METHOD: {}", method);
            println!("PUT DATA: {}", data);
        } else if *method == "DELETE" {
            println!("DELETE METHOD: {}", method);
            println!("DELETE DATA: {}", data);
        } else {
            println!("ERROR: Invalid HTTP method.");
        }

        max -= 2;
        counter += 2;
    }

}