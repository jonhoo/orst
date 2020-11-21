/*!
An implementation of American Flag sort (https://en.wikipedia.org/wiki/American_flag_sort), in particular inspired by M. Skarupke's 2017 C++Now talk "Sorting in less than O(n log n): Generalizing and optimizing radix sort" (https://www.youtube.com/watch?v=zqs87a_7zxw).

It sorts the slice into 256 buckets (stripes) according to the most significant byte. Then it recursively sorts each bucket with more than one element in it until there are no more bytes to sort by.

Currently only works on `&[T: Sized]`, as it uses `std::mem::size_of::<T>()` to decide that it is done with the recursion. In order to be able to sort `&[T: ?Sized]`, it must be rewritten to accomodate the fact that some `T` are longer than others.
*/

use super::Sorter;

#[cfg(test)]
use rand::seq::SliceRandom;
#[cfg(test)]
use rand::thread_rng;

pub struct RadixSort;

/// Implementing this trait means that an object can be turned into a collection of `u8` keys of decreasing
/// significance, to accomodate the radix sort.
pub trait Bytify
{
    /// Returns the `(n+1)`th most significant byte of `self`, recast to `usize` because it is
    /// exclusively used as an index into arrays. A return of `None` means that `n` goes beyond all
    /// the bytes that `self` can signify.
    ///
    /// # Example
    /// ```
    /// use orst::Bytify;
    /// let x: u8 = 5;
    /// assert_eq!(x.bytify(0), Some(5 as usize));
    /// assert_eq!(x.bytify(1), None);
    /// ```
    fn bytify(&self, n: usize) -> Option<usize>;
}

impl Bytify for u8
{
    fn bytify(&self, n: usize) -> Option<usize>
    {
        if n == 0 {
            return Some(*self as usize);
        }
        else {
            return None;
        }
    }
}

impl Bytify for usize
{
    fn bytify(&self, n: usize) -> Option<usize>
    {
        let bytes = self.to_be_bytes();
        if n >= bytes.len()
        {
            return None;
        }
        return Some(bytes[n] as usize);
    }
}

impl Bytify for i32
{
    fn bytify(&self, n: usize) -> Option<usize>
    {
        if n >= std::mem::size_of::<i32>()
        {
            return None;
        }
        let mut b = self.to_be_bytes()[n];
        if n == 0
        {
            b = b.wrapping_add(128);
        }
        return Some(b as usize);
    }
}

fn radix_sort<T>(slice: &mut [T], level: usize)
    where
        T: Bytify + Sized,
{
    let mut counts: [usize; 256] = [0; 256];
    let mut prefix_sums: [usize; 256] = [0; 256];
    let mut prefix_sums_shift: [usize; 256] = [0; 256];

    for i in slice.iter()
    {
        counts[i.bytify(level).unwrap()] += 1;
    }

    let mut total: usize = 0;
    for i in 0..256
    {
        prefix_sums[i] = total;
        total += counts[i];
        prefix_sums_shift[i] = total;
    }
    
    let mut i: usize = 0;
    while i < slice.len()
    {
        let j = slice[i].bytify(level).unwrap();
        if prefix_sums[j] == prefix_sums_shift[j]
        {
            i += 1;
            continue;
        }
        if prefix_sums[j] == i
        {
            prefix_sums[j] += 1;
            i += 1;
            continue;
        }
        slice.swap(i, prefix_sums[j]);
        prefix_sums[j] += 1;
    }
    if std::mem::size_of::<T>() <= level + 1
    {
        return;
    }
    if prefix_sums[0] > 1
    {
        radix_sort(&mut slice[0..prefix_sums[0]], level + 1);
    }
    for k in 1..256
    {
        if prefix_sums[k-1] + 1 < prefix_sums[k]
        {
            radix_sort(&mut slice[prefix_sums[k-1]..prefix_sums[k]], level + 1);
        }
    }
}

impl<T> Sorter<T> for RadixSort
    where
        T: Bytify,
{
    fn sort(&self, slice: &mut [T])
    {
        radix_sort(slice, 0);
    }
}

#[test]
fn it_works() {
    let mut nums: Vec<usize> = (0..600).collect();
    let mut rng = thread_rng();
    for _ in 0..100
    {
        nums.shuffle(&mut rng);
        RadixSort.sort(&mut nums[..]);
        assert_eq!(nums, (0..600).collect::<Vec<usize>>());
    }
}
