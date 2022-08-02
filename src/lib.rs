use std::fs::File;
use std::io::{BufRead, BufReader};
use hashbrown::HashMap;
use gfaR::Gfa;
use std::path::Path as file_path;


#[derive(Debug, Clone)]
pub struct NNode {
    pub id: u32,
    pub len: usize,
    pub seq: String,
}

#[derive(Debug, Clone)]
pub struct NEdge {
    pub from: u32,
    pub from_dir: bool,
    pub to: u32,
    pub to_dir: bool,
}

#[derive(Debug, Clone)]
pub struct NPath {
    pub name: String,
    pub dir: Vec<bool>,
    pub nodes: Vec<u32>,

}

#[derive(Debug, Clone)]
/// The NumericGFA is GFA were node_ids are interger (u32)
pub struct NGfa{
    pub nodes: HashMap<u32, NNode>,
    pub paths: Vec<NPath>,
    pub edges: Vec<NEdge>,
    pub path2id: HashMap<String, usize>,
}

impl NGfa {
    pub fn new() -> Self {
        let nodes: HashMap<u32, NNode> = HashMap::new();
        let paths: Vec<NPath> = Vec::new();
        let edges: Vec<NEdge> = Vec::new();

        Self {
            nodes: nodes,
            paths: paths,
            edges: edges,
            path2id: HashMap::new(),
        }
    }

    pub fn from_graph(& mut self, filename: &str){
        let mut  graph = Gfa::new();
        graph.read_file(filename);
        let mut nodes : HashMap<u32, NNode> = HashMap::new();
        for (k,v) in graph.nodes.iter(){
            let n: u32 = k.parse().unwrap();
            nodes.insert(n.clone(), NNode {id: n.clone(),  len: v.len.clone(), seq: v.seq.clone()});
        }

        let mut nedges: Vec<NEdge> = Vec::new();
        for x in graph.edges.iter(){
            nedges.push(NEdge {from: x.from.parse::<u32>().unwrap(), from_dir: x.from_dir.clone(), to: x.to.parse::<u32>().unwrap(), to_dir: x.to_dir.clone()})
        }
        let mut ps: Vec<NPath> = Vec::new();
        for (i,x) in graph.paths.iter().enumerate(){
            self.path2id.insert(x.name.clone(), i);
            let mut  j: Vec<bool> = Vec::new();
            let mut j2: Vec<u32> = Vec::new();
            for index in 0..x.nodes.len(){
                j.push(x.dir[index].clone());
                j2.push(x.nodes[index].parse::<u32>().unwrap());
            }
            j.shrink_to_fit();
            j2.shrink_to_fit();
            ps.push(NPath{name: x.name.clone(), dir: j, nodes: j2})
        }
        nodes.shrink_to_fit();
        nedges.shrink_to_fit();
        self.nodes = nodes;
        self.paths = ps;
        self.edges = nedges;

    }


    pub fn from_file_direct(& mut self, filename: &str) {
        if file_path::new(filename).exists() {
            let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let l = line.unwrap();
                let line_split: Vec<&str> = l.split("\t").collect();
                if l.starts_with("S") {
                    if self.nodes.contains_key(&line_split[1].parse::<u32>().unwrap()) {
                        eprintln!("Warning: Duplicated node id found");
                    }
                    self.nodes.insert(line_split[1].parse().unwrap(), NNode { id: line_split[1].parse().unwrap(), seq: String::from(line_split[2]), len: line_split[2].len() });
                } else if l.starts_with("P") {
                    let name: String = String::from(line_split[1]);
                    let mut dirs: Vec<bool> = line_split[2].split(",").map(|d| if &d[d.len() - 1..] == "+" { !false } else { !true }).collect();
                    let mut nodd: Vec<u32> = line_split[2].split(",").map(|d| d[..d.len() - 1].parse().unwrap()).collect();
                    dirs.shrink_to_fit();
                    nodd.shrink_to_fit();

                    self.paths.push(NPath { name: name, dir: dirs, nodes: nodd });
                } else if l.starts_with("L") {
                    self.edges.push(NEdge { from: line_split[1].parse().unwrap(), to: line_split[3].parse().unwrap(), from_dir: if line_split[2] == "+" { !false } else { !true }, to_dir: if line_split[4] == "+" { !false } else { !true } })
                }
            }
        }

        self.nodes.shrink_to_fit();
        self.edges.shrink_to_fit();
    }

    pub fn from_file_direct2(& mut self, filename: &str) {
        if file_path::new(filename).exists() {
            let file = File::open(filename).expect("ERROR: CAN NOT READ FILE\n");
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let l = line.unwrap();
                let line_split: Vec<&str> = l.split("\t").collect();
                if l.starts_with("S") {
                    if self.nodes.contains_key(&line_split[1].parse::<u32>().unwrap()) {
                        eprintln!("Warning: Duplicated node id found");
                    }
                    self.nodes.insert(line_split[1].parse().unwrap(), NNode { id: line_split[1].parse().unwrap(), seq: "".to_owned(), len: line_split[2].len() });
                } else if l.starts_with("P") {
                    let name: String = String::from(line_split[1]);
                    let mut dirs: Vec<bool> = line_split[2].split(",").map(|d| if &d[d.len() - 1..] == "+" { !false } else { !true }).collect();
                    let mut nodd: Vec<u32> = line_split[2].split(",").map(|d| d[..d.len() - 1].parse().unwrap()).collect();
                    dirs.shrink_to_fit();
                    nodd.shrink_to_fit();

                    self.paths.push(NPath { name: name, dir: dirs, nodes: nodd });
                }
            }
        }

        self.nodes.shrink_to_fit();

    }



    pub fn remove_seq(& mut self){
        for (_k,v) in self.nodes.iter_mut(){
            v.seq = "".to_owned();
        }

    }
}



///
pub struct GraphWrapper<'a>{
    pub genomes: Vec<(String, Vec<&'a NPath>)>,
    pub path2genome: HashMap<&'a String, String>
}


impl <'a> GraphWrapper<'a>{
    pub fn new() -> Self{
        Self{
            genomes: Vec::new(),
            path2genome: HashMap::new(),

        }
    }



    /// NGFA -> Graphwrapper
    /// If delimiter == " " (nothing)
    ///     -> No merging
    pub fn from_ngfa(& mut self, graph: &'a NGfa, del: &str) {
        let mut h: HashMap<String, Vec<&'a NPath>> = HashMap::new();
        if del == " " {
            for x in graph.paths.iter() {
                h.insert(x.name.clone(), vec![x]);
            }
        } else {
            for x in graph.paths.iter() {
                let j: Vec<&str> = x.name.split(del).collect();
                let k = j[0].clone();
                if h.contains_key(&k.to_owned().clone()) {
                    h.get_mut(&k.to_owned().clone()).unwrap().push(x)
                } else {
                    h.insert(k.to_owned().clone(), vec![x]);
                }
            }
        }
        let mut v: Vec<(String, Vec<&'a NPath>)> = Vec::new();
        let mut keyy : Vec<String> = h.keys().cloned().collect();
        keyy.sort();
        for x in keyy.iter(){
            v.push((x.clone(), h.get(x).unwrap().clone()));
        }
        let mut j = HashMap::new();
        for (k,v) in v.iter(){
            for x in v.iter(){
                j.insert(&x.name, k.to_owned());
            }
        }

        self.path2genome = j;
        self.genomes = v;
    }


}



#[cfg(test)]
mod tests {
    use crate::{NGfa, GraphWrapper};

    #[test]
    fn general_test() {
        let mut ngfa = NGfa::new();
        ngfa.from_file_direct("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa");
        let mut gwrapper = GraphWrapper::new();
        println!("Number of paths: {}", ngfa.edges.len());
        gwrapper.from_ngfa(&ngfa, "_");
        println!("Number of paths: {}", ngfa.edges.capacity());
        println!("Number of genomes: {}", gwrapper.genomes.capacity());
        println!("Path2genome: {:?}", gwrapper.path2genome);
        gwrapper.from_ngfa(&ngfa, " ");
        println!("Number of genome: {}", gwrapper.genomes.len());
        println!("Path2genome: {:?}", gwrapper.path2genome);

        let mut ngfa = NGfa::new();
        ngfa.from_file_direct2("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa");
        ngfa.remove_seq();
        let mut ngfa = NGfa::new();
        ngfa.from_file_direct("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa");

    }
}


