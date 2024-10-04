use std::collections::HashSet;

fn intersection<T>(set1: &HashSet<T>, set2: &HashSet<T>) -> HashSet<T>
where T: Eq + std::hash::Hash + Clone {
    let mut result = HashSet::new();

    for item in set1 {
        if set2.contains(item) {
            result.insert(item.clone());
        }
    }

    result
}

fn main() {
    let set1: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();

    let intersection = intersection(&set1, &set2);

    println!("Пересечение множеств: {:?}", intersection);
}
