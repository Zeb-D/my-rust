#[cfg(test)]
mod tests {
    use std::ops::Drop;

    #[derive(Debug)]
    struct S(i32);

    impl Drop for S {
        fn drop(&mut self) {
            println!("drop for {}",self.0)
        }
    }

    #[test]
    fn test_drop() {
        let x= S(1);
        println!("1st create x: {:?}",x);
        {
            let y = S(2);
            println!("create y: {:?}",y);
            println!("exit inner scope")
        }
        println!("exit main");
        // 变量遮蔽 不会导致其生命周期提前结束
        let x1 = S(3);
        println!("create x1: {:?}",x1);
        let x1 = S(4);
        println!("create shadowing x1: {:?}",x1);
    }

    // 接下来来演示drop失效的场景，同时也演示OOM场景
    use  std::rc::Rc;
    use std::cell::RefCell;
    type NodePtr<T> = Option<Rc<RefCell<Node<T>>>>;

    #[derive(Debug)]
    struct  Node<T>{
        data: T,
        next: NodePtr<T>,
    }
    impl<T> Drop for Node<T> {
        fn drop(&mut self) {
            println!("dropping...")
        }
    }

    #[test]
    fn test_oom_for_drop(){
        let first = Rc::new(RefCell::new(Node{
            data:1,
            next: None,
        }));
        let second = Rc::new(RefCell::new(Node{
            data:2,
            next: None,
        }));

        first.borrow_mut().next = Some(second.clone());
        second.borrow_mut().next = Some(first.clone());

        // 不能直接打印Node 会报 fatal runtime error: stack overflow：因为死循环了
        println!("first: {:?}",first.borrow().data);
        println!("second: {:?}",second.borrow_mut().data)
    }
}
