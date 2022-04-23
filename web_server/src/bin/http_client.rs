
pub fn init(args: Vec<String>) {
    let mut max = args.len() - 1;
    let mut counter = 1;
    let mut host_to_connect: &str = "localhost:7878/";

    while max > 0 {
        let flag = &args[counter];
        let flag_value = &args[counter + 1];
        
        if flag == "-h" {
            host_to_connect = flag_value;
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