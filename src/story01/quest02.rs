use crate::util::parse::*;
use std::collections::VecDeque;
use std::mem::swap;

struct Node<'a> {
    rank: u64,
    name: &'a str,
    left: Option<usize>,
    right: Option<usize>,
}

impl Node<'_> {
    fn new(rank: u64, name: &str) -> Node<'_> {
        Node { rank, name, left: None, right: None }
    }
}

type Tree<'a> = Vec<Node<'a>>;

pub fn part1(notes: &str) -> String {
    solve(notes, |_, _| ())
}

pub fn part2(notes: &str) -> String {
    solve(notes, |node1, node2| {
        swap(&mut node1.rank, &mut node2.rank);
        swap(&mut node1.name, &mut node2.name);
    })
}

pub fn part3(notes: &str) -> String {
    solve(notes, |node1, node2| swap(node1, node2))
}

fn solve(notes: &str, swap: for<'a> fn(node: &mut Node<'a>, node2: &mut Node<'a>)) -> String {
    let mut tree = Vec::new();

    for line in notes.lines() {
        if line.starts_with("ADD") {
            let tokens: Vec<_> = line.split(['[', ',', ']']).collect();
            let size = tree.len();

            tree.push(Node::new(tokens[1].unsigned(), tokens[2]));
            tree.push(Node::new(tokens[4].unsigned(), tokens[5]));

            if size > 0 {
                insert(&mut tree, size, 0);
                insert(&mut tree, size + 1, 1);
            }
        }
        if line.starts_with("SWAP") {
            let index = line.unsigned::<usize>();
            let [first, second] = tree.get_disjoint_mut([2 * index - 2, 2 * index - 1]).unwrap();
            swap(first, second);
        }
    }

    let mut result = String::new();
    result.push_str(&bfs(&tree, 0));
    result.push_str(&bfs(&tree, 1));
    result
}

fn insert(nodes: &mut Tree<'_>, from: usize, to: usize) {
    if nodes[from].rank < nodes[to].rank {
        if let Some(next) = nodes[to].left {
            insert(nodes, from, next);
        } else {
            nodes[to].left = Some(from);
        }
    } else if let Some(next) = nodes[to].right {
        insert(nodes, from, next);
    } else {
        nodes[to].right = Some(from);
    }
}

fn bfs(nodes: &Tree<'_>, start: usize) -> String {
    let mut todo = VecDeque::from([(start, 0)]);
    let mut messages = Vec::new();

    while let Some((index, depth)) = todo.pop_front() {
        if messages.len() == depth {
            messages.push(String::new());
        }
        messages[depth].push_str(nodes[index].name);

        if let Some(next) = nodes[index].left {
            todo.push_back((next, depth + 1));
        }
        if let Some(next) = nodes[index].right {
            todo.push_back((next, depth + 1));
        }
    }

    let max = messages.iter().map(String::len).max().unwrap();
    messages.extract_if(.., |m| m.len() == max).next().unwrap()
}
