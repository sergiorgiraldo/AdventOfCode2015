fn main() {
    let n = 5;
    let mut arr = vec![vec![0; n-1]; 100usize.pow(n as u32-1) as usize];
    let mut idx = 0;
    for i in 1..=10 {
        for j in 1..=10 {
            if i != j {
                for k in 1..=10 {
                    if k != i && k != j {
                        for l in 1..=10 {
                            if l != i && l != j && l != k {
                                for m in 1..=10 {
                                    if m != i && m != j && m != k && m != l {
                                        arr[idx] = vec![i, j, k, l, m];
                                        idx += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", arr);
}