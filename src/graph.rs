use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct RNode(Rc<RefCell<Node>>);

impl RNode {
    pub fn add(&self, n: &RNode) {
        println!("{}->{}", n, self);
        self.borrow_mut().parents.push(n.clone());
        self.borrow_mut().update_depth();
        n.borrow_mut().children.push(self.clone());
    }

    fn borrow_mut(&self) -> RefMut<'_, Node> {
        self.0.borrow_mut()
    }

    pub fn borrow(&self) -> Ref<'_, Node> {
        self.0.as_ref().borrow()
    }

    pub fn get_parents(&self) -> Vec<RNode> {
        self.borrow().parents.clone()
    }

    pub fn get_children(&self) -> Vec<RNode> {
        self.borrow().children.clone()
    }

    pub fn get_name(&self) -> String {
        self.borrow().name.clone()
    }

    pub fn get_id(&self) -> u32 {
        self.borrow().id
    }

    pub fn get_depth(&self) -> u32 {
        self.borrow().depth
    }
}

impl Display for RNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.borrow().to_string())
    }
}

impl PartialEq for RNode {
    fn eq(&self, other: &Self) -> bool {
        self.borrow().name == other.borrow().name && self.borrow().id == other.borrow().id
    }
}

impl Eq for RNode {}

impl PartialOrd for RNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let ord = self.borrow().depth.cmp(&other.borrow().depth);
        if ord != Ordering::Equal {
            return ord;
        }
        let ord = self.borrow().name.cmp(&other.borrow().name);
        if ord != Ordering::Equal {
            return ord;
        }
        self.borrow().id.cmp(&other.borrow().id)
    }
}

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<String, Vec<RNode>>,
    pub sorted: Vec<RNode>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            sorted: vec![],
        }
    }

    pub fn add(&mut self, n: Node) -> RNode {
        let n = RNode(Rc::new(RefCell::new(n)));
        if let Some(vec) = self.nodes.get_mut(&n.get_name()) {
            n.borrow_mut().id = vec.len() as u32;
            vec.push(n.clone());
        } else {
            self.nodes.insert(n.get_name(), vec![n.clone()]);
        }

        self.sorted.push(n.clone());

        n
    }

    pub fn contains(&self, n: &RNode) -> bool {
        if let Some(vec) = self.nodes.get(&n.get_name()) {
            vec.contains(n)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub id: u32,
    pub depth: u32,
    pub children: Vec<RNode>,
    pub parents: Vec<RNode>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name, self.id)
    }
}

fn max(x: u32, y: u32) -> u32 {
    if x >= y {
        x
    } else {
        y
    }
}

impl Node {
    pub fn new(name: String) -> Node {
        Node {
            name,
            id: 0,
            depth: 0,
            children: vec![],
            parents: vec![],
        }
    }

    fn set_depth(&mut self, depth: u32) {
        self.depth = depth;
    }

    pub fn update_depth(&mut self) {
        for p in self.parents.iter() {
            let mut p = p.borrow_mut();
            let d = p.depth;
            p.set_depth(max(d, self.depth + 1));
        }

        for p in self.parents.iter() {
            p.borrow_mut().update_depth();
        }
    }
}
