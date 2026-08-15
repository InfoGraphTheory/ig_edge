
use std::collections::HashMap;

use crate::edge_director::EdgeDirector;
use ig_desc::descriptor_store_fs::DescriptorStoreFS;
use ig_desc::descriptor_facade::DescriptorFacade;
use ig_desc::{App, Space};
use ig_tr::triple_store_fs::TripleStoreFS;
use ig_tr::triple_facade::TripleFacade;

#[derive(Clone)]
pub struct EdgeServiceFS {
    pub edges: EdgeDirector<TripleStoreFS, DescriptorStoreFS>,
    pub space_id: String,
}

impl EdgeServiceFS {
    pub fn new(app_name: App, space_id: Space, config: String) -> Self {

        let space_id_string = space_id.to_string();

        let triples: TripleStoreFS = TripleStoreFS::new(space_id_string.clone());
        let triple_facade = TripleFacade{ storage: triples};

        let descs = DescriptorStoreFS::new(app_name, space_id, config);
        let desc_facade = DescriptorFacade::new(descs);

        EdgeServiceFS {
            edges: EdgeDirector::new(triple_facade, desc_facade),
            space_id: space_id_string,
        }
    }

    ///
    /// Returns a list of InfoEdge objects for a given graph_name.
    /// This means triples and their descriptor notes.
    ///
    pub fn get_all_edge_names(&mut self) -> HashMap<String, String> {
        self.edges.get_all_edge_names()
    }

    ///
    /// Returns a list of the names of all vertexes that is connected to a vertex id specified in
    /// the parameter ref_id.
    /// If the connected vertex does not have a name in a descriptor note, the id of the vertex is returned
    /// instead. 
    pub fn get_all_ref_edge_names(&mut self, graph_name: String, ref_id: String, exception: String) -> HashMap<String, String> {
        self.edges.get_all_ref_edge_names_except(graph_name, ref_id, exception)
    }

    pub fn get_all_ref_edge_labels(&mut self, ref_id: String) -> HashMap<String, String> {
        self.edges.get_all_ref_edge_labels(ref_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Guards the App/Space/config constructor signature (a breaking change made this session
    // to match DescriptorStoreFS::new) against accidental regression.
    #[test]
    fn new_constructs_successfully_with_app_space_and_config() {
        let space_id = format!("ig_edge_test_service_{}", std::process::id());
        let service = EdgeServiceFS::new(
            App::from("ig_edge_test_app".to_string()),
            Space::from(space_id.clone()),
            "ig_edge_test_config".to_string(),
        );
        assert_eq!(service.space_id, space_id);
    }
}
