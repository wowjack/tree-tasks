fn main() {
    println!("Hello, world!");
}


struct BinaryTreeNode<I: PartialOrd> {
    data: I,
    left: Option<Box<BinaryTreeNode<I>>>,
    right: Option<Box<BinaryTreeNode<I>>>,
}
impl<I: PartialOrd> BinaryTreeNode<I> {
    pub fn new(data: I) -> Self {
        Self {
            data,
            left: None,
            right: None
        }
    }

    pub fn insert(&mut self, item: I) {
        if item < self.data {
            if let Some(t) = &mut self.left {
                t.insert(item);
            } else {
                self.left = Some(Box::new(BinaryTreeNode::new(item)));
            }
        } else {
            if let Some(t) = &mut self.right {
                t.insert(item);
            } else {
                self.right = Some(Box::new(BinaryTreeNode::new(item)));
            }
        }
    }

    pub fn get_mut(&mut self, cmp: I) -> Option<&mut I> {
        match self.data.partial_cmp(&cmp) {
            None => None,
            Some(std::cmp::Ordering::Equal) => Some(&mut self.data),
            Some(std::cmp::Ordering::Greater) => self.right.as_mut().map(|t| t.get_mut(cmp)).flatten(),
            Some(std::cmp::Ordering::Less) => self.left.as_mut().map(|t| t.get_mut(cmp)).flatten(),
        }
    }

    pub fn remove(&mut self, item: I) {
        // Find the parent of the node to be removed
        // Take ownership and split off the child tree
        // Modify child tree with node-to-be-removed at the root
        // Reattach child tree to parent
        
    }
}