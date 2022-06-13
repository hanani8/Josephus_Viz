use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn josephus(n: u64, k: u64) -> Vec<u64> {
    use std::collections::VecDeque;
    let mut vec:VecDeque<u64> = VecDeque::new();
    let mut ans:Vec<u64> = Vec::new();
    
    for i in 1..n+1 {
        vec.push_back(i)
    }
    let mut x = n.clone();
    while x > k {
        vec.rotate_left(k as usize);
        let second = vec.pop_front().unwrap();
        ans.push(second);
        x -= 1;
    }
    let mut y = vec.len();
    let mut idx:usize = 0;
    let k_usize:usize = k as usize;
    while y > 0 {

        idx += k_usize;
        idx %= y;
        vec.rotate_left(idx);
        let second = vec.pop_front().unwrap();
        ans.push(second);
        y -= 1;
        idx = 0;
    }
    println!("{:?}", ans);
    return ans
}