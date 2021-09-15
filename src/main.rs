use std::io;

fn main() {
    eprint!("サーバーのアドレスを入力してください : ");
    let mut server_address = String::new();

    io::stdin().read_line(&mut server_address).ok();

    eprint!("サーバーのサブネットマスクを入力してください : ");
    let mut server_subnet = String::new();

    io::stdin().read_line(&mut server_subnet).ok();

    eprint!("クライアントのアドレスを入力してください : ");
    let mut client_address = String::new();

    io::stdin().read_line(&mut client_address).ok();

    eprint!("クライアントのサブネットマスクを入力してください : ");
    let mut client_subnet = String::new();

    io::stdin().read_line(&mut client_subnet).ok();

    eprint!("\nサーバーアドレス: {}", server_address);
    eprint!("サーバーサブネット: {}", server_subnet);

    eprint!("\nクライアントアドレス: {}", client_address);
    eprint!("クライアントサブネット: {}", client_subnet);

    let server_address: Vec<&str> = server_address.trim().split('.').collect();
    let server_subnet: Vec<&str> = server_subnet.trim().split('.').collect();
    let _client_address: Vec<&str> = client_address.trim().split('.').collect();
    let _client_subnet: Vec<&str> = client_subnet.trim().split('.').collect();

    println!("capacity() = {}", server_address.capacity());

    let result_address = calculate_net(server_address, server_subnet);

    println!("{}", result_address);
    eprint!("{:0>8b}", result_address);
}

fn calculate_net(addr: Vec<&str>, sub: Vec<&str>) -> i64 {
    let mut address = String::new();
    let mut subnet = String::new();
    let result: i64;
    for i in &addr {
        address.push_str(i);
        // println!("{}", type_of(&address));
        println!("{}", address);
    }
    for j in &sub {
        subnet.push_str(j);
    }
    let address = address.trim().parse::<i64>().unwrap();
    let subnet = subnet.trim().parse::<i64>().unwrap();

    result = address & subnet;
    result
}

// fn type_of<T>(_: T) -> String {
//     let a = std::any::type_name::<T>();
//     return a.to_string();
// }
