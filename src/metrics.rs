//! This is a collection of string metrics that are suitable for use with a
//! BK-tree.

use Metric;

extern crate triple_accel;
use self::triple_accel::{hamming};
use self::triple_accel::hamming::{hamming_search_naive_with_opts};
use metrics::triple_accel::SearchType::Best;

/// This calculates the Levenshtein distance between two strings.
///
/// The [distance metric itself][1] is calculated using the [Wagner-Fischer][2]
/// dynamic programming algorithm.
///
/// # Examples
///
/// ```
/// use bk_tree::Metric;
/// use bk_tree::metrics::Levenshtein;
///
/// assert_eq!(Levenshtein.distance("bar", "baz"), 1);
/// assert_eq!(Levenshtein.distance("kitten", "sitting"), 3);
/// ```
///
/// [1]: https://en.wikipedia.org/wiki/Levenshtein_distance
/// [2]: https://en.wikipedia.org/wiki/Wagner%E2%80%93Fischer_algorithm
#[derive(Debug)]
pub struct Hamming;

impl<K: AsRef<str> + ?Sized> Metric<K> for Hamming
{
    fn distance(&self, a: &K, b: &K) -> u32 {
        let a_bytes = a.as_ref().as_bytes();
        let b_bytes = b.as_ref().as_bytes();
        hamming(a_bytes, b_bytes)
    }

    fn threshold_distance(&self, a: &K, b: &K, threshold: u32) -> Option<u32> {
        let a_bytes = a.as_ref().as_bytes();
        let b_bytes = b.as_ref().as_bytes();
        hamming_search_naive_with_opts(a_bytes, b_bytes, threshold, Best)
    }
}
