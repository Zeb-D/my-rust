// https://mp.weixin.qq.com/s/In-kA5wQ17xVLXtgPb-20A
// Rust命令行五彩斑斓！colored库让你的终端炫出新高度！
#[cfg(test)]
mod tests {
    use colored::Colorize;
    fn basic_colors() {
        //1. 简单着色
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
    fn test_basic_colors() {
        basic_colors()
    }

    fn background_colors() {
        // 背景色设置
        println!("{}", "红底文字".on_red());
        println!("{}", "绿底文字".on_green());
        println!("{}", "蓝底文字".on_blue());
        // 前景色和背景色组合
        println!("{}", "红字蓝底".red().on_blue());
        println!("{}", "蓝字红底".blue().on_red());
    }
    #[test]
    fn test_background_colors() {
        background_colors()
    }

    fn advanced_styling() {
        // 样式组合
        // 粗体
        println!("{}", "粗体文本".bold());
        // 下划线
        println!("{}", "下划线文本".underline());
        // 斜体
        println!("{}", "斜体文本".italic());
        // 删除线
        println!("{}", "删除线文本".strikethrough());
        println!("{}", "=== 系统菜单 ===".cyan().bold());
        // 组合多种样式
        println!(
            "{}",
            "粗体红色带下划线"
                .red()
                .bold()
                .italic()
                .underline()
                .strikethrough()
        );
    }
    #[test]
    fn test_advanced_styling() {
        advanced_styling()
    }

    fn custom_control() {
        //2. 自定义控制
        // 条件着色
        let status = true;
        println!(
            "状态: {}",
            if status {
                "正常".green()
            } else {
                "异常".red()
            }
        );
        // 动态色彩
        for i in 0..5 {
            let text = format!("动态颜色 {}", i);
            match i {
                0 => println!("{}", text.red()),
                1 => println!("{}", text.green()),
                2 => println!("{}", text.blue()),
                3 => println!("{}", text.yellow()),
                _ => println!("{}", text.cyan()),
            }
        }
    }
    #[test]
    fn test_custom_control() {
        custom_control()
    }

    fn log_system() {
        // 1. 日志级别颜色
        struct Logger;
        impl Logger {
            fn error(&self, msg: &str) {
                println!("{} {}", "[ERROR]".red().bold(), msg.red());
            }
            fn warn(&self, msg: &str) {
                println!("{} {}", "[WARN]".yellow().bold(), msg.yellow());
            }
            fn info(&self, msg: &str) {
                println!("{} {}", "[INFO]".green().bold(), msg.green());
            }
            fn debug(&self, msg: &str) {
                println!("{} {}", "[DEBUG]".blue().bold(), msg.blue());
            }
        }
        let logger = Logger;
        logger.error("系统错误");
        logger.warn("警告信息");
        logger.info("正常信息");
        logger.debug("调试信息");
    }
    #[test]
    fn test_log_system() {
        log_system()
    }

    //2. 进度展示
    fn progress_display() {
        use std::{thread, time::Duration};
        let steps = 10;
        for i in 0..=steps {
            print!("\r");
            print!("[");
            for j in 0..steps {
                if j < i {
                    print!("{}", "=".green());
                } else if j == i {
                    print!("{}", ">".yellow());
                } else {
                    print!(" ");
                }
            }
            print!("]{}", format!(" {}%", i * 10).blue());
            thread::sleep(Duration::from_millis(300));
        }
        println!("\n{}", "完成!".green().bold());
    }
    #[test]
    fn test_progress_display() {
        progress_display()
    }

    fn conditional_coloring() {
        use std::env;
        // 检查是否需要着色（例如在管道或重定向时禁用）
        if !colored::control::SHOULD_COLORIZE.should_colorize() {
            colored::control::set_override(false);
        }
        // 根据环境变量控制着色
        if env::var("NO_COLOR").is_ok() {
            colored::control::set_override(false);
        }
    }
    #[test]
    fn test_conditional_coloring() {
        conditional_coloring()
    }
}
