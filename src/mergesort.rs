use super::CloneSorter;

pub struct MergeSort;

impl CloneSorter for MergeSort {
    fn clone_sort<T>(&self, slice: &mut [T])
    where
        T: Ord+Clone,
    {
        //FIXME: not require T: Clone
        //FIXME: not require n log(n) memory
        //divide the list in half
        if slice.len() <= 1 {
            //Slices of 0 or 1 elements are already sorted
            return;
        }
        let (left, right) = slice.split_at(slice.len()/2);
        let (mut left, mut right) = (left.to_vec(), right.to_vec());
        self.clone_sort(&mut left);
        self.clone_sort(&mut right);
        //merge into original list
        let (mut i_left, mut i_right) = (0,0);
        for i in 0..slice.len(){
            if i_left >= left.len(){
                //copy the rest of right to the slice
                slice[i..].swap_with_slice(&mut right[i_right..]);
                return;
            }
            if i_right >= right.len(){
                //copy the rest of left to the slice
                slice[i..].swap_with_slice(&mut left[i_left..]);
                return;
            }
            slice[i] = if left[i_left] < right[i_right]{
                let val = left[i_left].clone();
                i_left += 1;
                val
            } else {
                let val = right[i_right].clone();
                i_right += 1;
                val
            };
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![4, 2, 5, 3, 1];
    MergeSort.clone_sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4, 5]);
}

