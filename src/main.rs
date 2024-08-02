use std::collections::BTreeMap;
use std::iter::zip;

pub mod graph;
use graph::*;

pub mod frontier;
use frontier::*;

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

fn reset_iteration_vals(n: &RNode) {
    if n.get_iteration() != None {
        n.set_iteration(None);
        for c in n.borrow().parents.iter() {
            reset_iteration_vals(c);
        }
    }
}

fn thrd_it_reassignment(
    n3: &RNode,
    pairs: &mut BTreeMap<RNode, RNode>,
    frontier: &mut MinHeap<(RNode, RNode)>,
) {
    let mut is_leaf_reassignment = true;
    for c in n3.borrow().children.iter() {
        if c.get_iteration() == Some(2) {
            is_leaf_reassignment = false;
            thrd_it_reassignment(c, pairs, frontier);
        } else if c.get_iteration() == Some(0) {
            let (c1, c2) = pairs
                .remove_entry(c)
                .expect("All iteration 0 nodes should be in pairs");
            c1.set_iteration(None);
            c2.set_iteration(None);
            frontier.push((c1, c2));
        }
    }

    let n2 = pairs
        .iter()
        .find(|(_, n_prime)| n_prime == &n3)
        .expect("n2 predecessor of n3 should be in pairs")
        .0
        .clone();
    let n1 = pairs
        .iter()
        .find(|(_, n_prime)| n_prime == &&n2)
        .expect("n1 predecessor of n2 should be in pairs")
        .0
        .clone();

    println!("3rd ItR: {}, {}, {}", n1, n2, n3);

    n1.set_iteration(None);
    n2.set_iteration(None);
    n3.set_iteration(None);

    pairs.remove(&n1);
    pairs.remove(&n2);

    if is_leaf_reassignment {
        frontier.push((n2, n3.clone()));
    }
}

//Recursively Explained by Start Nodes (RESN)
fn is_resn(r: &RNode, start_pairs: &BTreeMap<RNode, RNode>) -> bool {
    r.get_iteration() == Some(1)
        && r.borrow()
            .parents
            .iter()
            .all(|p| start_pairs.contains_key(p) || is_resn(p, start_pairs))
}

fn resn_reassignment(
    n2: &RNode,
    pairs: &mut BTreeMap<RNode, RNode>,
    frontier: &mut MinHeap<(RNode, RNode)>,
) {
    let start_pairs: BTreeMap<RNode, RNode> = pairs
        .iter()
        .filter(|(n, _)| n.get_iteration() == Some(1))
        .map(|(n, n_prime)| (n.clone(), n_prime.clone()))
        .collect();

    // I don't know if order matters for reassignment or not; I think not
    let mut to_check: Vec<RNode> = n2.get_children();
    let mut to_reassign: Vec<RNode> = Vec::new();

    while let Some(n) = to_check.pop() {
        for c in n.borrow().children.iter() {
            // TODO optimize conditions
            let is_start_node = start_pairs.contains_key(&c);
            let has_snd_it_child = c
                .borrow()
                .children
                .iter()
                .any(|g| g.get_iteration() == Some(1));
            let c_is_resn = is_resn(&c, &start_pairs);
            if is_start_node && has_snd_it_child {
                continue;
            } else if has_snd_it_child && c_is_resn {
                to_check.push(c.clone());
            } else if is_start_node && c_is_resn {
                to_reassign.push(c.clone());
                let c_prime = pairs.get(&c).expect("all start nodes should be paired");
                frontier.push((c.clone(), c_prime.clone()));
                println!("RESN pushing ({},{}) to frontier", c, c_prime);
            }
        }
    }

    while let Some(n2) = to_reassign.pop() {
        let n1 = pairs
            .iter()
            .find(|(_, n_prime)| n_prime == &&n2)
            .expect("n1 predecesor of n2 should be in pairs")
            .0
            .clone();

        n1.set_iteration(None);
        n2.set_iteration(None);

        pairs.remove(&n1);

        if let Some(n3) = pairs.get(&n2).cloned() {
            n3.set_iteration(None);
            pairs.remove(&n2);
            println!("RESN R: {}, {}, {}", n1, n2, n3);
        } else {
            println!("RESN R: {}, {}", n1, n2);
        }

        for p in n2.borrow().parents.iter() {
            let is_start_node = start_pairs.contains_key(p);
            let p_is_resn = is_resn(p, &start_pairs);
            let is_third_it = p.get_iteration() == Some(2);
            if (is_start_node && p_is_resn) || (!is_start_node && !is_third_it) {
                to_reassign.push(p.clone());
            }
        }
    }
}

fn get_mlg(s: &RNode) -> Option<BTreeMap<RNode, RNode>> {
    let max_num_attempts = 4;
    let max_num_node_actions = 100;

    'attempt: for i in 1..=max_num_attempts {
        println!("Attempt #{i}");

        reset_iteration_vals(s);

        let s_prime = match find_ith_progenitor(s, i) {
            Some(s_prime) => s_prime,
            None => return None,
        };

        let mut frontier = MinHeap::new();
        frontier.push((s.clone(), s_prime.clone()));
        let mut pairs = BTreeMap::new();

        let mut num_node_actions = 0;
        'next_node: while !frontier.is_empty() || num_node_actions > max_num_node_actions {
            num_node_actions += 1;
            // get (n, n_prime) off frontier
            let (n, n_prime) = frontier.pop().unwrap();
            // check n parents matches n_prime parents
            if n.borrow().parents.len() != n_prime.borrow().parents.len()
                || zip(n.borrow().parents.iter(), n_prime.borrow().parents.iter())
                    .any(|(p, p_prime)| p.borrow().name != p_prime.borrow().name)
            {
                continue 'attempt;
            }

            assert!(
                n.get_iteration() == None
                    || n_prime.get_iteration() == None
                    || n.get_iteration().unwrap() + 1 == n_prime.get_iteration().unwrap()
            );

            match n.get_iteration() {
                None => {
                    n.set_iteration(Some(0));
                    n_prime.set_iteration(Some(1));

                    println!("{}, {}, {:?} onto pairs", n, n_prime, n.get_iteration());
                    for (p, p_prime) in zip(n.get_parents(), n_prime.get_parents()) {
                        frontier.push((p, p_prime));
                    }
                    pairs.insert(n, n_prime);
                    continue 'next_node;
                }
                Some(0) => {
                    // if n is in graph check that n, n_prime correspond
                    let x = pairs
                        .get(&n)
                        .expect("all Nodes assigned iteration 0 are in pairs");
                    if x == &n_prime {
                        println!(
                            "{}, {}, {:?} already on pairs",
                            n,
                            n_prime,
                            n.get_iteration()
                        );
                        continue 'next_node;
                    } else {
                        println!(
                            "{}, {}, {:?} failed: {}, {} already paired",
                            n,
                            n_prime,
                            n.get_iteration(),
                            n,
                            x
                        );
                        continue 'attempt;
                    }
                }
                Some(1) => {
                    // if n is in graph_prime
                    if let Some(x) = pairs.get(&n) {
                        // if (n, x) has already been seen check that x == n_prime, continue
                        if x == &n_prime {
                            println!(
                                "{}, {}, {:?} already on pairs",
                                n,
                                n_prime,
                                n.get_iteration()
                            );
                            continue 'next_node;
                        } else {
                            println!(
                                "{}, {}, {:?} failed: {}, {} already paired",
                                n,
                                n_prime,
                                n.get_iteration(),
                                n,
                                x
                            );
                            continue 'attempt;
                        }
                    } else {
                        // else mark n_prime as being in graph_prime_prime, try resn resassignment, continue
                        n_prime.set_iteration(Some(2));
                        println!("{}, {}, {:?} onto pairs", n, n_prime, n.get_iteration());
                        pairs.insert(n.clone(), n_prime);

                        resn_reassignment(&n, &mut pairs, &mut frontier);

                        continue 'next_node;
                    }
                }
                Some(2) => {
                    // if n is in graph_prime_prime perform 3rd iteration reassignment
                    thrd_it_reassignment(&n, &mut pairs, &mut frontier);
                    continue 'next_node;
                }
                _ => panic!("There shouldn't be any further assignments"),
            }
        }

        return Some(pairs);
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

    fn test_graph_path() -> Graph {
        let mut graph = Graph::new();
        for i in 0..10 {
            let a = graph.add(Node::new("A".to_owned()));
            let b = graph.add(Node::new("B".to_owned()));

            if i > 0 {
                let ns = &mut graph.nodes;
                add_to_previous(&a, "A", 1, ns);
                add_to_previous(&b, "B", 1, ns);
                add_to_previous(&b, "A", 1, ns);
                if i > 2 {
                    add_to_previous(&a, "B", 2, ns);
                }
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
        let graph = test_graph_path();
        if let Some(pairs) = get_mlg(&graph.sorted[0]) {
            println!("Pairs");
            for (n, n_prime) in pairs {
                println!("({n},{n_prime}), {:?}", n.get_iteration());
            }
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
