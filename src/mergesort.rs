use std::cmp::Ordering::Less;

use crate::Sorter;

pub struct MergeSort;
impl<T: Ord> Sorter<T> for MergeSort {
    fn sort(&self, slice: &mut [T]) {
        let len = slice.len();
        match len {
            0 | 1 => {}
            2 => {
                if slice[0] > slice[1] {
                    slice.swap(0, 1);
                }
            }
            _ => {
                let (l, r) = slice.split_at_mut(len / 2);
                self.sort(l);
                self.sort(r);
                self.merge(slice, len / 2);
            }
        }
    }
}
impl MergeSort {
    fn merge<T: Ord>(&self, slice: &mut [T], mut r_start: usize) {
        let mut l_start = 0;
        // [merged | l_start..| mid_point ..]
        // in every loop, slice[l_start..r_start] is sorted, slice[r_start..] is sorted,
        // when l's head is less then r's head, increase l_start by 1
        // when r's head is not less then l's head, rotate it to the the index of l_start,
        // then increase l_start and r_start by 1
        while r_start < slice.len() && l_start < r_start {
            match slice[l_start].cmp(&slice[r_start]) {
                Less => {
                    l_start += 1;
                }
                _ => {
                    let r = &mut slice[r_start..];
                    r.rotate_left(1);
                    let unmerged = &mut slice[l_start..];
                    unmerged.rotate_right(1);
                    r_start += 1;
                    l_start += 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut things = vec![4, 2, 5, 3, 1];
        MergeSort.sort(&mut things);
        assert_eq!(things, &[1, 2, 3, 4, 5]);
    }
    #[test]
    fn edge_cases() {
        let mut empty: Vec<u32> = vec![];
        MergeSort.sort(&mut empty);
        assert_eq!(empty, vec![]);
        let mut one_element = vec![0];
        MergeSort.sort(&mut one_element);
        assert_eq!(one_element, vec![0]);
    }
}
