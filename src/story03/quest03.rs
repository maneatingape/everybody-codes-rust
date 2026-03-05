use crate::util::iter::*;
use crate::util::parse::*;

type Pair<'a> = (&'a str, &'a str);

struct Node<'a> {
    id: u64,
    plug: Pair<'a>,
    sockets: [Pair<'a>; 2],
    connected: [Option<usize>; 2],
}

pub fn part1(notes: &str) -> u64 {
    build_tree(notes, false, false)
}

pub fn part2(notes: &str) -> u64 {
    build_tree(notes, true, false)
}

pub fn part3(notes: &str) -> u64 {
    build_tree(notes, true, true)
}

fn build_tree(notes: &str, weak_links: bool, replace_links: bool) -> u64 {
    let mut nodes: Vec<_> = notes
        .split(['=', ',', '\n'])
        .chunk::<10>()
        .map(|[_, id, _, plug, _, left, _, right, _, _]| {
            let id = id.unsigned();
            let plug = plug.split_once(' ').unwrap();
            let left = left.split_once(' ').unwrap();
            let right = right.split_once(' ').unwrap();
            Node { id, plug, sockets: [left, right], connected: [None; 2] }
        })
        .collect();

    for mut to in 1..nodes.len() {
        while !insert(&mut nodes, weak_links, replace_links, 0, &mut to) {}
    }

    checksum(&nodes)
}

fn insert(
    nodes: &mut [Node<'_>],
    weak_links: bool,
    replace_links: bool,
    from: usize,
    to: &mut usize,
) -> bool {
    for side in 0..2 {
        let plug = nodes[*to].plug;
        let socket = nodes[from].sockets[side];

        if let Some(next) = nodes[from].connected[side] {
            if replace_links && strong(plug, socket) && weak(nodes[next].plug, socket) {
                nodes[from].connected[side] = Some(*to);
                *to = next;
            } else if insert(nodes, weak_links, replace_links, next, to) {
                return true;
            }
        } else if strong(plug, socket) || (weak_links && weak(plug, socket)) {
            nodes[from].connected[side] = Some(*to);
            return true;
        }
    }

    false
}

fn checksum(nodes: &[Node<'_>]) -> u64 {
    let mut order = Vec::new();
    in_order(nodes, 0, &mut order);
    order.iter().zip(1..).map(|(id, position)| id * position).sum()
}

fn in_order(nodes: &[Node<'_>], index: usize, order: &mut Vec<u64>) {
    if let Some(left) = nodes[index].connected[0] {
        in_order(nodes, left, order);
    }

    order.push(nodes[index].id);

    if let Some(right) = nodes[index].connected[1] {
        in_order(nodes, right, order);
    }
}

fn strong(first: Pair<'_>, second: Pair<'_>) -> bool {
    first == second
}

fn weak(first: Pair<'_>, second: Pair<'_>) -> bool {
    first != second && (first.0 == second.0 || first.1 == second.1)
}
