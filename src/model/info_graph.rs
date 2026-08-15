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
        self.edges
            .values()
            .filter(|info_edge| info_edge.does_refer(id))
            .cloned()
            .collect()
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
        let Some(edge) = self.get_info_edge(id) else {
            return Err(InfoGraphError);
        };
        let mut edge = edge.clone();
        edge.descriptor.set_description(description);
        self.edges.insert(id.to_string(), edge);
        Ok(())
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
        
        for info_edge in graph.edges.values() {
            let triple: InfoTriple = InfoTriple::from(info_edge.clone());
            rows.insert(triple.id, (triple.id1, triple.id2));
        }

        InfoTable{
            rows: Into::into(rows),
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_description_updates_the_stored_edge() {
        // Regression test: set_description used to clone the edge, mutate the clone, then
        // re-insert the original unmutated edge, making it silently a no-op.
        let mut ig = InfoGraph::mock_with_some_edges();
        let id = "InfoEdge1".to_string();
        assert!(ig.get_info_edge(&id).is_some());

        assert!(ig.set_description(&id, "a new description").is_ok());

        let updated = ig.get_info_edge(&id).unwrap();
        assert_eq!(updated.descriptor.description.as_deref(), Some("a new description"));
    }

    #[test]
    fn set_description_errors_for_unknown_id() {
        let mut ig = InfoGraph::mock_empty();
        assert!(ig.set_description("does-not-exist", "x").is_err());
    }

    #[test]
    fn get_refering_edges_finds_only_edges_that_refer_to_the_id() {
        let mut ig = InfoGraph::mock_empty();
        ig.add_info_edge(InfoEdge::new("e1", "a", "b"));
        ig.add_info_edge(InfoEdge::new("e2", "b", "c"));
        ig.add_info_edge(InfoEdge::new("e3", "x", "y"));

        let mut referring: Vec<String> = ig.get_refering_edges("b")
            .iter()
            .map(|e| e.get_point().to_string())
            .collect();
        referring.sort();

        assert_eq!(referring, vec!["e1".to_string(), "e2".to_string()]);
    }

    #[test]
    fn add_info_edge_does_not_overwrite_an_existing_edge_with_the_same_point() {
        let mut ig = InfoGraph::mock_empty();
        ig.add_info_edge(InfoEdge::new("e1", "a", "b"));
        ig.add_info_edge(InfoEdge::new("e1", "different", "vertices"));

        let edge = ig.get_info_edge("e1").unwrap();
        assert_eq!(edge.get_label("a"), "");
        assert!(!edge.does_refer("different"));
    }

    #[test]
    fn remove_deletes_the_edge_and_returns_it() {
        let mut ig = InfoGraph::mock_empty();
        ig.add_info_edge(InfoEdge::new("e1", "a", "b"));

        let removed = ig.remove("e1");
        assert!(removed.is_some());
        assert!(ig.get_info_edge("e1").is_none());
    }

    #[test]
    fn from_iter_of_info_triples_builds_an_edge_per_triple() {
        let triples = vec![
            InfoTriple::new("e1", "a", "b"),
            InfoTriple::new("e2", "b", "c"),
        ];

        let ig: InfoGraph = triples.into_iter().collect();

        assert!(ig.get_info_edge("e1").is_some());
        assert!(ig.get_info_edge("e2").is_some());
    }
}

