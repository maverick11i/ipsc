use std::io;

fn main() {
    println!("サーバーのアドレスを入力してください");
    let mut server_address = String::new();

    io::stdin()
        .read_line(&mut server_address)
        .expect("Failed to read line");

    println!("サーバーのサブネットマスクを入力してください");
    let mut server_subnet = String::new();

    io::stdin()
        .read_line(&mut server_subnet)
        .expect("サブネットが上手く入力されていません");

    println!("クライアントのアドレスを入力してください");
    let mut client_address = String::new();

    io::stdin()
        .read_line(&mut client_address)
        .expect("アドレスが上手く入力されていません");

    println!("クライアントのサブネットマスクを入力してください");
    let mut client_subnet = String::new();

    io::stdin()
        .read_line(&mut client_subnet)
        .expect("サブネットが上手く入力されていません");

    let server_address: Vec<&str> = server_address.split('.').collect();
    let server_subnet: Vec<&str> = server_subnet.split('.').collect();
    let client_address: Vec<&str> = client_address.split('.').collect();
    let client_subnet: Vec<&str> = client_subnet.split('.').collect();

    let cnt = 0;

    for i in server_address.iter() {
        i.parse::<i32>().unwrap();
    }
    for j in server_subnet.iter() {
        j.parse::<i32>().unwrap();
    }
    for x in client_address.iter() {
        x.parse::<i32>().unwrap();
    }
    for y in client_subnet.iter() {
        y.parse::<i32>().unwrap();
    }
}
