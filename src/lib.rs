use std::collections::HashMap;
use gfaR::Gfa;
use std::mem::size_of_val;


#[derive(Debug)]
pub struct NNode {
    pub id: u32,
    pub len: usize,
    pub seq: String,
}

#[derive(Debug)]
pub struct NEdge {
    pub from: u32,
    pub from_dir: bool,
    pub to: u32,
    pub to_dir: bool,
}

#[derive(Debug)]
pub struct NPath {
    pub name: String,
    pub dir: Vec<bool>,
    pub nodes: Vec<u32>,

}

#[derive(Debug)]
pub struct NGfa{
    pub nodes: HashMap<u32, NNode>,
    pub paths: Vec<NPath>,
    pub edges: Vec<NEdge>,
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
        for x in graph.paths.iter(){
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

pub fn accession_chr_separator(input_vec: Vec<String>, del: &str) -> HashMap<String, Vec<String>>{
    let mut h: HashMap<String, Vec<String>> = HashMap::new();

    for x in input_vec.iter() {
        let j: Vec<&str> = x.split(del).collect();
        let k = j[0].clone();
        if h.contains_key(&k.to_owned().clone()) {
            h.get_mut(&k.to_owned().clone()).unwrap().push(x.clone())
        } else {
            h.insert(k.to_owned().clone(), vec![x.clone()]);
        }
    }
    h

}


pub struct GraphWrapper<'a>{
    pub genomes: HashMap<String, Vec<&'a NPath>>,
}


impl <'a> GraphWrapper<'a>{
    pub fn new() -> Self{
        let mut h: HashMap<String, Vec<&'a NPath>> =  HashMap::new();
        Self{
            genomes: h,
        }
    }


    pub fn fromNGfa(& mut self, graph: &'a NGfa, del: &str){
        let mut h: HashMap<String, Vec<&'a NPath>> =  HashMap::new();
        for x in graph.paths.iter(){
        let j: Vec<&str> = x.name.split(del).collect();
        let k = j[0].clone();
        if h.contains_key(&k.to_owned().clone()){
            h.get_mut(&k.to_owned().clone()).unwrap().push(x)
        } else {
            h.insert(k.to_owned().clone(), vec![x]);
        }

        }
        self.genomes = h;
    }


}



#[cfg(test)]
mod tests {
    use crate::NGfa;

    #[test]
    fn it_works() {
        let mut g= NGfa::new();
        g.from_graph("/home/svorbrugg_local/Rust/data/AAA_AAB.cat.gfa");
        println!("{}", g.paths.len());
        assert_eq!(2 + 2, 4);
    }
}


