# Matching Loop Graph Prototype

This project is a prototype for the matching loop graph generation algorithm to be used in 
[the axiom profiler](https://github.com/viperproject/axiom-profiler-2).

Below is pseudocode and some notes describing and explaining my approach.

```
// takes an initial node s and an integer i
// performs a backwards search visiting nodes of the deepest depth first
// returns the ith closest ancestor of the same quantifier
fn find_ith_progenitor(s: Node, i: Integer) -> Node or None:
    if i < 1:
        return None

    let frontier be a set of Nodes ordered by depth
    let nodes be a set of Nodes ordered by depth

    for each parent p of s, the initial node:
        push p onto frontier

    while frontier is not empty:
        pop the deepest depth node off of frontier, call it n
        record that we have visited n by pushing it onto nodes

        if n and s are the same quantifier && i == 1:
            return n
        else if n and s are the same quantifier:
            i -= 1

        for each parent p of n:
            if p is not in nodes (has not been visited):
                push p onto frontier

    return None

// takes a third iteration node and recursively on its third iteration children
// drops ALL THREE corresponding nodes from pairs and sets their iteration to None so that they are "undiscovered"
// leaf recursive call (n2, n3) are added the frontier 
// as a result all dropped (n2, n3) pairs will be discovered now as first and second iteration nodes, respectively
// this effectively reduces each such corresponding node's iteration by one
//  -> (in a way that preserves important information about the graph and reuses code)
fn thrd_it_reassignment(n3, pairs, frontier):
    for each child c of n3:
        if c is a 3rd iteration node:
            thrd_it_reassignment(c, pairs, frontier)
        else if c is a 1st iteration node:
            (c1, c2) = pairs.remove_entry(c) 
            set c1.iteration to None
            set c2.iteration to None
            remove (c1, c2) from pairs
            add (c1, c2) to frontier
    let n1 be the 1st iteration node
    let n2 be the 2nd iteration node
    let n3 be the 3rd iteration node
    set n1.iteration to None
    set n2.iteration to None
    set n3.iteration to None
    remove the pair (n1, n2) from pairs:
    remove the pair (n2, n3) from pairs:
    if this is a leaf recursive call:
        push pair (n2, n3) to frontier:

fn is_resn(n):
    n is resn iff:
        n is in 2nd iteration
        and each parent of n is either a start node or is resn

// this function proceeds in two parts:
//  first it identifies resn start nodes,
//  then it performs reassignment on the identified nodes and any of that node's
//  parents that become resn start nodes as a result of reassignment are added
//  to the worklist
//  reassignment is done similarly to third_it_reassignment, by dropping corresponding 
//  nodes from each iteration and adding leaf dropped nodes' pairs to the frontier
fn resn_reassignment(n2, pairs, frontier):
    let to_check be the nodes to be checked for being resn start nodes
    let to_reassign be the resn start nodes to be reassigned

    while let x = to_check.pop()
        for each child c of n2:
            if c is a start node (has a child from the first iteration)
                 and c has children from the second iteration:
                continue
            else if c has a child from the second iteration and is resn:
                try_resn_reassignment(c, pairs, frontier)
                continue
            else if c is a start node and is resn:
                let c_prime be pairs.get(c)
                resn_reassignment(c, pairs)
                frontier.push((c, c_prime))
                continue

    while let n2 = to_reassign.pop():
        let n1 be the 1st iteration node
        let n2 be the 2nd iteration node
        let n3 be the 3rd iteration node if it exists
        set n1.iteration to None
        set n2.iteration to None
        set n3.iteration to None if it exists
        remove the pair (n1, n2) from pairs (it must exist)
        remove the pair (n2, n3) from pairs if it exists
    
        for each parent p of n2:
            if p is a start node and is resn
               and p is not a start node and is not from the third iteration:
            resn_reassignment(p, pairs)
            
// takes initial node that is part of a matching loop
// performs two-finger matching loop algorithm to identify two complete iterations of a matching loop
// with "3rd Iteration Reassignment"
// with "Recursively Explained Start Node Reassignment"
// returns paired nodes in matching loop graph or None
fn get_mlg(s):
    'next_attempt: for i from 1 to num_attempts:
        recursively reset the iteration values of s and its ancestors

        let s_prime = find_ith_progenitor(s, i)
        if find_ith_progenitor fails, return None
        
        let frontier be a min heap of Node pairings (elements are not guarenteed unique)
        push (s, s_prime) to frontier
        let pairs be an ordered map of Node pairings (ordered and unique by "key" node)
        
        let number_of_node_actions = 0
        'next_node: while frontier is not empty or number_of_node_actions > max_number_of_node_actions:
            number_of_node_actions += 1

            pop (n, n_prime) off frontier

            check parents of n match parents of parents of n_prime modulo quantifier name
            if not continue to 'next_attempt
            
            match n.iteration on:
                None:
                    this means that n has not been seen before
                    set n to first iteration
                    set n_prime to second iteration
                    let parent_pairings = zip(n.parents, n_prime.parents)
                    push each parent_paring to frontier
                    add (n, n_prime) to pairs
                first:
                    this means that n has already been assigned a pairing 
                    check that n_prime == pairs.get(n)
                    if not continue to 'next_attempt
                second:
                    this means that n is a start node
                    if n has already been assigned a pairing:
                        check that n_prime == pairs.get(n)
                        if not continue to 'next_attempt
                    else:       
                        set n_prime to third iteration
                        add (n, n_prime) to pairs
                        call resn_reassignment(n, pairs, frontier)
                third:
                    this means that a first iteration node has a third iteration node as a parent, which is not allowed
                    call thrd_it_reassignment(n, pairs, frontier)
                otherwise:
                    panic! no other values are ever used

        return pairs
    return None
        
```
