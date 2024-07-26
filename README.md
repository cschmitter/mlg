```
// takes an initial node s and an integer i
// performs a backwards search visiting nodes of the deepest depth first
// returns the ith closest ancestor of the same quantifier
Function find_ith_progenitor(s: Node, i: Integer) -> Node or None {
    if i < 1 {
        return None
    }

    let frontier be a set of Nodes ordered by depth
    let nodes be a set of Nodes ordered by depth

    for each parent p of s, the initial node {
        push p onto frontier
    }

    while frontier is not empty {
        pop the deepest depth node off of frontier, call it n
        record that we have visited n by pushing it onto nodes

        if n and s are the same quantifier && i == 1 {
            return n
        } else if n and s are the same quantifier {
            i -= 1
        }

        for each parent p of n {
            if p is not in nodes (has not been visited) {
                push p onto frontier
            }
        }
    }

    None
}

\\ takes pair of 2nd and 3rd iteration node, set of all pairs in mlg, frontier of pairs to search
\\ for each child of 3rd itartion node
\\     if child is a 3rd iteration node, c3
\\         get corresponding 2nd iteration node, c2
\\         recursively apply thrd_it_reassignment
\\ let n3 be the 3rd iteration node
\\ let n2 be the 2nd iteration node
\\ remove the pair (__, n2) from pairs:
\\     this corresponds to dropping n1 from the graph
\\ decrement the iteration marker in the pair (n2, n3):
\\     this corresponds to reassigning n2 to be the new n1 and n3 to be the new n2
\\ push pair (n2, n3) to frontier:
\\     this will result in correctly finding the new corresponding 3rd iteration start nodes and adding the pairs (n3, __) to pairs set
fn thrd_it_reassignment() 

\\ while for each recursively explained start nodes (in order of depth, deepest first)
\\     tries RE start node reassignment
\\     if total number of RE start nodes decreases
\\         keep reassignment and recurse
\\     else
\\         ignore this start node and continue
fn resn_reassignment(frontier:, pairs: )

\\ takes initial node s
\\ uses find_ith_progenitor to guess start node corresponding to s from previous iteration
\\     call this s_prime
\\     (i is incremented for each attempt and is capped at some max number of attempts)
\\ performs two-finger matching loop algorithm to identify two complete iterations of a matching loop
\\ performs "3rd Iteration Reassignment"
\\ performs "Recursively Explained Start Node Reassignment"
\\ returns nodes in matching loop graph paired nodes

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
```
