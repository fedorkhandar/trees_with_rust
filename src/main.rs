#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    // pub fn insert(&mut self, val: i32) {
    //     if val < self.val {
    //         if let Some(left) = &mut self.left {
    //             left.insert(val);
    //         } else {
    //             self.left = Some(Box::new(TreeNode::new(val)));
    //         }
    //     } else {
    //         if let Some(right) = &mut self.right {
    //             right.insert(val);
    //         } else {
    //             self.right = Some(Box::new(TreeNode::new(val)));
    //         }
    //     }
    // }
}

fn volume(d: u32) -> i32 {
    if d == 0 {
        1
    } else {
        i32::pow(2, d - 1) + volume(d - 1)
    }
}

fn create_tree_dfs(d: u32, start: i32) -> Option<Box<TreeNode>> {
    if d == 0 {
        None
    } else {
        let mut root = Box::new(TreeNode::new(start));
        root.left = create_tree_dfs(d - 1, start + 1);
        root.right = create_tree_dfs(d - 1, volume(d - 1) + start);
        Some(root)
    }
}

fn print_tree(root: Option<&Box<TreeNode>>, indent: u32) {
    if root.is_none() {
        return;
    }

    println!("{}{}", " ".repeat(indent as usize), root.unwrap().val);
    print_tree(root.unwrap().left.as_ref(), indent + 4);
    print_tree(root.unwrap().right.as_ref(), indent + 4);
}
fn main() {
    // let v = vec![4, 2, 6, 1, 3, 5, 7];
    // let mut root = Box::new(TreeNode::new(v[0]));
    // for i in 1..v.len() {
    //     root.insert(v[i]);
    // }
    let root = create_tree_dfs(4, 0);
    print_tree(root.as_ref(), 0);
}
