use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_incoming(stream: TcpStream) {
    let mut stream = BufReader::new(stream); // 创建可变缓存，存储网络流
    let mut buf = String::from("Replay: "); // 创建字符可变变量
    while match stream.read_line(&mut buf) {
        // 循环读取网络流，并将网络流缓存到字段缓冲区
        Ok(size) => {
            //模式匹配搭并将接收到的字符回写到网络流里
            stream.get_ref().write(buf.as_bytes()).unwrap();
            println!("size: {} echo: {:?}", size, buf); //打印收到的字符
            true
        }
        Err(e) => {
            //模式匹配到错误并打印错误详情
            println!("An error occurred, terminating connection with {}", e);
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap(); //在本地网络环路的3000端口开启监听
    println!("Server listening on port 3000");
    for stream in listener.incoming() {
        //循环从网络套接字上获取入站迭代器的返回值
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_incoming(stream)); // 多进程方式处理网络套接字
            }
            Err(e) => {
                println!("Error: {}", e);
                // stream.shutdown(Shutdown::Both).unwrap();
            }
        }
    }
    drop(listener); //移除监听器
}
