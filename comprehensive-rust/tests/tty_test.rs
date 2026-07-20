#[cfg(test)]
mod tests {

    #[test]
    fn test_ansi_escape_codes() {
        // 1. 基础颜色 (SGR: Select Graphic Rendition)
        // 31=红, 32=绿, 33=黄, 34=蓝, 0=重置
        println!("Red: \x1b[31mThis is Red\x1b[0m");
        println!("aa \x1b[32mThis is Green\x1b[0m");
        println!("\x1b[1m\x1b[34mBold Blue\x1b[0m"); // 1=粗体

        // 2. 光标移动 & 清除行 (用于进度条)
        print!("\x1b[2J"); // 清空整个屏幕 (Clear Screen)
        print!("\x1b[1;1H"); // 移动光标到 (行1, 列1) (Home)

        print!("Loading");
        std::thread::sleep(std::time::Duration::from_millis(500));

        print!("\x1b[1G"); // 移动光标到当前行首 (Column 1)
        print!("\x1b[K"); // 清除从光标到行尾 (Erase Line)
        println!("Loading... Done!");

        // 3. 手动构造复杂样式
        let bold_italic_underline = "\x1b[1;3;4m"; // 1粗体, 3斜体, 4下划线
        let reset = "\x1b[0m";
        println!("{}{}{}", bold_italic_underline, "Fancy Text", reset);
    }
}
