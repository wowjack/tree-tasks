// mod list

fn main() {
    println!("Hello, world!");
}


struct BinaryTree<I: PartialOrd> {
    head: Option<BinaryTreeNode<I>>,
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
        match self.data.partial_cmp(&item) {
            None => panic!("what"),
            Some(std::cmp::Ordering::Equal | std::cmp::Ordering::Greater) => {
                if let Some(t) = &mut self.right {
                    t.insert(item);
                } else {
                    self.right = Some(Box::new(BinaryTreeNode::new(item)));
                }
            },
            Some(std::cmp::Ordering::Less) => {
                if let Some(t) = &mut self.left {
                    t.insert(item);
                } else {
                    self.left = Some(Box::new(BinaryTreeNode::new(item)));;
                }
            }
        }
    }

    pub fn get_mut(&mut self, cmp: I) -> Option<&mut Self> {
        match self.data.partial_cmp(&cmp) {
            None => None,
            Some(std::cmp::Ordering::Equal) => Some(self),
            Some(std::cmp::Ordering::Greater) => self.right.as_mut().map(|t| t.get_mut(cmp)).flatten(),
            Some(std::cmp::Ordering::Less) => self.left.as_mut().map(|t| t.get_mut(cmp)).flatten(),
        }
    }

    /// Must take ownership of self because the root node may be removed
    pub fn remove(mut self, item: I) -> Option<Self> {
        // get node to be removed
        // if it has two or one children, 

        let Some(mut to_remove) = self.get_mut(item) else { return Some(self) };

        None
    }
}