use std::collections::HashMap;

use ig_desc::{descriptor_facade::DescriptorFacade, descriptor_store::DescriptorStore};
use ig_tr::{triple_facade::TripleFacade, triple_store::TripleStore, InfoTable};

use crate::model::{NameOrPoint, Point, LabelOrPoint};


#[derive(Clone)]
pub struct EdgeDirector <T: TripleStore, S: DescriptorStore> {
    triples: TripleFacade<T>,
    descs: DescriptorFacade<S>
}

impl<T:TripleStore, S:DescriptorStore> EdgeDirector<T,S> {

    pub fn new(triples: TripleFacade<T>, descs: DescriptorFacade<S>) -> EdgeDirector<T,S> {
        EdgeDirector{triples,descs}
    }

    pub fn prettify(&self, buffer: String) -> String {
           
                let vec: Vec<&str> = Vec::from_iter(buffer.lines());
                
                let descs = self.descs.get_descs(vec);

                descs.iter().enumerate().map(|(c,d)| format!("{}: {} {} {} {}\n",c, d.point, d.name.as_deref().unwrap_or(""), d.label.as_deref().unwrap_or(""), d.description.as_deref().unwrap_or("")))
                            .collect::<String>()
    }

    pub fn get_all_edge_names(&mut self) -> HashMap<Point,NameOrPoint> {
    
    let infotable_name = "main_table";
    let triple_ids = Vec::from_iter(self.triples.get_all_ids_from_info_table(infotable_name));
    //After that we need to trim quotes from triple ids.... at triple creation time!
    let mut descs = self.descs.get_descs_or_else_ids(triple_ids);
    //- then goto director and get descs for ids through ohter facade, 
    //- then in director pair id with desc.name or else id and return.
        descs
            .iter_mut()
            .map(|x| {
                let name = x.name.as_deref().map(|s| s.to_string()).unwrap_or_else(|| x.point.to_string());
                (x.point.to_string(), name)
            })
            .collect()
    }

    pub fn get_all_ref_edge_names_except(&mut self, _graph_name: String, ref_id: String, exception: String) -> HashMap<Point,NameOrPoint> {
        
        let infotable_name = "main_table";

        let infotable: InfoTable = self.triples.get_info_table_as_info_table(infotable_name);

        let triple_ids: Vec<String> = infotable.get_neighbor_ids_except(&ref_id, &exception);

//println!("all triple_ids********************************");    
        triple_ids
            .iter()
            .for_each(|x|{println!("id={}",x);});
    
//    let canceled = "1c1ca1cef969f76f757fdf7ff3ff9f365a864ef579f2d45866f8d0d5ef9f01df".to_string();

        let mut descs = self.descs.get_descs_or_else_ids(triple_ids);
    //- then goto director and get descs for ids through ohter facade, 
    //- then in director pair id with desc.name or else id and return.
        descs
            .iter_mut()
            .filter(|x| x.point.0 != ref_id)
            .map(|x| {
                let name = x.name.as_deref().map(|s| s.to_string()).unwrap_or_else(|| x.point.to_string());
                (x.point.to_string(), name)
            })
            .collect()
    }


    pub fn get_all_ref_edge_labels(&mut self, ref_id: String) -> HashMap<Point,LabelOrPoint> {

        let infotable_name = "main_table";
        let triple_ids = Vec::from_iter(self.triples.get_all_ids_from_info_table_select(infotable_name,&ref_id));
        let mut descs = self.descs.get_descs_or_else_ids(triple_ids);
        descs
            .iter_mut()
            .map(|x| {
                let label = x.label.as_deref().map(|s| s.to_string()).unwrap_or_else(|| x.point.to_string());
                (x.point.to_string(), label)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ig_desc::{App, Space, Descriptor, descriptor_store_fs::DescriptorStoreFS};
    use ig_tr::{triple_store_fs::TripleStoreFS, InfoTriple};

    // These tests exercise EdgeDirector against real filesystem-backed stores (TripleStoreFS +
    // DescriptorStoreFS), covering the general pairing pattern this crate is meant to support:
    // pair ids together via triples, give some of them descriptors separately, then query
    // names/labels for paired items through the director, relying on its fallback-to-point
    // behavior for ids that were never given a descriptor. Each test uses a space id unique to
    // this process run so repeated runs never see stale data from a previous run.

    fn new_director(space_id: &str) -> EdgeDirector<TripleStoreFS, DescriptorStoreFS> {
        let triples = TripleFacade::new(TripleStoreFS::new(space_id.to_string()));
        let descs = DescriptorFacade::new(DescriptorStoreFS::new(
            App::from("ig_edge_test_app".to_string()),
            Space::from(space_id.to_string()),
            "ig_edge_test_config".to_string(),
        ));
        EdgeDirector::new(triples, descs)
    }

    #[test]
    fn get_all_edge_names_falls_back_to_point_when_no_descriptor_exists() {
        let space_id = format!("ig_edge_test_names_{}", std::process::id());
        let mut director = new_director(&space_id);
        director.triples.clear_infotable("main_table".to_string());

        // "widget-1" is tagged with a category and has a descriptor with a name.
        director.triples.add_to_infotable("main_table".to_string(), InfoTriple::new("t1", "widget-1", "cat-tag"));
        director.descs.add_desc_n_index(Descriptor { point: "widget-1".into(), name: Some("Widget One".into()), ..Default::default() });

        // "widget-2" is tagged the same way but was never given a descriptor.
        director.triples.add_to_infotable("main_table".to_string(), InfoTriple::new("t2", "widget-2", "cat-tag"));

        let names = director.get_all_edge_names();

        assert_eq!(names.get("widget-1").map(String::as_str), Some("Widget One"));
        assert_eq!(names.get("widget-2").map(String::as_str), Some("widget-2"));
    }

    #[test]
    fn get_all_ref_edge_names_except_excludes_the_given_exception() {
        let space_id = format!("ig_edge_test_refs_{}", std::process::id());
        let mut director = new_director(&space_id);
        director.triples.clear_infotable("main_table".to_string());

        director.triples.add_to_infotable("main_table".to_string(), InfoTriple::new("t1", "cat-tag", "widget-1"));
        director.triples.add_to_infotable("main_table".to_string(), InfoTriple::new("t2", "cat-tag", "widget-2"));

        let refs = director.get_all_ref_edge_names_except("unused".to_string(), "cat-tag".to_string(), "widget-1".to_string());

        assert!(refs.contains_key("widget-2"));
        assert!(!refs.contains_key("widget-1"));
        assert!(!refs.contains_key("cat-tag"));
    }

    #[test]
    fn get_all_ref_edge_labels_falls_back_to_point_when_no_label_exists() {
        let space_id = format!("ig_edge_test_labels_{}", std::process::id());
        let mut director = new_director(&space_id);
        director.triples.clear_infotable("main_table".to_string());

        director.triples.add_to_infotable("main_table".to_string(), InfoTriple::new("t1", "cat-tag", "widget-1"));
        director.descs.add_desc_n_index(Descriptor { point: "widget-1".into(), label: Some("Widget Label".into()), ..Default::default() });

        let labels = director.get_all_ref_edge_labels("cat-tag".to_string());

        assert_eq!(labels.get("widget-1").map(String::as_str), Some("Widget Label"));
    }
}
