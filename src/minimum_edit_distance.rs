/* This Source Code Form is subject to the terms of the Mozilla Public
 *  * License, v. 2.0. If a copy of the MPL was not distributed with this
 *   * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use ndarray::OwnedArray;
use std::cmp::min;
use unicode_segmentation::UnicodeSegmentation;


pub fn minimum_edit_distance(source: &str, target: &str) -> i32 {
    // Split each word into its graphemes
    let source_graphemes = UnicodeSegmentation::graphemes(source, true).collect::<Vec<&str>>();
    let target_graphemes = UnicodeSegmentation::graphemes(target, true).collect::<Vec<&str>>();

    let n = source_graphemes.len() + 1;
    let m = target_graphemes.len() + 1;

    // Solve as a search problem with a dynamic programming technique
    let mut d = OwnedArray::zeros((n,m));

    // Distance from the empty string
    d[(0, 0)] = 0;


    // Find the minimum and maximum solutions from the empty string
    for i in 1..n {
        d[(i, 0)] =
            d[(i - 1, 0)] + deletion_cost(source_graphemes[i - 1]);
    }

    for j in 1..m {
        d[(0, j)] =
            d[(0, j - 1)] + insertion_cost(target_graphemes[j - 1]);
    }

    // Recurrence relation
    for i in 1..n {
        for j in 1..m {
            // Find minima
            d[(i, j)] =
                min(d[(i - 1, j    )] + deletion_cost(source_graphemes[i - 1]),
                min(d[(i - 1, j - 1)] + substitution_cost(source_graphemes[i - 1], target_graphemes[j - 1]),
                    d[(i,     j - 1)] + insertion_cost(target_graphemes[j - 1])));

        }
    }

    d[(n - 1, m - 1)]
}

fn insertion_cost(_: &str) -> i32 {
    1
}

fn deletion_cost(_: &str) -> i32 {
    1
}

// This could be weighted based on probabilities of a substitution
fn substitution_cost(a: &str, b: &str) -> i32 {
    if a == b { 0 } else { 2 }
}

#[test]
fn test_levenshtein_distance() {
   assert_eq!(8, minimum_edit_distance("intention", "execution"));
   assert_eq!(4, minimum_edit_distance("book", "back"));
   assert_eq!(2, minimum_edit_distance("at", "to"));
   assert_eq!(13, minimum_edit_distance("ferocious", "draining"));
}

