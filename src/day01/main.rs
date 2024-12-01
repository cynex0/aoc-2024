fn get_distances(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    let mut distances = Vec::with_capacity(a.len());
    let mut a_sorted = a.to_vec();
    let mut b_sorted = b.to_vec();
    a_sorted.sort();
    b_sorted.sort();

    for (a, b) in a_sorted.iter().zip(b_sorted.iter()) {
        distances.push(a.abs_diff(*b));
    }

    distances
}

fn main() {
    let a = vec![3, 4, 2, 1, 3, 3];
    let b = vec![4, 3, 5, 3, 9, 3];
    let distances = get_distances(a, b);
    let sum: u32 = distances.iter().sum();
    println!("{}", sum);
}
