use ig_tr::InfoTriple;

use ig_desc::Descriptor;
use super::info_graph::ToOneString;


#[derive(Debug)]
#[derive(PartialEq)]
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
                point: id.to_string(),
                ..Default::default()
            },
                vertex1: Descriptor {
                point: id1.to_string(),
                ..Default::default()
            },
                vertex2: Descriptor {
                point: id2.to_string(),
                ..Default::default()
            },
        }

    }

    pub fn new_empty_edge(id: &str) -> Self {
        Self::new(id, "", "")
    }

    pub fn does_refer(&self, id: &str) -> bool {
        if self.vertex1.point == id || self.vertex2.point == id {
            return true
        }
        false
    }

    pub fn get_point(&self) -> &str {
        &self.descriptor.point
    }

    pub fn get_point_from_label(&self, label: &str) -> Vec<String>{
        let mut result: Vec<String> = Vec::new();
        if self.descriptor.label == label {result.push(self.descriptor.point.to_string());}
        if self.vertex1.label == label {result.push(self.vertex1.point.to_string());}
        if self.vertex2.label == label {result.push(self.vertex2.point.to_string());}
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
            if id == self.vertex1.point  { return self.vertex1.label.as_str(); }
            if id == self.vertex2.point  { return self.vertex2.label.as_str(); }
            if id == self.descriptor.point  { return self.descriptor.label.as_str(); }
            else {"N/A"} //TODO: Make an error, use Result I guess...
    }

    #[allow(dead_code)]
    pub fn set_label_for_id(&mut self, new_label: &str, obj_id: &str) {

        let mut edge_copy:InfoEdge = self.clone();
        match &mut edge_copy {
            InfoEdge{descriptor,..} if descriptor.point == *obj_id.to_string()
                => {self.set_label(new_label);},
            
            InfoEdge{vertex1,..} if vertex1.point == *obj_id.to_string()
                => {self.set_label_vertex1(new_label);},  
            
            InfoEdge{vertex2,..} if vertex2.point == *obj_id.to_string() 
                => {self.set_label_vertex2(new_label);},  
            
            _ => {println!("This is fine!")},
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
                point: triple.id,
                label: String::new(),
                ..Default::default()
            },
            vertex1: Descriptor{
                point: triple.id1,
                label: String::new(),
                ..Default::default()
            },
            vertex2: Descriptor{
                point: triple.id2,
                label: String::new(),
                ..Default::default()
            }

        }
    }
}

impl Clone for InfoEdge{ 
    fn clone(&self) -> Self {
        InfoEdge{
            descriptor: self.descriptor.clone(),
            vertex1: self.vertex1.clone(),
            vertex2: self.vertex2.clone(),
        } 
    } 
}

#[allow(dead_code)]
pub(crate) fn mock() -> InfoEdge {
    InfoEdge {
        descriptor: Descriptor::mock_with_id("edge vertex"),
        vertex1: Descriptor::mock_with_id("vertex1"),
        vertex2: Descriptor::mock_with_id("vertex2"),
    } 
}

impl ToOneString for InfoEdge {
    fn to_one_string (&self) -> String {
        let mut one_string = String::new();
        one_string.push_str(&InfoTriple::from(self.clone()).to_one_string());
        one_string.push_str(&self.descriptor.name);
        one_string.push_str(&self.descriptor.label);
        one_string.push_str(&self.descriptor.description);
        one_string 
    }
}

impl From<InfoEdge> for InfoTriple {
    fn from(triple: InfoEdge) -> Self {
        InfoTriple {
            id: triple.descriptor.point,
            id1: triple.vertex1.point,
            id2: triple.vertex2.point,
        }
    }
}
