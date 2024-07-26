use std::cmp::{Ordering, Reverse};
use std::{
    collections::{BTreeSet, BinaryHeap},
    iter::zip,
};

pub mod graph;
use graph::*;

struct MinHeap<T>(BinaryHeap<Reverse<T>>)
where
    T: Ord + Clone;

impl<T> MinHeap<T>
where
    T: Ord + Clone,
{
    fn new() -> MinHeap<T> {
        MinHeap(BinaryHeap::new())
    }

    fn push(&mut self, t: T) {
        self.0.push(Reverse(t));
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|t| t.0)
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn contains(&self, t: &T) -> bool {
        self.0.iter().any(|r| &r.0 == t)
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|r| &r.0)
    }
}

struct MinSet<T>(BTreeSet<T>)
where
    T: Ord + Clone;

impl<T> MinSet<T>
where
    T: Ord + Clone,
{
    fn new() -> MinSet<T> {
        MinSet(BTreeSet::new())
    }

    fn push(&mut self, t: T) {
        self.0.insert(t);
    }

    fn pop(&mut self) -> Option<T> {
        self.0.pop_first()
    }

    fn size(&self) -> usize {
        self.0.len()
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn contains(&self, t: &T) -> bool {
        self.0.contains(t)
    }

    fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

fn find_ith_progenitor(s: &RNode, i: usize) -> Option<RNode> {
    if i < 1 {
        return None;
    }

    let mut i = i;
    let mut frontier: MinSet<RNode> = MinSet::new();
    let mut nodes: MinSet<RNode> = MinSet::new();

    let s_ref = s.borrow();
    println!("{}", s);

    for n in &s_ref.parents {
        frontier.push(n.clone());
    }

    while !frontier.is_empty() {
        let n = frontier.pop().expect("should never run on empty frontier");
        nodes.push(n.clone());
        let n_ref = n.borrow();

        println!("{}", n);

        if n_ref.name == s_ref.name && i == 1 {
            return Some(n.clone());
        } else if n_ref.name == s_ref.name {
            // i > 1
            i -= 1;
        }

        for n in &n_ref.parents {
            if !nodes.contains(n) {
                frontier.push(n.clone());
            }
        }
    }

    None
}

type Iteration = u32;

#[derive(Clone, PartialEq, Eq)]
struct Pairing {
    n: RNode,
    n_prime: RNode,
    iteration: Iteration,
    is_start_restart: bool,
}

impl Pairing {
    fn new(n: RNode, n_prime: RNode, iteration: Iteration) -> Pairing {
        Pairing {
            n,
            n_prime,
            iteration,
            is_start_restart: false,
        }
    }
}

impl PartialOrd for Pairing {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.n.partial_cmp(&other.n)
    }
}

impl Ord for Pairing {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.n.cmp(&other.n)
    }
}

fn get_mlg(s: &RNode) -> Option<(Vec<(RNode, RNode)>, Vec<RNode>)> {
    let num_attempts = 4;

    'attempt: for i in 1..=num_attempts {
        println!("Attempt #{i}");
        let s_prime = match find_ith_progenitor(s, i) {
            Some(s_prime) => s_prime,
            None => return None,
        };

        let mut frontier = MinHeap::new();
        frontier.push(Pairing::new(s.clone(), s_prime.clone(), 0));
        let mut pairs: Vec<Pairing> = vec![];

        'next_node: while !frontier.is_empty() {
            // get (n, n_prime) off stack
            let mut p = frontier.pop().unwrap();
            // check n parents matches n_prime parents
            let paired_parents: Vec<Pairing> = zip(p.n.get_parents(), p.n_prime.get_parents())
                .map(|(n, m)| Pairing::new(n, m, 0))
                .collect();
            if paired_parents
                .iter()
                .any(|p| p.n.borrow().name != p.n_prime.borrow().name)
            {
                continue 'attempt;
            }

            for q in pairs.iter_mut() {
                if q.iteration == 0 && p.n == q.n {
                    // if n is in graph check that n, n_prime correspond
                    if p.n_prime == q.n_prime {
                        continue 'next_node;
                    } else {
                        continue 'attempt;
                    }
                } else if q.iteration == 0 && p.n == q.n_prime {
                    // if n is in graph_prime mark n_prime as being in graph_prime_prime continue
                    p.iteration = 1;
                    q.is_start_restart = true;
                    println!("{}, {}, {:?} onto pairs", p.n, p.n_prime, p.iteration);
                    pairs.push(p);
                    continue 'next_node;
                } else if q.iteration == 1 && p.n == q.n_prime {
                    // else if n is in graph_prime_prime STOP
                    continue 'attempt;
                }
            }

            println!("{}, {}, {:?} onto pairs", p.n, p.n_prime, p.iteration);
            pairs.push(p);
            for p in paired_parents {
                frontier.push(p);
            }
            continue 'next_node;
        }

        let mut start_nodes = vec![];
        let mut nodes = vec![];

        for p in pairs.into_iter() {
            if p.iteration == 0 && !p.is_start_restart {
                nodes.push(p.n);
            } else if p.iteration == 0 && p.is_start_restart {
                nodes.push(p.n.clone());
                start_nodes.push((p.n, p.n_prime));
            }
        }

        return Some((start_nodes, nodes));
    }

    None
}

fn main() {}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use super::*;

    fn add_to_previous(n: &RNode, s: &str, i: u32, ns: &mut HashMap<String, Vec<RNode>>) {
        let ms = ns.get_mut(&s.to_owned()).expect("Bad Node Name");
        let l = ms.len() - 1;
        ms[l - i as usize].add(n);
    }

    fn test_graph_abcd() -> Graph {
        let mut graph = Graph::new();
        for i in 0..10 {
            let a = graph.add(Node::new("A".to_owned()));
            let b = graph.add(Node::new("B".to_owned()));
            let c = graph.add(Node::new("C".to_owned()));
            let d = graph.add(Node::new("D".to_owned()));

            a.add(&d);
            b.add(&a);
            b.add(&c);

            if i > 0 {
                let ns = &mut graph.nodes;
                add_to_previous(&a, "C", 1, ns);
                add_to_previous(&b, "D", 1, ns);
                add_to_previous(&b, "B", 1, ns);
            }
        }
        graph.sorted.sort();
        graph
    }

    fn test_graph_yorg() -> Graph {
        let mut graph = Graph::new();
        for i in 0..10 {
            let y = graph.add(Node::new("Y".to_owned()));
            let o = graph.add(Node::new("O".to_owned()));
            let r = graph.add(Node::new("R".to_owned()));
            let g = graph.add(Node::new("G".to_owned()));

            o.add(&r);
            o.add(&g);

            if i > 0 {
                let ns = &mut graph.nodes;
                add_to_previous(&o, "G", 1, ns);
                add_to_previous(&o, "R", 1, ns);
                add_to_previous(&y, "R", 1, ns);
                add_to_previous(&y, "Y", 1, ns);
            }
        }
        graph.sorted.sort();
        graph
    }

    fn test_graph_layered() -> Graph {
        let mut graph = Graph::new();
        for i in 0..10 {
            let r = graph.add(Node::new("R".to_owned()));
            let b = graph.add(Node::new("B".to_owned()));
            let g = graph.add(Node::new("G".to_owned()));
            let c0 = graph.add(Node::new("C".to_owned()));
            let c1 = graph.add(Node::new("C".to_owned()));
            let c2 = graph.add(Node::new("C".to_owned()));
            let y0 = graph.add(Node::new("Y".to_owned()));
            let y1 = graph.add(Node::new("Y".to_owned()));

            r.add(&b);
            r.add(&g);
            b.add(&c0);
            c0.add(&c1);
            c1.add(&c2);
            g.add(&y0);
            y0.add(&y1);

            if i > 0 {
                let ns = &mut graph.nodes;
                add_to_previous(&r, "Y", 2, ns);
                add_to_previous(&r, "C", 3, ns);
            }
        }
        graph.sorted.sort();
        graph
    }

    #[test]
    pub fn test_find_ith_progenitor() {
        let graph = test_graph_abcd();

        println!();

        for m in graph.sorted.iter().take(4) {
            let n = find_ith_progenitor(&m, 2);

            assert!(n.clone().is_some_and(|n| {
                let x: u32;
                if n.borrow().name != "C" {
                    x = 2;
                } else {
                    x = 3;
                }

                let m = m.borrow();
                let n = n.borrow();
                n.to_string() == format!("{}{}", m.name, m.id + x)
            }));

            println!(
                "--------\n{} ~> {}\n",
                m.borrow(),
                match n {
                    Some(n) => n.borrow().to_string(),
                    None => "None".to_string(),
                }
            );
        }
    }

    #[test]
    pub fn test_get_mlg() {
        let graph = test_graph_abcd();
        if let Some((start_nodes, nodes)) = get_mlg(&graph.sorted[2]) {
            println!("Start Nodes");
            for (n, n_prime) in start_nodes {
                print!("({n},{n_prime}), ");
            }
            println!("\nNodes");
            for n in nodes {
                print!("{n}, ");
            }
            println!();
        } else {
            println!("Got None!");
        }
        assert!(false);
    }

    #[test]
    pub fn test_sorted() {
        let graph = test_graph_layered();
        for n in graph.sorted.iter() {
            println!("{}: {}", n, n.borrow().depth);
        }
        // assert!(false);
    }

    #[test]
    pub fn test_priorty_queue() {
        let mut graph = Graph::new();

        for i in 0..10 {
            let a = graph.add(Node::new("A".to_string()));
            let b = graph.add(Node::new("B".to_string()));
            let c = graph.add(Node::new("C".to_string()));

            a.add(&b);
            b.add(&c);

            if i > 0 {
                add_to_previous(&a, "A", 1, &mut graph.nodes);
                add_to_previous(&a, "C", 1, &mut graph.nodes);
            }
        }
        graph.sorted.sort();

        get_mlg(&graph.sorted[0]);

        // assert!(false);
    }
}
