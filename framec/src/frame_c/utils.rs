extern crate exitcode;
use std::collections::HashMap;

pub struct Node {
    pub name: String,
    pub parent_name: String,
    pub children: Vec<String>,
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name,
            parent_name: String::new(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child_name: &String) {
        //       let child_name_debug = child_name.clone();
        self.children.push(child_name.clone());
    }

    pub fn remove_child(&mut self, child_name: &String) {
        //        let child_name_debug = child_name.clone();
        let index = self
            .children
            .iter()
            .position(|x| *x == *child_name)
            .unwrap();
        self.children.remove(index);
    }
}

pub struct SystemHierarchy {
    pub index: HashMap<String, Node>,
    pub system_name: String,
}

impl SystemHierarchy {
    pub fn new(system_name: String) -> SystemHierarchy {
        let system_node = Node::new(system_name.clone());
        let mut index = HashMap::new();
        index.insert(system_name.clone(), system_node);

        SystemHierarchy { index, system_name }
    }

    pub fn add_node(&mut self, node_name: String, parent_node_name_original: String) {
        let parent_node_name = parent_node_name_original.clone();

        match self.index.get_mut(&node_name) {
            Some(_index_node) => {
                //node = index_node;
            }
            None => {
                let index_node = Node::new(node_name.clone());
                self.index.insert(node_name.clone(), index_node);
            }
        }

        if parent_node_name != "" {
            match self.index.get_mut(&parent_node_name) {
                Some(_index_parent_node) => {
                    // found parent node in index
                    // parent_node = index_parent_node;
                }
                None => {
                    // have parent node name but...
                    // no parent node in index. tree parent into system as parent's parent by default
                    let parent_node = Node::new(parent_node_name.clone());
                    //parent_node.set_parent(self.system_name.clone());
                    self.index.insert(parent_node_name.clone(), parent_node);
                    self.set_parent(&parent_node_name, &self.system_name.clone());
                }
            }
        }
        self.set_parent(&node_name, &parent_node_name);
    }

    fn set_parent(&mut self, node_name: &String, new_parent_name: &String) {
        // let node_name_debug = node_name.clone();
        // let new_parent_name_debug = new_parent_name.clone();

        // all nodes must have a parent.

        let current_parent_name;

        // {
        match self.index.get_mut(node_name) {
            Some(node) => {
                current_parent_name = node.parent_name.clone();
                node.parent_name = new_parent_name.clone();
            }
            None => {
                panic!();
            }
        }

        match self.index.get_mut(&current_parent_name) {
            Some(current_parent_node) => {
                current_parent_node.remove_child(node_name);
            }
            None => {
                // no parent to remove from.
            }
        }

        let mut attach_to_system = false;
        match self.index.get_mut(new_parent_name) {
            Some(new_parent_node) => {
                new_parent_node.add_child(node_name);
            }
            None => {
                attach_to_system = true;
            }
        }
        if attach_to_system {
            match self.index.get_mut(&self.system_name) {
                Some(system_node) => {
                    system_node.add_child(node_name);
                }
                None => panic!("Error - couldn't locate system node."),
            }
        }
    }

    pub(crate) fn get_node(&self, node_name: &str) -> Option<&Node> {
        self.index.get(node_name)
    }
    pub(crate) fn get_system_node(&mut self) -> Option<&Node> {
        self.index.get(&self.system_name)
    }
}

pub(crate) mod frame_exitcode {
    pub type FrameExitCode = i32;

    /// Framepiler parse error exit
    pub const PARSE_ERR: FrameExitCode = 1;
    pub const DEFAULT_CONFIG_ERR: FrameExitCode = 2;
}

pub struct RunError {
    pub code: frame_exitcode::FrameExitCode,
    pub error: String,
}

impl RunError {
    pub fn new(code: frame_exitcode::FrameExitCode, msg: &str) -> RunError {
        RunError {
            code,
            error: String::from(msg),
        }
    }
}
