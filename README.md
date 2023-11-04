# rs-graphs
Graph algorithms in Rust

Rust 实现图数据库

```toml
[dependencies]
rs-graphs = "*"
```



初始化一个链表
```rust
use rs_graphs::graph::{Node, ArenaList, Graph};

let mut arena_list = ArenaList::new();
let mut graph = Graph::new(&mut arena_list);
```

## 基础功能

- 增
    - 增加节点 `graph.add_node(name: &str, data:T) -> usize`
    - 增加边 `graph.add_node_and_edge(src_name: &str, src_data: T, dst_name: &str, dst_data: T)`
- 查
    - 获取节点: `graph.get_node(idx: usize) -> &Node<T>`
    - 根据节点名字获取节点编号 `graph.get_idx_by_name(name: &str) -> Option<&usize>`
    - 根据节点编号获取节点名字 `graph.get_name_by_idx(idx: usize) -> &str`
    - [ ] 根据各种条件做查询
- 删
    - 删除节点，根据节点编号删除节点 `graph.del_node_by_idx()`，根据节点名字删除节点 `graph.del_node_by_name()`，
    - 删除边，`graph.del_edge_by_idx()`，`graph.del_edge_by_name()`
    - 清空所有数据，得到一个空图 `graph.clear()`
- 改
    - 改节点对应的值：`graph.add_node(name: &str, data: T) -> usize` 如果不存在，则创建节点。如果存在则修改其值。
- 持久化。目前只支持 `T` 为字符串的情况
  - 保存 `graph.save(filename)`
  - 加载 `graph.load(filename)`

## 图算法


获取全部下游

```Rust
// 寻找节点 2 和节点 3 的全部下游
let downstream = graph.get_downstream(vec![2, 3], 100000000);
for level in 0..downstream.len() {
    println!("[level = {}], idx = {:?}", level, downstream.get(&level));
}
```



寻找最近链路两个节点之间的最短长度

```Rust
println!("2 到 7 的最短路径长度为： {}", graph.get_shortest(2, 7, 1000000).unwrap());
```

- [ ] 寻找关键链接

