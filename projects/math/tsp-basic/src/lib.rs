use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

use std::cmp::{Ordering, PartialEq, PartialOrd};

#[derive(Debug, Clone)]
pub struct TspNode {
    pub node_no: usize,
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct TspData {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    comment: String,
    #[allow(dead_code)]
    tsp_type: String,
    pub dimension: usize,
    #[allow(dead_code)]
    edge_weight_type: String,
    pub nodes: Vec::<TspNode>
}

impl PartialEq for TspNode {
    fn eq(&self, other: &Self) -> bool {
        self.node_no.eq(&other.node_no)
    }
    fn ne(&self, other: &Self) -> bool {
        self.node_no.ne(&other.node_no)
    }
}

impl PartialOrd for TspNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.node_no.partial_cmp(&other.node_no)
    }
}

impl TspData {
    pub fn load(tsp_path: impl AsRef<Path>) -> Result<TspData, String> {
        let mut file_handle = match File::open(&tsp_path) {
            Ok(file) => std::io::BufReader::new(file),
            Err(error) => {
                return Err(
                    format!("Problem opening the file: path={:?} error={:?}", tsp_path.as_ref(), error)
                )
            },
        };

        let mut contents = String::new(); 
        let _ = file_handle.read_to_string(&mut contents).unwrap();

        // Read header
        let re = Regex::new(r"^(?P<key>[_A-Z]+)\s*:\s*(?P<value>.*)$").unwrap();
        let mut tsp_name: String = String::from("");
        let mut tsp_comment: String = String::from("");
        let mut tsp_type: String = String::from("");
        let mut dimension: usize = 0;
        let mut edge_weight_type: String = String::from("");

        let mut line_iter = contents.split('\n');
        while let Some(line) = line_iter.next() {
            if let Some(header) = re.captures(line) {
                match &header["key"][..] {
                    "NAME" => tsp_name = String::from(&header["value"]),
                    "COMMENT" => tsp_comment = String::from(&header["value"]),
                    "TYPE" => tsp_type = String::from(&header["value"]),
                    "DIMENSION" => dimension = header["value"].parse::<usize>().unwrap(),
                    "EDGE_WEIGHT_TYPE" => edge_weight_type = String::from(&header["value"]),
                    _ => println!("{:?}", header),
                }
                continue;
            } else if line.starts_with("NODE_COORD_SECTION") {
                break;
            }
        }

        // Read data
        let mut tsp_nodes = Vec::<TspNode>::with_capacity(dimension);
        while let Some(line) = line_iter.next() {
            if line.trim().len() == 0 {
                continue;
            } else if line.starts_with("EOF") {
                break;
            } else {
                let v: Vec<i32> = line.split_whitespace().map(
                    |s| s.parse::<i32>().unwrap()
                ).collect();
                tsp_nodes.push(
                    TspNode { node_no: v[0] as usize, x: v[1], y: v[2] }
                );
            }
        }

        Ok(TspData {
            name: tsp_name,
            comment: tsp_comment,
            tsp_type: tsp_type,
            dimension: dimension,
            edge_weight_type: edge_weight_type,
            nodes: tsp_nodes,
        })
    }

    pub fn calc_distance(&self, route: &Vec::<usize>) -> f32 {
        let distance = |n1: &TspNode, n2: &TspNode| -> f32 {
            (((n1.x - n2.x).pow(2) + (n1.y - n2.y).pow(2)) as f32).sqrt()
        };

        let mut total_dist_cur = 0.0;
        for i in 0..self.dimension {
            let n1 = &self.nodes[route[i] - 1];
            let n2 = &self.nodes[route[(i + 1) % self.dimension] - 1];
            total_dist_cur += distance(&n1, &n2);
        }

        return total_dist_cur;
    }    
}
