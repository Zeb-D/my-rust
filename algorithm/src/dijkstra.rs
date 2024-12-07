// Dijkstra算法（最短路径）
// Dijkstra算法是一种经典算法，用于在图中寻找两个节点之间的最短路径。可以使用Rust的 HashMap和 BinaryHeap结构来高效实现。
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    graph: &HashMap<usize, Vec<(usize, usize)>>,
    start: usize,
    goal: usize,
) -> Option<usize> {
    let mut dist: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });
    while let Some(State { cost, position }) = heap.pop() {
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
                    heap.push(State {
                        cost: next_cost,
                        position: next_pos,
                    });
                    dist.insert(next_pos, next_cost);
                }
            }
        }
    }
    None
}
