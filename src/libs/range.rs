use std::{cmp, ops::RangeInclusive};

/// Merge all ranges into a minimal cover.
/// The function requires all ranges to be inclusive, i.e. (3..=6)
///
/// ```
/// use aoc_2025::libs::range;
///
/// let ranges = vec![(3..=5), (4..=9)];
/// let mc = range::minimal_cover(ranges);
///
/// assert_eq!(mc, [(3..=9)]);
/// ```
pub fn minimal_cover(ranges: Vec<RangeInclusive<isize>>) -> Vec<RangeInclusive<isize>> {
    let mut combined = vec![ranges[0].clone()];

    for range in ranges.iter().skip(1) {
        let current: RangeInclusive<isize> = range.clone();
        let j: usize = combined.len() - 1;

        let (cs, ce) = (*current.start(), *current.end());
        let (bs, be) = (*combined[j].start(), *combined[j].end());

        // basically, if one's start is between the last one, extend if possible.
        // so like we have: [3, 7] and we want to merge [4, 9]
        if bs <= cs && cs <= be {
            combined[j] = bs..=cmp::max(ce, be);
        } else {
            combined.push(current);
        }
    }

    combined
}
