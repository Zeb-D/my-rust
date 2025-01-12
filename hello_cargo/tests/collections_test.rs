// https://mp.weixin.qq.com/s/nEzs1M7M9GKDg2lPt-Vssg  4 个 Rust 中你不常用但应该了解的数据结构
#[cfg(test)]
mod tests {
    #[test]
    fn vec_deque_test() {
        // 如果你在做滑动窗口算法或需要高效地从两端操作数据，VecDeque 将成为你不可或缺的伙伴。比起传统的队列，VecDeque 提供了更多的灵活性和性能。
        use std::collections::VecDeque;

        let mut deque: VecDeque<i32> = VecDeque::new();
        // 添加元素
        deque.push_front(1);  // 从前端添加
        deque.push_back(2);   // 从后端添加
        deque.push_back(3);   // 从后端添加

        // 获取元素
        if let Some(&front) = deque.front() {
            println!("前端元素是: {}", front);
        }
        if let Some(&back) = deque.back() {
            println!("后端元素是: {}", back);
        }

        // 移除元素
        if let Some(value) = deque.pop_front() {
            println!("从前端移除: {}", value);
        }
        if let Some(value) = deque.pop_back() {
            println!("从后端移除: {}", value);
        }

        for element in &deque {
            println!("{}", element);
        }

        deque.iter().for_each(|&element| println!("{}", element));

        // 使用索引遍历（不推荐，因为效率不高）
        for index in 0..deque.len() {
            println!("{}", deque[index]);
        }
    }

    #[test]
    fn btree_map_test() {
        // BTreeMap 使用 B 树结构来实现元素的有序存储，支持按键进行查找、插入、删除和遍历，且所有操作的时间复杂度都是 O(log n)，非常适合需要有序数据访问的场景。
        use std::collections::BTreeMap;

        let mut map: BTreeMap<&str, i32> = BTreeMap::new();
        // 插入键值对
        map.insert("Delhi", 10);
        map.insert("Mumbai", 4);
        map.insert("Bhopal", 12);

        // 获取值
        if let Some(&value) = map.get("Delhi") {
            println!("Delhi 的值是: {}", value);
        }

        // 更新值
        map.insert("Delhi", 5);  // 更新 Delhi 的值为 5
        // 移除键值对
        map.remove("Mumbai");

        // 遍历
        while let Some(elem) = map.pop_first() {
            println!("{},{}", elem.0, elem.1);
        }
        for i in 0..map.len() {

        }
    }

    #[test]
    fn btree_set_test() {
        use std::collections::BTreeSet;

        let mut set: BTreeSet<i32> = BTreeSet::new();

        // 插入元素
        set.insert(1);
        set.insert(2);
        set.insert(3);

        // 检查元素
        if set.contains(&1) {
            println!("集合中包含 1");
        }

        // 移除元素
        set.remove(&1);

        for &item in &set {
            println!("{}", item);
        }
        for item in set.iter() {
            println!("{}", item);
        }
        // 使用into_iter()方法（将所有权移动到迭代器）
        for item in set.into_iter() {
            println!("{}", item);
        }
    }
}
