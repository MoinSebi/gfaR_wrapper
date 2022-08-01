use std::collections::HashMap;
use gfaR::Gfa;

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
            ps.push(NPath{name: x.name.clone(), dir: j, nodes: j2})
        }
        self.nodes = nodes;
        self.paths = ps;
        self.edges = nedges;

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
        ngfa.from_graph("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa");
        let mut gwrapper = GraphWrapper::new();
        gwrapper.from_ngfa(&ngfa, "_");
        println!("Number of paths: {}", ngfa.paths.len());
        println!("Number of genomes: {}", gwrapper.genomes.len());
        println!("Path2genome: {:?}", gwrapper.path2genome);
        gwrapper.from_ngfa(&ngfa, " ");
        println!("Number of genome: {}", gwrapper.genomes.len());
        println!("Path2genome: {:?}", gwrapper.path2genome);
    }
}


