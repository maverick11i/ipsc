use std::io;

fn main() {
    println!("サーバーのアドレスを入力してください");
    let mut server_address = String::new();

    io::stdin()
        .read_line(&mut server_address)
        .expect("アドレスが上手く入力されていません");

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

    let mut vec1: Vec<&str> = server_address.split('.').collect();
    let mut vec2: Vec<&str> = server_subnet.split('.').collect();
    let mut vec3: Vec<&str> = client_address.split('.').collect();
    let mut vec4: Vec<&str> = client_subnet.split('.').collect();

    let num: Vec<i32> = vec1.parse().unwrap();

    println!("{}", vec1.len());

    // for i in &mut vec1 {
    //     println!("{}", i);
    // }
    // for j in &mut vec2 {
    //     println!("{}", j);
    // }
    // for x in &mut vec3 {
    //     println!("{}", x);
    // }
    // for y in &mut vec4 {
    //     println!("{}", y);
    // }
}
