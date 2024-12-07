// https://mp.weixin.qq.com/s/pMS3ac9sEF6w7_F8t9c-2g 让你的Rust命令行颜值爆表！indicatif进度条终极指南
#[cfg(test)]
mod tests {
    use indicatif::MultiProgress;
    use indicatif::ProgressBar;
    use indicatif::ProgressStyle;

    fn basic_progress_bar() {
        //1. 基础进度条
        let pb = ProgressBar::new(100);
        for i in 0..100 {
            pb.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        pb.finish_with_message("done");
    }

    #[test]
    fn test_basic_progress_bar() {
        basic_progress_bar()
    }

    fn styled_progress_bar() {
        //自定义样式
        let pb = ProgressBar::new(100);
        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        for i in 0..100 {
            pb.inc(1);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    #[test]
    fn test_styled_progress_bar() {
        styled_progress_bar()
    }

    fn multi_progress_bars() {
        // 多进度条
        let m = MultiProgress::new();
        let pb1 = m.add(ProgressBar::new(100));
        let pb2 = m.add(ProgressBar::new(100));
        let pb3 = m.add(ProgressBar::new(100));
        pb1.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
                )
                .unwrap()
                .progress_chars("#>-"),
        );
        std::thread::scope(|s| {
            s.spawn(|| {
                for i in 0..100 {
                    pb1.inc(1);
                    std::thread::sleep(std::time::Duration::from_millis(50));
                }
            });
            s.spawn(|| {
                for i in 0..100 {
                    pb2.inc(1);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                }
            });
            s.spawn(|| {
                for i in 0..100 {
                    pb3.inc(1);
                    std::thread::sleep(std::time::Duration::from_millis(75));
                }
            });
        });
    }

    #[test]
    fn test_multi_progress_bars() {
        multi_progress_bars()
    }

    fn iterator_wrapper() {
        // 2. 迭代器包装
        let items = vec![1, 2, 4, 8];
        let pb = ProgressBar::new(items.len() as u64);
        for item in pb.wrap_iter(items.iter()) {
            std::thread::sleep(std::time::Duration::from_millis(500)); // 处理item...
        }
    }
    #[test]
    fn test_iterator_wrapper() {
        iterator_wrapper()
    }

    fn custom_state_display() {
        //自定义状态展示
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                // .tick_strings(&["abcd", "bcde", "cdef", "defg", "efgh", "fghi"]),
                .tick_strings(&[
                    "▹▹▹▹▹▹",
                    "▸▹▹▹▹▹",
                    "▹▸▹▹▹▹",
                    "▹▹▸▹▹▹",
                    "▹▹▹▸▹▹",
                    "▹▹▹▹▸▹",
                    "▹▹▹▹▹▸",
                ]),
        );
        for i in 0..100 {
            pb.set_message(format!(
                "{} : Processing... {}",
                i,
                chrono::Local::now().format("%H:%M:%S")
            ));
            std::thread::sleep(std::time::Duration::from_millis(50));
            pb.tick();
        }
    }

    #[test]
    fn test_custom_state_display() {
        custom_state_display()
    }

    fn nested_progress() {
        //嵌套进度显示
        let main_pb = ProgressBar::new(3);
        main_pb.set_style(
            ProgressStyle::default_bar()
                .template("[{bar:40}] {pos}/{len}")
                .unwrap(),
        );
        for i in 0..3 {
            let sub_pb = ProgressBar::new(100);
            sub_pb.set_style(
                ProgressStyle::default_bar()
                    .template("  ╰─▶ {spinner:.green} [{bar:40.cyan/blue}] {pos}/{len}")
                    .unwrap(),
            );
            for j in 0..100 {
                sub_pb.inc(1);
                std::thread::sleep(std::time::Duration::from_millis(20));
            }
            sub_pb.finish_and_clear();
            main_pb.inc(1);
        }
    }

    #[test]
    fn test_nested_progress() {
        nested_progress()
    }

    fn download_progress() { // 下载进度显示
        let pb = ProgressBar::new(1024);
        pb.set_style(ProgressStyle::default_bar()        
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")        
        .unwrap()        
        .progress_chars("=>-")); // 模拟下载
        let chunk_size = 32;
        for _ in 0..1024 / chunk_size {
            pb.inc(chunk_size);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    #[test]
    fn test_download_progress(){
        download_progress()
    }

    use std::collections::HashMap;
    fn process_tasks() {     // 2. 任务处理进度
        let tasks: HashMap<&str, u64> = [        
            ("数据下载", 100),        
            ("数据处理", 200),        
            ("生成报告", 50),    
            ].into_iter().collect();    
        let pb = ProgressBar::new(tasks.values().sum());    
        pb.set_style(ProgressStyle::default_bar()        
        .template("{msg}\n{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")        
        .unwrap());    
        for (task, size) in tasks {       
            pb.set_message(format!("正在{}...", task));        
            for _ in 0..size {            
                pb.inc(1);            
                std::thread::sleep(std::time::Duration::from_millis(50));        
            }    
        }
    }

    #[test]
    fn test_process_tasks(){
        process_tasks()
    }
}
