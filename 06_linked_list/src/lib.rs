//! # Linked List
//!
//! This module contains solutions for linked list problems
//! from the NeetCode 250 list.


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub mod p074_reverse_a_linked_list;
pub mod p075_merge_two_sorted_linked_lists;
pub mod p076_linked_list_cycle_detection;
pub mod p077_reorder_linked_list;
pub mod p078_remove_node_from_end_of_linked_list;
pub mod p079_copy_linked_list_with_random_pointer;
pub mod p080_add_two_numbers;
pub mod p081_find_duplicate_integer;
pub mod p082_reverse_linked_list_ii;
pub mod p083_design_circular_queue;
pub mod p084_lru_cache;
pub mod p085_lfu_cache;
pub mod p086_merge_k_sorted_linked_lists;
pub mod p087_reverse_nodes_in_k_group;
