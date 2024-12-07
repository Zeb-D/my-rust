//A*算法（路径寻找）
//A算法是一种用于寻找目标最短路径的算法，特别常用于游戏和导航系统中。可以使用Rust的数据结构和 BinaryHeap类来实现A算法。
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Node {
    cost: usize,
    heuristic: usize,
    position: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(
    graph: &HashMap<usize, Vec<(usize, usize)>>,
    start: usize,
    goal: usize,
    heuristic: &HashMap<usize, usize>,
) -> Option<usize> {
    let mut dist: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(Node {
        cost: 0,
        heuristic: *heuristic.get(&start).unwrap_or(&0),
        position: start,
    });
    
    while let Some(Node { cost, position, .. }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }
        if let Some(neighbors) = graph.get(&position) {
            for &(next_pos, weight) in neighbors {
                let next_cost = cost + weight;
                if next_cost < *dist.get(&next_pos).unwrap_or(&usize::MAX) {
                    heap.push(Node {
                        cost: next_cost,
                        heuristic: *heuristic.get(&next_pos).unwrap_or(&0),
                        position: next_pos,
                    });
                    dist.insert(next_pos, next_cost);
                }
            }
        }
    }
    None
}
