use std::{collections::VecDeque, fmt::Debug};

#[derive(Debug, Clone)]
struct ClusteringFeature {
    n: usize,
    ls: Vec<f64>,
    ss: f64,
}

impl ClusteringFeature {
    fn new(element: Vec<f64>) -> Self {
        ClusteringFeature {
            n: 1,
            ls: element.clone(),
            ss: element.iter().map(|&sub_element| sub_element * sub_element).sum(),
        }
    }

    fn centroid(&self) -> Vec<f64> {
        self.ls.iter().map(|&l| l / self.n as f64).collect()
    }

    fn radius(&self) -> f64 {
        (self.ss / self.n as f64) - (self.centroid().iter().map(|&c| c * c).sum::<f64>().sqrt())
    }

    /// Euclidean distance
    fn distance_0(&self, other: &Self) -> f64 {
        let a: Vec<f64> = self.centroid();
        let b: Vec<f64> = other.centroid();
        a.iter().zip(b.iter()).map(|(&a, &b)| (a - b).powi(2)).sum::<f64>().sqrt()
    }

    #[allow(dead_code)]
    /// Manhattan distance
    fn distance_1(&self, other: &Self) -> f64 {
        let a: Vec<f64> = self.centroid();
        let b: Vec<f64> = other.centroid();
        a.iter().zip(b.iter()).map(|(&a, &b)| (a - b).abs()).sum()
    }
}

impl std::ops::Add for ClusteringFeature {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let n = self.n + other.n;
        let ls = self.ls.iter().zip(other.ls.iter()).map(|(&a, &b)| a + b).collect();
        let ss = self.ss + other.ss;
        ClusteringFeature { n, ls, ss }
    }
}

#[derive(Clone, Debug)]
enum CFNode {
    Leaf{
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
            CFNode::Leaf { features, .. } => features.iter().cloned().reduce(|acc, cf| acc + cf).unwrap(),
            CFNode::NonLeaf { features, .. } => features.iter().cloned().map(|(cf, _)| cf).reduce(|acc, cf| acc + cf).unwrap(),
        }
    }
}

enum UpdateType {
    Simple(usize),
    Split(CFNode, usize),
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
        let mut queue = VecDeque::new();
        queue.push_back((self.root_id, 0));
        while let Some((id, indent)) = queue.pop_front() {
            let sum = self.arena[id].sum();
            match &self.arena[id] {
                CFNode::Leaf { .. } => {
                    println!("{:indent$}Leaf({}, {:?}, {})", "", sum.n, sum.ls, sum.ss, indent=indent);
                },
                CFNode::NonLeaf { features,  .. } => {
                    println!("{:indent$}NonLeaf({}, {:?}, {})", "", sum.n, sum.ls, sum.ss, indent=indent);
                    for (_, child_id) in features {
                        queue.push_back((*child_id, indent + 2));
                    }
                },
            }
        }
    }

    fn insert(&mut self, instance: Vec<f64>) {
        let entry =  ClusteringFeature::new(instance);

        match self.arena.get_mut(self.root_id) {
            Some(root) => {
                let mut current_node = root;
                let update_type;
                // Insering stage
                loop {
                    match current_node {
                        CFNode::Leaf { id, features, prev, next, parent_id } => {
                            let closest_feature_idx = features.iter().enumerate().min_by(|(_, a), (_, b)| {
                                a.distance_0(&entry).partial_cmp(&b.distance_0(&entry)).unwrap()
                            }).unwrap().0;
                            if (features[closest_feature_idx].clone() + entry.clone()).radius() < self.threshold {
                                features[closest_feature_idx] = features[closest_feature_idx].clone() + entry;
                                update_type = UpdateType::Simple(*id);
                            } else if features.len() < self.branching_factor {
                                features.push(entry);
                                update_type = UpdateType::Simple(*id);
                            } else {
                                let (seed_0, seed_1, _) = features.iter().enumerate().flat_map(|(i, f)| {
                                    features.iter().enumerate().map(move |(j, g)| (i, j, f.distance_0(g)))
                                }).max_by(|(_, _, a), (_, _, b)| a.partial_cmp(b).unwrap()).unwrap();
                                let (group_0, group_1): (Vec<_>, Vec<_>) = features.iter().cloned().partition(|cf| cf.distance_0(&features[seed_0]) < cf.distance_0(&features[seed_1]));
                                let new_leaf_0 = CFNode::Leaf {
                                    id: *id,
                                    features: group_0,
                                    prev: *prev,
                                    next: Some(self.next_id),
                                    parent_id: *parent_id,
                                };
                                let new_leaf_1 = CFNode::Leaf {
                                    id: self.next_id,
                                    features: group_1,
                                    prev: Some(*id),
                                    next: *next,
                                    parent_id: *parent_id,
                                };
                                self.next_id += 1;
                                update_type = UpdateType::Split(new_leaf_0, new_leaf_1.id());
                                self.arena.push(new_leaf_1);
                            }
                            break;
                        },
                        CFNode::NonLeaf { features, .. } => {
                            let closest_child_id = features.iter().min_by(|(a, _), (b, _)| {
                                a.distance_0(&entry).partial_cmp(&b.distance_0(&entry)).unwrap()
                            }).unwrap().1;
                            current_node = &mut self.arena[closest_child_id];
                        },
                    }
                }

                // Update stage
                match update_type {
                    UpdateType::Simple(id) => {
                        self.update_upward(id);
                    },
                    UpdateType::Split(node_to_update, id_1) => {
                        // Update phase
                        let id_0 = node_to_update.id();
                        self.arena[id_0] = node_to_update.clone();
                        self.update_upward(id_0);
                        let mut child = self.arena[id_1].clone();
                        let mut parent_id = child.parent_id();
                        loop {
                            match parent_id {
                                Some(p_id) => {
                                    let parent = &mut self.arena[p_id];
                                    match parent {
                                        CFNode::Leaf { .. } => {},
                                        CFNode::NonLeaf { features, .. } => {
                                            features.push((child.sum(), id_1));
                                        },
                                    }
                                    parent_id = parent.parent_id();
                                    child = parent.clone();
                                },
                                None => {
                                    match child {
                                        CFNode::Leaf { .. } => {
                                            println!("Creating a NonLeaf root");
                                            self.arena[id_0].set_parent_id(Some(self.next_id));
                                            self.arena[id_1].set_parent_id(Some(self.next_id));
                                            let new_root = CFNode::NonLeaf {
                                                id: self.next_id,
                                                parent_id: None,
                                                features: vec![(self.arena[id_0].sum(), id_0), (self.arena[id_1].sum(), id_1)],
                                            };
                                            self.root_id = self.next_id;
                                            self.arena.push(new_root);
                                            self.next_id += 1;
                                        },
                                        CFNode::NonLeaf { .. } => {},
                                    }
                                    break;
                                },
                            }
                        }
                        // NonLeaf split phase
                    }
                }
            },
            None => {
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
            },
        }
    }

    fn update_upward(&mut self, id: usize) {
        let mut child = self.arena[id].clone();
        let mut parent_id = child.parent_id();
        while let Some(p_id) = parent_id {
            let parent = &mut self.arena[p_id];
            match parent {
                CFNode::Leaf { .. } => {},
                CFNode::NonLeaf { features, .. } => {
                    features.iter_mut().find(|(_, child_id)| *child_id == child.id()).unwrap().0 = child.sum();
                },
            }
            parent_id = parent.parent_id();
            child = parent.clone();
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

    pub fn insert(&mut self, instance: Vec<f64>) {
        self.tree.insert(instance);
    }

    pub fn print_tree(&self) {
        self.tree.print();
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
