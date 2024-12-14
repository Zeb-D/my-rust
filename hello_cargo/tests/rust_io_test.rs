// https://mp.weixin.qq.com/s/mK8JNySdVwoyjuZ5fSAyOA 深入解析Rust异步网络利器MIO：从入门到实战，一文搞懂高性能IO
// MIO是Rust语言的低级别非阻塞I/O库，它为不同操作系统的异步I/O操作提供了统一的抽象层。在Linux上它使用epoll，在macOS和BSD上使用kqueue，在Windows上使用IOCP。
#[cfg(test)]
mod tests {

    use mio::net::TcpListener;
    use mio::*;
    use std::io::{self, Read, Write};
    const SERVER: Token = Token(0);
    fn tcp_server() -> io::Result<()> {
        // 这个示例实现了一个简单的echo服务器，展示了MIO的基本用法：
        // 创建Poll实例 进行事件轮询 注册TCP监听器处 理新连接 读写数据
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

    use mio::net::UdpSocket;
    use mio::{Events, Interest, Poll, Token};
    fn udp_server() -> io::Result<()> {
        // 这个示例展示了UDP服务器的实现：使用UdpSocket处理数据报 实现数据的接收和发送 简单的echo功能
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(1024);
        let addr = "127.0.0.1:8000".parse().unwrap();
        let mut socket = UdpSocket::bind(addr)?;
        poll.registry()
            .register(&mut socket, SERVER, Interest::READABLE)?;
        let mut buffer = [0; 1024];
        loop {
            poll.poll(&mut events, None)?;
            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let (len, addr) = socket.recv_from(&mut buffer)?;
                        socket.send_to(&buffer[..len], addr)?;
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    use mio::*;
    use std::time::{Duration, Instant};
    const TIMER: Token = Token(0);
    struct Timer {
        deadline: Instant,
    }
    impl Timer {
        fn new(timeout: Duration) -> Timer {
            Timer {
                deadline: Instant::now() + timeout,
            }
        }
        fn is_expired(&self) -> bool {
            Instant::now() >= self.deadline
        }
    }
    fn timer() -> std::io::Result<()> {
        // 如何实现定时器功能：创建自定义定时器 结构设置 超时检查 结合Poll使用
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(1024);
        let timer = Timer::new(Duration::from_secs(5));
        loop {
            let timeout = if timer.is_expired() {
                println!("Timer expired!");
                break;
            } else {
                Some(Duration::from_millis(100))
            };
            poll.poll(&mut events, timeout)?;
        }
        Ok(())
    }

    // 4. 异步文件读取
    use mio::unix::SourceFd;
    use std::fs::File;
    use std::os::unix::io::AsRawFd;
    const FILE: Token = Token(0);
    fn async_read_file() -> io::Result<()> {
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(1024);
        // 打开文件
        let mut file = File::open("rust-logo.png")?;
        let fd = file.as_raw_fd();
        // 将文件描述符包装为 SourceFd
        let mut source = SourceFd(&fd);
        // 注册到 poll
        poll.registry()
            .register(&mut source, FILE, Interest::READABLE)?;
        let mut buffer = Vec::new();
        let mut temp_buf = [0; 1024];
        'outer: loop {
            poll.poll(&mut events, None)?;
            for event in events.iter() {
                if event.token() == FILE {
                    loop {
                        match file.read(&mut temp_buf) {
                            Ok(0) => {
                                // 文件读取完成
                                break 'outer;
                            }
                            Ok(n) => {
                                buffer.extend_from_slice(&temp_buf[..n]);
                            }
                            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                // 需要等待更多数据
                                break;
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
            }
        }
        println!("Read {} bytes", buffer.len());
        // 将读取的内容转换为字符串（假设是UTF-8编码）
        match String::from_utf8(buffer) {
            Ok(content) => println!("Content: {}", content),
            Err(_) => println!("File contains non-UTF8 data"),
        }
        Ok(())
    }
    #[test]
    fn test_async_read_file() {
        async_read_file();
    }

    use mio::net::TcpStream;
    use mio::*;
    use std::collections::HashMap;
    struct Connection {
        socket: TcpStream,
        last_active: Instant,
    }
    fn main() -> io::Result<()> {
        // 6. 组合使用示例
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(1024);
        // 设置TCP服务器
        let addr = "127.0.0.1:8000".parse().unwrap();
        let mut server = TcpListener::bind(addr)?;
        poll.registry()
            .register(&mut server, SERVER, Interest::READABLE)?;
        let mut connections = HashMap::new();
        let timeout = Duration::from_secs(60); // 60秒超时
        loop {
            poll.poll(&mut events, Some(Duration::from_secs(1)))?;
            for event in events.iter() {
                match event.token() {
                    SERVER => {
                        let (mut socket, addr) = server.accept()?;
                        let token = Token(connections.len() + 2);
                        poll.registry().register(
                            &mut socket,
                            token,
                            Interest::READABLE | Interest::WRITABLE,
                        )?;
                        connections.insert(
                            token,
                            Connection {
                                socket,
                                last_active: Instant::now(),
                            },
                        );
                    }
                    token => {
                        if let Some(conn) = connections.get_mut(&token) {
                            if event.is_readable() {
                                let mut buffer = [0; 1024];
                                match conn.socket.read(&mut buffer) {
                                    Ok(n) if n > 0 => {
                                        conn.socket.write_all(&buffer[..n])?;
                                        conn.last_active = Instant::now();
                                    }
                                    _ => {
                                        connections.remove(&token);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // 清理超时连接
            connections.retain(|_, conn| Instant::now().duration_since(conn.last_active) < timeout);
        }
    }
}
