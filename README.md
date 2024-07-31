#Matching Loop Graph Prototype

This project is a prototype for the matching loop graph generation algorithm to be used in 
[the axiom profiler](https://github.com/viperproject/axiom-profiler-2).


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

\\ takes 3rd iteration node, set of all pairs in mlg, frontier of pairs to search
\\ for each child of 3rd itartion node
\\     if child is a 3rd iteration node, c3
\\         recursively apply thrd_it_reassignment
\\ let n1 be the 1st iteration node
\\ let n2 be the 2nd iteration node
\\ let n3 be the 3rd iteration node
\\ set n1.iteration to None
\\ set n2.iteration to None
\\ set n3.iteration to None
\\ remove the pair (n1, n2) from pairs:
\\     this corresponds to dropping n1 from the graph
\\ remove the pair (n2, n3) from pairs:
\\     this corresponds to dropping n1 from the graph
\\ if this is a leaf recursive call push pair (n2, n3) to frontier:
\\     this will result in correctly finding the new corresponding 3rd iteration start nodes and adding the pairs (n3, __) to pairs
fn thrd_it_reassignment() 

\\ while for each recursively explained start nodes (in order of depth, deepest first)
\\     tries RE start node reassignment
\\     if total number of RE start nodes decreases
\\         keep reassignment and recurse
\\     else
\\         ignore this start node and continue
fn resn_reassignment()

\\ takes initial node s
\\ uses find_ith_progenitor to guess start node corresponding to s from previous iteration
\\     call this s_prime
\\     (i is incremented for each attempt and is capped at some max number of attempts)
\\ performs two-finger matching loop algorithm to identify two complete iterations of a matching loop
\\ with "3rd Iteration Reassignment"
\\ with "Recursively Explained Start Node Reassignment"
\\ returns nodes in matching loop graph paired nodes
```
