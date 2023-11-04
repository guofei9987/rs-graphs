use std::collections::{HashMap, HashSet};
use rs_graphs::graph::{Node, ArenaList, Graph};

#[test]
fn test1() {
    let mut arena_list = ArenaList::new();
    let mut graph = Graph::new(&mut arena_list);
    let vec1 = vec![
        ("John", "Emma")
        , ("Sophia", "Tom")
        , ("Isabella", "Emma")
        , ("Tom", "Isabella")
        , ("Tom", "John")
        , ("Tom", "Michael")
        , ("John", "Emma")
        , ("Tom", "Sophia")
        , ("Oliver", "Emma")
        , ("Michael", "Daniel")
        , ("Michael", "Lucy")
        , ("Sophia", "Michael")
        , ("Oliver", "Lucy")
        , ("Sophia", "Emily")
        , ("Michael", "Daniel")
        , ("Sophia", "Michael")
        , ("Michael", "Sophia")
        , ("John", "Emma")
        , ("Tom", "Sophia")
        , ("Sophia", "John")]
        ;

    for (src_name, dst_name) in vec1 {
        graph.add_node_and_edge(
            src_name, src_name.to_string(),
            dst_name, dst_name.to_string());
    }

    // 获取所有节点名
    let all_node_names: Vec<&String> = graph.get_all_node_names();
    // 获取所有的边(返回值是节点的 index)
    let all_edges: Vec<(usize, usize)> = graph.get_all_edges();


    // 打印所有的边
    graph.print_edges();

    // 打印所有节点
    graph.print_nodes();

    graph.del_edge_by_name("Michael", "Lucy");
    println!("======after del edge [Sophia]-> [Lucy]：======");
    graph.print_edges();

    graph.del_node_by_name("Sophia");
    println!("======after del node 【Sophia】：======");
    graph.print_edges();
    graph.clear();
}


#[test]
fn test2() {
    let mut arena_list = ArenaList::new();
    let mut graph = Graph::new(&mut arena_list);
    let vec1 = vec![
        ("John", "Emma")
        , ("Sophia", "Tom")
        , ("Isabella", "Emma")
        , ("Tom", "Isabella")
        , ("Tom", "John")
        , ("Tom", "Michael")
        , ("John", "Emma")
        , ("Tom", "Sophia")
        , ("Oliver", "Emma")
        , ("Michael", "Daniel")
        , ("Michael", "Lucy")
        , ("Sophia", "Michael")
        , ("Oliver", "Lucy")
        , ("Sophia", "Emily")
        , ("Michael", "Daniel")
        , ("Sophia", "Michael")
        , ("Michael", "Sophia")
        , ("John", "Emma")
        , ("Tom", "Sophia")
        , ("Sophia", "John")]
        ;

    for (src_name, dst_name) in vec1 {
        graph.add_node_and_edge(
            src_name, src_name.to_string(),
            dst_name, dst_name.to_string());
    }

    // 寻找节点 2 和节点 3 的全部下游
    let downstream = graph.get_downstream(vec![2, 3], 100000000);
    for level in 0..downstream.len() {
        println!("[level = {}], idx = {:?}", level, downstream.get(&level));
    }

    println!("=====print names=====");
    for level in 0..downstream.len() {
        let node_names: Vec<&String> =
            downstream.get(&level).unwrap()
                .iter().map(|idx| graph.get_name_by_idx(*idx))
                .collect();
        println!("[level = {}], names = {:?}", level, node_names);
    }

    println!("2 到 7 的最短路径长度为： {}", graph.get_shortest(2, 7, 1000000).unwrap());
}
