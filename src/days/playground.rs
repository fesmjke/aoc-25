use std::str::FromStr;

use crate::{aoc_test, day::Day};

#[derive(Debug)]
struct Node {
    parent: usize,
    size: usize,
}

struct DisjointSet {
    nodes: Vec<Node>,
}

impl DisjointSet {
    pub fn new(size: usize) -> Self {
        Self {
            nodes: (0..size)
                .map(|id| Node {
                    parent: id,
                    size: 1,
                })
                .collect(),
        }
    }

    pub fn find(&mut self, id: usize) -> usize {
        if self.nodes[id].parent == id {
            return id;
        } else {
            self.nodes[id].parent = self.find(self.nodes[id].parent);
            return self.nodes[id].parent;
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);

        if x == y {
            return;
        }

        let (x, y) = if self.nodes[x].size < self.nodes[y].size {
            (y, x)
        } else {
            (x, y)
        };

        self.nodes[y].parent = x;
        self.nodes[x].size += self.nodes[y].size;
    }
}

struct Playground;

#[derive(Debug)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3d {
    pub fn distance(&self, other: &Self) -> i64 {
        ((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)).isqrt()
    }
}

impl FromStr for Point3d {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums = s
            .split(',')
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        Ok(Self {
            x: nums[0],
            y: nums[1],
            z: nums[2],
        })
    }
}

impl Day for Playground {
    type Input = Vec<Point3d>;

    fn parse(lines: impl Iterator<Item = std::io::Result<String>>) -> Self::Input {
        lines
            .filter_map(|line| line.ok())
            .map(|line| Point3d::from_str(&line).unwrap())
            .collect()
    }

    fn part_a(input: &Self::Input) -> impl std::fmt::Display {
        let length = input.len();
        let mut distances = vec![];

        for i in 0..length - 1 {
            for j in i + 1..length {
                let d = input[i].distance(&input[j]);

                distances.push((d, i, j));
            }
        }
        distances.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        let mut disjointset = DisjointSet::new(length);

        for (_, i, j) in distances.into_iter().take(1000) {
            disjointset.union(i, j);
        }

        let mut sizes = disjointset.nodes.iter().map(|n| n.size).collect::<Vec<_>>();
        sizes.sort_by(|a, b| b.cmp(&a));

        sizes[0..3].iter().fold(1, |acc, x| acc * x)
    }

    fn part_b(input: &Self::Input) -> impl std::fmt::Display {
        -1
    }
}

aoc_test!(Playground, "day8", 40, 131580, 0, 0);
