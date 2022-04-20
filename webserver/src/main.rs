mod preforked;

fn main() {
    //while true {
        let mut input = String::new();
        println!("Enter command (write exit to end): ");
        let mut input_bytes = std::io::stdin().read_line(&mut input).unwrap();
        if input.starts_with("prethread-WebServer"){
            // code for prethread
            // preforked-WebServer -n cantidad -w http-root -p port

        } else if input.starts_with("p"){
            let mut input = input.trim(); //removes \n from read_line
            let input_vector: Vec<String> = input.split(' ').map(|s| s.to_string()).collect(); // creates a vector from args
            println!("vector: {:?}", input_vector);
            preforked::init(input_vector);
            // preforked-WebServer -n cantidad -w http-root -p port
        } else if input.starts_with("HTTPclient"){
            // code for HTTP client
        } else if input.starts_with("stress"){
            // code for stress cmd
        } else if input.starts_with("exit"){
            //break;
        } else {
            println!("Invalid command");
        }
    //}
}
