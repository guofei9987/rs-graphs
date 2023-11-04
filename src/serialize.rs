use std::fs;
use std::io::Write;
use crate::graph::Graph;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SaveObj {
    all_node_names: Vec<String>,
    // TODO:这个需要用户自己定义序列化方法
    // node_data:Vec<T>,
    all_edges: Vec<(usize, usize)>,
}

impl<'a, T> Graph<'a, T> {
    // 保存
    pub fn save(&self, filename: &str) {
        let all_node_names = self.get_all_node_names()
            .into_iter().map(|x| x.clone()).collect::<Vec<String>>();
        let all_edges: Vec<(usize, usize)> = self.get_all_edges();

        let save_obj = SaveObj {
            all_node_names,
            all_edges,
        };

        let serialized: String = serde_json::to_string(&save_obj).unwrap();

        let mut file = std::fs::File::create(filename).unwrap();
        file.write(serialized.as_bytes()).unwrap();
    }


    // 加载
    // fn load( filename: &str)->Self<T> {
    //
    //     let serialized = fs::read_to_string(filename).unwrap();
    //
    //
    //     let save_obj: SaveObj = serde_json::from_str(&serialized).unwrap();
    //     let all_node_names = save_obj.all_node_names;
    //     let all_edge = save_obj.all_edges;
    //
    //
    //     Self{
    //
    //     }

}