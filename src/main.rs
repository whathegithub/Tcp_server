use std::io::{*};
use std::net::{TcpStream,TcpListener};
use std::str;


/*
Tcp server

*/
fn main() -> std::io::Result<()>{

    //监听对应的ip+端口
    let  listener =  TcpListener::bind("127.0.0.1:80")?;

    
    //   listener.incoming();//1.income有self ,可以直接 . 调用

    for stream in listener.incoming(){ //income返回一个iterator
        match stream {//模式匹配
            Ok(stream) => {
               handClientMsg(stream);
            }
            Err(e) => {
                println!("new client error");
            }
        }
    };

    Ok(())
}

fn handClientMsg(mut stream : TcpStream) {
    let mut buf  = [0;30];//这地方不能用vec?
    //stream.read(&mut buf);//method not found in `std::net::TcpStream`
    stream.read(&mut buf[..]).unwrap();//什么都没改,就不报错了,什么垃圾编译器
    // let msg = String::from_utf8(buf).expect("error");
    // println!("from server msg length : {}", buf.len() );

    //打印客户端输入的内容
    let msg = str::from_utf8_mut(&mut buf).unwrap();
    println!("from server msg : {}", msg );
    
}