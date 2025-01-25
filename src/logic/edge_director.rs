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

                descs.iter().enumerate().map(|(c,d)| format!("{}: {} {} {} {}\n",c, d.point, d.name, d.label, d.description))
                            .reduce(|mut result, var| { result.push_str(&var); result}).unwrap()
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
                let mut name = x.name.clone(); 
                if x.name.is_empty() {
                    name = x.point.clone()
                } 
                (x.point.clone(), name.clone())
            })
            .collect() 
    }

    pub fn get_all_ref_edge_names_except(&mut self, graph_name: String, ref_id: String, exception: String) -> HashMap<Point,NameOrPoint> {
        
        let infotable_name = "main_table";

        let infotable: InfoTable = self.triples.get_info_table_as_info_table(infotable_name);

        let triple_ids: Vec<String> = infotable.get_neighbor_ids_except(ref_id.clone(), exception.clone());

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
            .filter(|x| x.point != ref_id) 
            .map(|x| { 
                let mut name = x.name.clone(); 
                if x.name.is_empty() {
                    name = x.point.clone()
                } 
                (x.point.clone(), name.clone())
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
                let mut label = x.label.clone(); 
                if x.label.is_empty() {
                    label = x.point.clone()
                } 
                (x.point.clone(), label.clone())
            })
            .collect() 
    }
}
