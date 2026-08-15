use ig_tr::InfoTriple;

use ig_desc::Descriptor;
use super::info_graph::ToOneString;


#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct InfoEdge {
    pub descriptor: Descriptor, 
    pub vertex1: Descriptor,
    pub vertex2: Descriptor,
}

#[allow(dead_code)]
impl InfoEdge {

    pub fn new(id: &str, id1: &str, id2: &str) -> Self {
        InfoEdge {
                descriptor: Descriptor {
                point: id.into(),
                ..Default::default()
            },
                vertex1: Descriptor {
                point: id1.into(),
                ..Default::default()
            },
                vertex2: Descriptor {
                point: id2.into(),
                ..Default::default()
            },
        }

    }

    pub fn new_empty_edge(id: &str) -> Self {
        Self::new(id, "", "")
    }

    pub fn does_refer(&self, id: &str) -> bool {
        if &*self.vertex1.point == id || &*self.vertex2.point == id {
            return true
        }
        false
    }

    pub fn get_point(&self) -> &str {
        &self.descriptor.point
    }

    pub fn get_point_from_label(&self, label: &str) -> Vec<String>{
        let mut result: Vec<String> = Vec::new();
        if self.descriptor.label.as_deref() == Some(label) {result.push(self.descriptor.point.to_string());}
        if self.vertex1.label.as_deref() == Some(label) {result.push(self.vertex1.point.to_string());}
        if self.vertex2.label.as_deref() == Some(label) {result.push(self.vertex2.point.to_string());}
        result
    }

    #[allow(dead_code)]
    pub fn set_label(&mut self, label:&str){
        self.descriptor.set_label(label);
    }
    


    #[allow(dead_code)]
    pub fn set_label_vertex1(&mut self, label:&str){
        self.vertex1.set_label(label);
    }

    #[allow(dead_code)]
    pub fn set_label_vertex2(&mut self, label:&str){
        self.vertex2.set_label(label);
    }

    #[allow(dead_code)]
    pub fn set_description(&mut self, label:&str){
        self.descriptor.set_description(label);
    }

    #[allow(dead_code)]
    pub fn set_description_vertex1(&mut self, label:&str){
        self.vertex1.set_description(label);
    }

    #[allow(dead_code)]
    pub fn set_description_vertex2(&mut self, label:&str){
        self.vertex2.set_description(label);
    }


    #[allow(dead_code)]
    pub fn get_label(&self, id: &str) -> &str {
            if id == &*self.vertex1.point  { return self.vertex1.label.as_deref().unwrap_or(""); }
            if id == &*self.vertex2.point  { return self.vertex2.label.as_deref().unwrap_or(""); }
            if id == &*self.descriptor.point  { self.descriptor.label.as_deref().unwrap_or("") }
            else {"N/A"} //TODO: Make an error, use Result I guess...
    }

    #[allow(dead_code)]
    pub fn set_label_for_id(&mut self, new_label: &str, obj_id: &str) {
        if &*self.descriptor.point == obj_id {
            self.set_label(new_label);
        } else if &*self.vertex1.point == obj_id {
            self.set_label_vertex1(new_label);
        } else if &*self.vertex2.point == obj_id {
            self.set_label_vertex2(new_label);
        } else {
            println!("This is fine!");
        }
    }

    pub fn mock() -> InfoEdge {
        InfoEdge::mock_with_id("InfoEdge mock")
    }

    pub fn mock_with_id(id: &str) -> InfoEdge {
        let edge_id = format!("InfoEdge{}",&id);
        let edge_id = edge_id.as_str();
        let vertex1_id = format!("InfoEdge{}vertex1",&id);
        let vertex1_id = vertex1_id.as_str();
        let vertex2_id = format!("InfoEdge{}vertex2",&id);
        let vertex2_id = vertex2_id.as_str();
        InfoEdge{
            descriptor: Descriptor::mock_with_id(edge_id), 
            vertex1: Descriptor::mock_with_id(vertex1_id),
            vertex2: Descriptor::mock_with_id(vertex2_id),
        }   
    }
    
    pub fn mock_with_id_verbose(id: &str) -> InfoEdge {
        let edge_id = format!("InfoEdge - {} edge",&id);
        let edge_id = edge_id.as_str();
        let vertex1_id = format!("InfoEdge - {} vertex1",&id);
        let vertex1_id = vertex1_id.as_str();
        let vertex2_id = format!("InfoEdge - {} vertex2",&id);
        let vertex2_id = vertex2_id.as_str();
        InfoEdge{
            descriptor: Descriptor::mock_with_id(edge_id), 
            vertex1: Descriptor::mock_with_id(vertex1_id),
            vertex2: Descriptor::mock_with_id(vertex2_id),
        }   
    }
}

impl From<InfoTriple> for InfoEdge {
    fn from(triple: InfoTriple) -> Self {
        InfoEdge {
            descriptor: Descriptor{
                point: triple.id.into(),
                ..Default::default()
            },
            vertex1: Descriptor{
                point: triple.id1.into(),
                ..Default::default()
            },
            vertex2: Descriptor{
                point: triple.id2.into(),
                ..Default::default()
            }

        }
    }
}

impl ToOneString for InfoEdge {
    fn to_one_string (&self) -> String {
        let mut one_string = String::new();
        one_string.push_str(&InfoTriple::from(self.clone()).to_one_string());
        one_string.push_str(self.descriptor.name.as_deref().unwrap_or(""));
        one_string.push_str(self.descriptor.label.as_deref().unwrap_or(""));
        one_string.push_str(self.descriptor.description.as_deref().unwrap_or(""));
        one_string 
    }
}

impl From<InfoEdge> for InfoTriple {
    fn from(triple: InfoEdge) -> Self {
        InfoTriple {
            id: triple.descriptor.point.to_string(),
            id1: triple.vertex1.point.to_string(),
            id2: triple.vertex2.point.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_refer_true_for_either_vertex_false_otherwise() {
        let edge = InfoEdge::new("e1", "a", "b");
        assert!(edge.does_refer("a"));
        assert!(edge.does_refer("b"));
        assert!(!edge.does_refer("e1"));
        assert!(!edge.does_refer("c"));
    }

    #[test]
    fn get_point_from_label_matches_any_of_the_three_descriptors() {
        let mut edge = InfoEdge::new("e1", "a", "b");
        edge.set_label("edge-label");
        edge.set_label_vertex1("v1-label");

        assert_eq!(edge.get_point_from_label("edge-label"), vec!["e1".to_string()]);
        assert_eq!(edge.get_point_from_label("v1-label"), vec!["a".to_string()]);
        assert!(edge.get_point_from_label("no-such-label").is_empty());
    }

    #[test]
    fn set_label_for_id_updates_only_the_matching_part() {
        let mut edge = InfoEdge::new("e1", "a", "b");

        edge.set_label_for_id("edge-label", "e1");
        assert_eq!(edge.get_label("e1"), "edge-label");
        assert_eq!(edge.get_label("a"), "");

        edge.set_label_for_id("v1-label", "a");
        assert_eq!(edge.get_label("a"), "v1-label");

        edge.set_label_for_id("v2-label", "b");
        assert_eq!(edge.get_label("b"), "v2-label");

        // No matching id: leaves the edge unchanged and does not panic.
        edge.set_label_for_id("ignored", "no-such-id");
        assert_eq!(edge.get_label("e1"), "edge-label");
    }

    #[test]
    fn get_label_falls_back_to_n_a_for_an_unknown_id() {
        let edge = InfoEdge::new("e1", "a", "b");
        assert_eq!(edge.get_label("unknown"), "N/A");
    }

    #[test]
    fn to_one_string_includes_descriptor_fields() {
        let mut edge = InfoEdge::new("e1", "a", "b");
        edge.set_label("my-label");
        assert!(edge.to_one_string().contains("my-label"));
    }

    #[test]
    fn from_info_triple_round_trips_through_info_edge() {
        let triple = InfoTriple::new("e1", "a", "b");
        let edge: InfoEdge = triple.clone().into();
        let back: InfoTriple = edge.into();

        assert_eq!(back.id, triple.id);
        assert_eq!(back.id1, triple.id1);
        assert_eq!(back.id2, triple.id2);
    }
}
