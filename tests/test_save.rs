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
        , ("Sophia", "John")
        , ("中文", "John")
    ]
        ;

    for (src_name, dst_name) in vec1 {
        graph.add_node_and_edge(
            src_name, src_name.to_string(),
            dst_name, dst_name.to_string());
    }


    let filename="save.db";
    graph.save(filename);


}
