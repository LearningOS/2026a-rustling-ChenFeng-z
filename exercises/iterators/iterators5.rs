// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.


use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values().filter(|&&val| val == value).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if *val == value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection
        .iter()
        .map(|map| count_iterator(map, value))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let mut map = HashMap::new();
        map.insert(String::from("Rustlings"), Progress::Complete);
        map.insert(String::from("Book"), Progress::Some);
        map.insert(String::from("Tour"), Progress::Complete);

        assert_eq!(count_iterator(&map, Progress::Complete), 2);
    }

    #[test]
    fn count_equals_for() {
        let mut progress_states = vec![];

        let mut map1 = HashMap::new();
        map1.insert(String::from("Rustlings"), Progress::Complete);
        map1.insert(String::from("Book"), Progress::Some);
        progress_states.push(map1);

        let mut map2 = HashMap::new();
        map2.insert(String::from("Cookbook"), Progress::None);
        map2.insert(String::from("Course"), Progress::Complete);
        progress_states.push(map2);

        assert_eq!(
            count_collection_for(&progress_states, Progress::Complete),
            count_collection_iterator(&progress_states, Progress::Complete)
        );
    }
}
