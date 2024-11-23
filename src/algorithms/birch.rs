use std::{collections::VecDeque, fmt::Debug};

type Point = Vec<f64>;

#[derive(Debug, Clone)]
struct ClusteringFeature {
    n: usize,
    ls: Point,
    ss: f64,
}

impl ClusteringFeature {
    fn new(element: Point) -> Self {
        ClusteringFeature {
            n: 1,
            ls: element.clone(),
            ss: element
                .iter()
                .map(|&sub_element| sub_element * sub_element)
                .sum(),
        }
    }

    fn centroid(&self) -> Point {
        self.ls.iter().map(|&l| l / self.n as f64).collect()
    }

    fn radius(&self) -> f64 {
        (self.ss / self.n as f64) - (self.centroid().iter().map(|&c| c * c).sum::<f64>().sqrt())
    }

    /// Euclidean distance_0
    fn distance_0(&self, other: &Self) -> f64 {
        let a: Point = self.centroid();
        let b: Point = other.centroid();
        a.iter()
            .zip(b.iter())
            .map(|(&a, &b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }

    #[allow(dead_code)]
    /// Manhattan distance_0
    fn distance_1(&self, other: &Self) -> f64 {
        let a: Point = self.centroid();
        let b: Point = other.centroid();
        a.iter().zip(b.iter()).map(|(&a, &b)| (a - b).abs()).sum()
    }
}

impl std::ops::Add for ClusteringFeature {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let n = self.n + other.n;
        let ls = self
            .ls
            .iter()
            .zip(other.ls.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        let ss = self.ss + other.ss;
        ClusteringFeature { n, ls, ss }
    }
}

impl std::ops::AddAssign for ClusteringFeature {
    fn add_assign(&mut self, other: Self) {
        self.n += other.n;
        self.ls = self
            .ls
            .iter()
            .zip(other.ls.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        self.ss += other.ss;
    }
}

#[derive(Clone, Debug)]
enum CFNode {
    Leaf {
        id: usize,
        parent_id: Option<usize>,
        features: Vec<ClusteringFeature>,
        prev: Option<usize>,
        next: Option<usize>,
    },
    NonLeaf {
        id: usize,
        parent_id: Option<usize>,
        features: Vec<(ClusteringFeature, usize)>,
    },
}

impl CFNode {
    fn id(&self) -> usize {
        match self {
            CFNode::Leaf { id, .. } => *id,
            CFNode::NonLeaf { id, .. } => *id,
        }
    }

    fn parent_id(&self) -> Option<usize> {
        match self {
            CFNode::Leaf { parent_id, .. } => *parent_id,
            CFNode::NonLeaf { parent_id, .. } => *parent_id,
        }
    }

    fn set_parent_id(&mut self, parent_id: Option<usize>) {
        match self {
            CFNode::Leaf { parent_id: p, .. } => *p = parent_id,
            CFNode::NonLeaf { parent_id: p, .. } => *p = parent_id,
        }
    }

    fn sum(&self) -> ClusteringFeature {
        match self {
            CFNode::Leaf { features, .. } => {
                if features.is_empty() {
                    println!("Empty leaf node");
                }
                features.iter().cloned().reduce(|acc, cf| acc + cf).unwrap()
            }
            CFNode::NonLeaf { features, .. } => features
                .iter()
                .cloned()
                .map(|(cf, _)| cf)
                .reduce(|acc, cf| acc + cf)
                .unwrap(),
        }
    }
}

#[derive(Debug)]
struct CFTree {
    arena: Vec<CFNode>,
    root_id: usize,
    next_id: usize,
    threshold: f64,
    branching_factor: usize,
}

impl CFTree {
    fn new(threshold: f64, branching_factor: usize) -> Self {
        CFTree {
            arena: Vec::new(),
            root_id: 0,
            next_id: 0,
            threshold,
            branching_factor,
        }
    }

    fn print(&self) {
        if self.arena.is_empty() {
            return;
        }
        let mut queue = VecDeque::new();
        queue.push_back((self.root_id, 0));
        while let Some((id, indent)) = queue.pop_front() {
            let sum = self.arena[id].sum();
            match &self.arena[id] {
                CFNode::Leaf { .. } => {
                    println!(
                        "{:indent$}Leaf({}, {:?}, {})",
                        "",
                        sum.n,
                        sum.ls,
                        sum.ss,
                        indent = indent
                    );
                }
                CFNode::NonLeaf { features, .. } => {
                    println!(
                        "{:indent$}NonLeaf({}, {:?}, {})",
                        "",
                        sum.n,
                        sum.ls,
                        sum.ss,
                        indent = indent
                    );
                    for (_, child_id) in features {
                        queue.push_back((*child_id, indent + 2));
                    }
                }
            }
        }
    }

    fn insert(&mut self, instance: Point) {
        let entry = ClusteringFeature::new(instance);

        if self.arena.get(self.root_id).is_some() {
            // Insert
            let mut current_search_id = self.root_id;
            loop {
                match self.arena.get_mut(current_search_id).unwrap() {
                    CFNode::Leaf { features, .. } => {
                        let closest_feature = features
                            .iter_mut()
                            .min_by(|cf_0, cf_1| {
                                cf_0.distance_0(&entry)
                                    .partial_cmp(&cf_1.distance_0(&entry))
                                    .unwrap()
                            })
                            .unwrap();
                        if (closest_feature.clone() + entry.clone()).radius() < self.threshold {
                            // Absorb
                            *closest_feature += entry;
                            self.refresh_tree_from(current_search_id);
                        } else if features.len() < self.branching_factor {
                            // Insert
                            features.push(entry);
                            self.refresh_tree_from(current_search_id);
                        } else {
                            // Split
                            self.split(current_search_id);
                        }
                        break;
                    }
                    CFNode::NonLeaf { features, .. } => {
                        let min_child = features
                            .iter()
                            .min_by(|cf_0, cf_1| {
                                cf_0.0
                                    .distance_0(&entry)
                                    .partial_cmp(&cf_1.0.distance_0(&entry))
                                    .unwrap()
                            })
                            .unwrap();
                        current_search_id = min_child.1;
                    }
                }
            }
        } else {
            let mut features = Vec::with_capacity(self.branching_factor);
            features.push(entry);
            let leaf = CFNode::Leaf {
                id: self.next_id,
                features,
                prev: None,
                next: None,
                parent_id: None,
            };
            self.arena.push(leaf);
            self.next_id += 1;
        }
    }

    fn split(&mut self, node_id: usize) {
        match self.arena.get(node_id).cloned().unwrap() {
            CFNode::Leaf {
                id,
                parent_id,
                features,
                prev,
                next,
            } => {
                let (seed_index_a, seed_index_b, seed_a, seed_b) = features
                    .iter()
                    .enumerate()
                    .flat_map(|(i, p1)| {
                        features
                            .iter()
                            .enumerate()
                            .skip(i + 1)
                            .map(move |(j, p2)| (i, j, p1, p2))
                    })
                    .max_by(|(_, _, p1, p2), (_, _, q1, q2)| {
                        p1.distance_0(p2).partial_cmp(&q1.distance_0(q2)).unwrap()
                    })
                    .unwrap();
                let (mut group_a, mut group_b): (Vec<ClusteringFeature>, Vec<ClusteringFeature>) =
                    features
                        .iter()
                        .cloned()
                        .enumerate()
                        .filter(|(i, _)| i != &seed_index_a && i != &seed_index_b)
                        .map(|(_, p)| p)
                        .partition(|p| seed_a.distance_0(p) < seed_b.distance_0(p));
                group_a.push(seed_a.clone());
                group_b.push(seed_b.clone());
                let b_id = self.next_id;
                self.next_id += 1;
                let mut leaf_split_a = CFNode::Leaf {
                    id: id,
                    features: group_a,
                    prev: prev.clone(),
                    next: Some(b_id),
                    parent_id: parent_id,
                };
                let mut leaf_split_b = CFNode::Leaf {
                    id: b_id,
                    features: group_b,
                    prev: Some(id),
                    next: next.clone(),
                    parent_id: parent_id,
                };
                if let Some(parent_id) = parent_id {
                    // Update parent
                    let sum_a = leaf_split_a.sum();
                    let sum_b = leaf_split_b.sum();
                    self.arena[id] = leaf_split_a;
                    self.arena.push(leaf_split_b);
                    match &mut self.arena[parent_id] {
                        CFNode::NonLeaf { features, .. } => {
                            features.push((sum_b, b_id));
                            features.iter_mut().find(|(_, i)| *i == id).unwrap().0 = sum_a;
                            if features.len() > self.branching_factor {
                                self.split(parent_id);
                            }
                        }
                        _ => {}
                    }
                } else {
                    // Create new root
                    leaf_split_a.set_parent_id(Some(self.next_id));
                    leaf_split_b.set_parent_id(Some(self.next_id));
                    let sum_a = leaf_split_a.sum();
                    let sum_b = leaf_split_b.sum();
                    self.arena[id] = leaf_split_a;
                    self.arena.push(leaf_split_b);

                    let non_leaf = CFNode::NonLeaf {
                        id: self.next_id,
                        parent_id: None,
                        features: vec![(sum_a, id), (sum_b, b_id)],
                    };
                    self.arena.push(non_leaf);
                    self.root_id = self.next_id;
                    self.next_id += 1;
                }
            }
            CFNode::NonLeaf {
                id,
                parent_id,
                features,
            } => {
                let (seed_index_a, seed_index_b, seed_a, seed_b) = features
                    .iter()
                    .enumerate()
                    .flat_map(|(i, p1)| {
                        features
                            .iter()
                            .enumerate()
                            .skip(i + 1)
                            .map(move |(j, p2)| (i, j, p1, p2))
                    })
                    .max_by(|(_, _, p1, p2), (_, _, q1, q2)| {
                        p1.0.distance_0(&p2.0).partial_cmp(&q1.0.distance_0(&q2.0)).unwrap()
                    })
                    .unwrap();
                let (mut group_a, mut group_b): (Vec<(ClusteringFeature, usize)>, Vec<(ClusteringFeature, usize)>) =
                    features
                        .iter()
                        .cloned()
                        .enumerate()
                        .filter(|(i, _)| i != &seed_index_a && i != &seed_index_b)
                        .map(|(_, p)| p)
                        .partition(|p| seed_a.0.distance_0(&p.0) < seed_b.0.distance_0(&p.0));
                group_a.push(seed_a.clone());
                group_b.push(seed_b.clone());
                let b_id = self.next_id;
                self.next_id += 1;
                let mut split_a = CFNode::NonLeaf {
                    id: id,
                    features: group_a,
                    parent_id: parent_id,
                };
                group_b.iter_mut().for_each(|(_, i) | {
                    self.arena[*i].set_parent_id(Some(b_id));
                });
                let mut split_b = CFNode::NonLeaf {
                    id: b_id,
                    features: group_b,
                    parent_id: parent_id,
                };
                if let Some(parent_id) = parent_id {
                    // Update parent
                    let sum_a = split_a.sum();
                    let sum_b = split_b.sum();
                    self.arena[id] = split_a;
                    self.arena.push(split_b);
                    match &mut self.arena[parent_id] {
                        CFNode::NonLeaf { features, .. } => {
                            features.push((sum_b, b_id));
                            features.iter_mut().find(|(_, i)| *i == id).unwrap().0 = sum_a;
                            if features.len() > self.branching_factor {
                                self.split(parent_id);
                            }
                        }
                        _ => {}
                    }
                } else {
                    // Create new root
                    split_a.set_parent_id(Some(self.next_id));
                    split_b.set_parent_id(Some(self.next_id));
                    let sum_a = split_a.sum();
                    let sum_b = split_b.sum();
                    self.arena[id] = split_a;
                    self.arena.push(split_b);

                    let new_root = CFNode::NonLeaf {
                        id: self.next_id,
                        parent_id: None,
                        features: vec![(sum_a, id), (sum_b, b_id)],
                    };
                    self.arena.push(new_root);
                    self.root_id = self.next_id;
                    self.next_id += 1;
                }
            }
        }
    }

    fn refresh_tree_from(&mut self, node_id: usize) {
        let mut current_node_id = node_id;
        while let Some(parent_id) = self.arena[current_node_id].parent_id() {
            let updated_sum = self.arena[current_node_id].sum();
            match &mut self.arena[parent_id] {
                CFNode::NonLeaf { features, .. } => {
                    for (feature, child_id) in features.iter_mut() {
                        if *child_id == current_node_id {
                            *feature = updated_sum;
                            break;
                        }
                    }
                }
                _ => {}
            }
            current_node_id = parent_id;
        }
    }
}

#[derive(Debug)]
pub struct Birch {
    tree: CFTree,
}

impl Birch {
    pub fn new(threshold: f64, branching_factor: usize) -> Self {
        Birch {
            tree: CFTree::new(threshold, branching_factor),
        }
    }

    pub fn insert(&mut self, instance: Point) {
        self.tree.insert(instance);
    }

    pub fn print_tree(&self) {
        self.tree.print();
    }
}

impl super::DataStreamClusteringAlgorithm for Birch {
    fn insert(&mut self, data: Point) {
        self.insert(data);
    }
    fn name(&self) -> String {
        "BIRCH".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clustering_feature() {
        let cf1 = ClusteringFeature::new(vec![2.]);
        let cf2 = ClusteringFeature::new(vec![3.]);
        let cf3 = cf1 + cf2;

        assert_eq!(cf3.centroid(), vec![2.5]);
        assert_eq!(cf3.radius(), 4.);
    }
}
