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
        for c in n.borrow().children.iter() {
            reset_iteration_vals(c);
        }
    }
}

fn thrd_it_reassignment(
    n3: &RNode,
    pairs: &mut BTreeMap<RNode, RNode>,
    frontier: &mut MinHeap<(RNode, RNode)>,
) {
    let mut has_3rd_it_children = false;
    for c in n3.borrow().children.iter() {
        if c.get_iteration() == Some(2) {
            has_3rd_it_children = true;
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

    let (n2, _) = pairs
        .iter()
        .find(|(_, n_prime)| n_prime == &n3)
        .expect("n2 predecessor of n3 should be in pairs");
    let (n1, _) = pairs
        .iter()
        .find(|(_, n_prime)| n_prime == &n2)
        .expect("n1 predecessor of n2 should be in pairs");

    let n1 = n1.clone();
    let n2 = n2.clone();
    let n3 = n3.clone();

    println!("3rd ItR: {}, {}, {}", n1, n2, n3);
    n1.set_iteration(None);
    n2.set_iteration(None);
    n3.set_iteration(None);

    pairs.remove(&n1);
    pairs.remove(&n2);

    if !has_3rd_it_children {
        frontier.push((n2, n3));
    }
}

fn get_mlg(s: &RNode) -> Option<BTreeMap<RNode, RNode>> {
    let num_attempts = 4;

    'attempt: for i in 1..=num_attempts {
        println!("Attempt #{i}");

        reset_iteration_vals(s);

        let s_prime = match find_ith_progenitor(s, i) {
            Some(s_prime) => s_prime,
            None => return None,
        };

        let mut frontier = MinHeap::new();
        frontier.push((s.clone(), s_prime.clone()));
        let mut pairs = BTreeMap::new();

        'next_node: while !frontier.is_empty() {
            // get (n, n_prime) off stack
            let (n, n_prime) = frontier.pop().unwrap();
            // check n parents matches n_prime parents
            if zip(n.get_parents(), n_prime.get_parents())
                .any(|(n, n_prime)| n.borrow().name != n_prime.borrow().name)
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
                    for p in zip(n.get_parents(), n_prime.get_parents()) {
                        frontier.push(p);
                    }
                    pairs.insert(n, n_prime);
                    continue 'next_node;
                }
                Some(0) => {
                    // if n is in graph check that n, n_prime correspond
                    if pairs
                        .get(&n)
                        .expect("all Nodes assigned iteration 0 are in pairs")
                        == &n_prime
                    {
                        continue 'next_node;
                    } else {
                        continue 'attempt;
                    }
                }
                Some(1) => {
                    // if n is in graph_prime mark n_prime as being in graph_prime_prime continue
                    n_prime.set_iteration(Some(2));
                    println!("{}, {}, {:?} onto pairs", n, n_prime, n.get_iteration());
                    pairs.insert(n, n_prime);
                    continue 'next_node;
                }
                Some(2) => {
                    // if n is in graph_prime_prime perform 3rd iteration reassignment
                    thrd_it_reassignment(&n, &mut pairs, &mut frontier);
                    continue 'next_node;
                }
                Some(3) => continue 'attempt,
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
        if let Some(pairs) = get_mlg(&graph.sorted[1]) {
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
