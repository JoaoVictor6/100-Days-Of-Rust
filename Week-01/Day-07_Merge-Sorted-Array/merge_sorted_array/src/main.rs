fn main() {
    println!("Hello, world!");
    merge_sorted_array(vec![1,2,3,0,0,0], vec![2,5,6]);
}

fn merge_sorted_array (nums1: Vec<i8>, nums2: Vec<i8>) -> Vec<i8> {
    let mut merged_vector: Vec<i8> = Vec::new();
    let mut first_vec = nums1;
    let mut second_vec = nums2;
    merged_vector.append(&mut first_vec);
    merged_vector.append(&mut second_vec);
    merged_vector.sort();

    println!("{:?}", merged_vector);
    return merged_vector;
}
