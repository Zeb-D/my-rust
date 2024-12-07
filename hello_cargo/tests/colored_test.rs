// https://mp.weixin.qq.com/s/In-kA5wQ17xVLXtgPb-20A
// Rust命令行五彩斑斓！colored库让你的终端炫出新高度！
#[cfg(test)]
mod tests {
    use colored::Colorize;
    fn basic_colors() { //1. 简单着色
        println!("{}", "红色文本".red());
        println!("{}", "绿色文本".green());
        println!("{}", "蓝色文本".blue());
        println!("{}", "黄色文本".yellow());
        println!("{}", "青色文本".cyan());
        println!("{}", "洋红文本".magenta());
        println!("{}", "白色文本".white());
        println!("{}", "黑色文本".black());
    }
    #[test]
    fn test_basic_colors(){
        basic_colors()
    }

    fn background_colors() {    
        println!("{}", "红底文字".on_red());    println!("{}", "绿底文字".on_green());    println!("{}", "蓝底文字".on_blue());
    // 前景色和背景色组合    
    println!("{}", "红字蓝底".red().on_blue());    println!("{}", "蓝字红底".blue().on_red());
}
}
