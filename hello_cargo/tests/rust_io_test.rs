// https://mp.weixin.qq.com/s/mK8JNySdVwoyjuZ5fSAyOA 深入解析Rust异步网络利器MIO：从入门到实战，一文搞懂高性能IO
// MIO是Rust语言的低级别非阻塞I/O库，它为不同操作系统的异步I/O操作提供了统一的抽象层。在Linux上它使用epoll，在macOS和BSD上使用kqueue，在Windows上使用IOCP。
#[cfg(test)]
mod tests {

    use mio::net::TcpListener;
    use mio::*;
    use std::io::{self, Read, Write};
    const SERVER: Token = Token(0);
    fn main() -> io::Result<()> {
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(1024);
        let addr = "127.0.0.1:8000".parse().unwrap();
        let mut server = TcpListener::bind(addr)?;
        poll.registry()
            .register(&mut server, SERVER, Interest::READABLE)?;
        let mut connections = Vec::new();
        loop {
            poll.poll(&mut events, None)?;
            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let (mut connection, _) = server.accept()?;
                        let token = Token(connections.len() + 1);
                        poll.registry()
                            .register(&mut connection, token, Interest::READABLE)?;
                        connections.push(connection);
                    }
                    token => {
                        let mut connection = &mut connections[token.0 - 1];
                        let mut buffer = [0; 1024];
                        match connection.read(&mut buffer) {
                            Ok(n) if n > 0 => {
                                connection.write_all(&buffer[..n])?;
                            }
                            _ => { // 连接关闭或错误处理
                            }
                        }
                    }
                }
            }
        }
    }
}
