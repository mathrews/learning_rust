pub fn transpose(matrix: [[i32; 3]; 3]) -> Vec<Vec<i32>> {
    let mut vec: Vec<i32> = Vec::new();
    let mut vec_bid: Vec<Vec<i32>> = Vec::new();
    for i in 0..matrix.len() {
        for i1 in 0..3 {
            vec.push(matrix[i1][i]);
        }
        vec_bid.push(vec.clone());
        let vec_reseted: Vec<i32> = Vec::new();
        vec = vec_reseted;
    }
    println!("{:#?}", vec_bid);
    vec_bid
}

