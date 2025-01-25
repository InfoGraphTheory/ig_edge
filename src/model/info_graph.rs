use std::collections::HashMap;
use crate::info_edge::InfoEdge;
use ig_tr::{InfoTriple, InfoTable};
use ig_desc::Descriptor;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InfoGraph {
    pub edges: HashMap<String, InfoEdge>, 
    pub descriptor: Descriptor,    
}

impl Default for InfoGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct InfoGraphError;

impl fmt::Display for InfoGraphError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InfoEdge not found")
    }
}

impl InfoGraph {

    pub fn new() -> Self {
        InfoGraph {
            edges: HashMap::new(),
            descriptor: Default::default(),
        }
    }

    pub fn get_info_edge(&self, id: &str) -> Option<&InfoEdge>{
        self.edges.get(id)
    }

    pub fn get_refering_edges(&self, id: &str) -> Vec<InfoEdge> {
        let mut refers = vec![]; 
        let iterator = self.edges.iter();
        let mut iterator = iterator.map(|(_key, value)| value).collect::<Vec<_>>();
        for info_edge in &mut iterator{
            if info_edge.does_refer(id) { 
                refers.push(info_edge.clone());
            }
        }
        refers
    }

    pub fn add_info_edge(&mut self, info_edge: InfoEdge) -> &mut InfoEdge {
        self.edges.entry(info_edge.get_point().to_string()).or_insert(info_edge)
    }

    pub fn add_info_triple(&mut self, info_triple: InfoTriple, label: &str, label1: &str, label2: &str){
        let mut info_edge: InfoEdge = info_triple.into();
        info_edge.descriptor.set_label(label);
        info_edge.vertex1.set_label(label1);
        info_edge.vertex2.set_label(label2);
        self.add_info_edge(info_edge);
    }

    //find edge on descriptor id, remove edge on descriptor id, set description on edge, vertex1 or
    //vertex2 based on edge id, same set functions based on descriptor id.

    pub fn set_description(&mut self, id: &str, description: &str) -> Result<(), InfoGraphError>{
        let mut edge = self.get_info_edge(id);
        if let Some(e) = &mut edge {
            let mut e2 = e.clone();
            e2.descriptor.set_description(description);
            self.edges.insert(id.to_string(), e.clone());
            return Ok(());
        }

        Err(InfoGraphError)
    }


    #[allow(dead_code)]
    fn serialize(){
        
    }

    pub fn remove(&mut self, id: &str) -> Option<InfoEdge> {
        self.edges.remove(id)
    }

    #[allow(dead_code)]
    pub(crate) fn mock() -> InfoGraph {
        let mut ig: InfoGraph = Self::mock_empty(); 
        ig.add_info_edge(InfoEdge::mock());
        ig
    }

    pub(crate) fn mock_empty() -> InfoGraph {
        let mut ig: InfoGraph = InfoGraph::new();    
        ig.descriptor = Descriptor::mock_with_id("InfoGraphMock");
        ig
    }

    #[allow(dead_code)]
    pub(crate) fn mock_with_some_edges() -> InfoGraph {
        let mut ig: InfoGraph = Self::mock_empty(); 
        ig.add_info_edge(InfoEdge::mock_with_id("1"));
        ig.add_info_edge(InfoEdge::mock_with_id("2"));
        ig.add_info_edge(InfoEdge::mock_with_id("3"));
        ig
    }
}


impl FromIterator<InfoTriple> for InfoGraph {
    
    fn from_iter<T: IntoIterator<Item = InfoTriple>>(iter: T) -> Self {
        
        let mut ig = InfoGraph::default();
        
        iter.into_iter()
            .for_each(|t|{ig.add_info_triple(t, "", "", "")});

        ig
    }
}

pub trait ToOneString {
    fn to_one_string (&self) -> String;
}

///
/// Constructs an InfoTable from an InfoGraph.
///
impl From<InfoGraph> for InfoTable {
    
    fn from(graph: InfoGraph) -> Self {
         
        let mut rows: HashMap<String,(String,String)> = HashMap::new();
        //let mut rows: Vec<InfoTriple> = vec!();
        
        for (_key, info_edge) in &graph.edges{
            let triple: InfoTriple = InfoTriple::from(info_edge.clone());
            rows.insert(triple.id, (triple.id1, triple.id2));
        }

        InfoTable{
            rows: Into::into(rows),    
        }

    }
}

