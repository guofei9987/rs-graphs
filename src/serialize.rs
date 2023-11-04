use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;
use std::io::Write;
use crate::graph::Graph;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
// TODO:这个需要用户自己定义T的序列化方法

#[derive(Serialize, Deserialize)]
struct SaveObj<T> {
    all_node_names: Vec<String>,
    all_data: Vec<T>,
    all_edges: Vec<(usize, usize)>,
}

impl<'a, T: Serialize + DeserializeOwned + Clone+Debug> Graph<'a, T> {
    // 保存
    pub fn save(&self, filename: &str) {
        let all_node_names = self.get_all_node_names()
            .into_iter().map(|x| x.clone()).collect::<Vec<String>>();
        let all_edges: Vec<(usize, usize)> = self.get_all_edges();

        let all_data = self.get_all_data().clone();

        let save_obj = SaveObj {
            all_node_names,
            all_data,
            all_edges,
        };

        let serialized: String = serde_json::to_string(&save_obj).unwrap();

        let mut file = std::fs::File::create(filename).unwrap();
        file.write(serialized.as_bytes()).unwrap();
    }


    // 加载
    pub fn load(&mut self, filename: &str) {
        self.clear();
        let serialized = fs::read_to_string(filename).unwrap();
        let save_obj: SaveObj<T> = serde_json::from_str(&serialized).unwrap();


        let all_node_names = save_obj.all_node_names.clone();
        let all_edge = save_obj.all_edges.clone();
        let mut all_data = save_obj.all_data.clone();
        println!("{:?}",all_node_names);
        println!("{:?}",all_edge);
        println!("{:?}",all_data);


        for idx in (0..all_node_names.len()).rev() {
            self.add_node(all_node_names[idx].clone().as_str(), all_data[idx].clone());
        }


        for (src_idx, dst_idx) in all_edge {
            self.add_edge(src_idx, dst_idx);
        }
    }
}