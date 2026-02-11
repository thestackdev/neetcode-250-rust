//! # Trees
//!
//! This module contains solutions for trees problems
//! from the NeetCode 250 list.


use std::rc::Rc;
use std::cell::RefCell;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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
}

pub mod p088_binary_tree_inorder_traversal;
pub mod p089_binary_tree_preorder_traversal;
pub mod p090_binary_tree_postorder_traversal;
pub mod p091_invert_a_binary_tree;
pub mod p092_depth_of_binary_tree;
pub mod p093_binary_tree_diameter;
pub mod p094_balanced_binary_tree;
pub mod p095_same_binary_tree;
pub mod p096_subtree_of_a_binary_tree;
pub mod p097_lowest_common_ancestor_in_binary_search_tree;
pub mod p098_insert_into_a_binary_search_tree;
pub mod p099_delete_node_in_a_bst;
pub mod p100_level_order_traversal_of_binary_tree;
pub mod p101_binary_tree_right_side_view;
pub mod p102_construct_quad_tree;
pub mod p103_count_good_nodes_in_binary_tree;
pub mod p104_valid_binary_search_tree;
pub mod p105_kth_smallest_integer_in_bst;
pub mod p106_binary_tree_from_preorder_and_inorder_traversal;
pub mod p107_house_robber_iii;
pub mod p108_delete_leaves_with_a_given_value;
pub mod p109_binary_tree_maximum_path_sum;
pub mod p110_serialize_and_deserialize_binary_tree;
