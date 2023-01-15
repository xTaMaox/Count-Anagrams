use std::collections::HashMap;

impl Solution {
    pub fn count_anagrams(s: String) -> i32 {
        let mut mp = HashMap::<char, i32>::new();
        let n = s.len();
        let s = s.chars().collect::<Vec<char>>();
        let mut ret = 1;
        
        for i in 0 ..= n {
            if i == n || s[i] == ' ' {
                let data = mp.values().map(|a| *a).collect::<Vec<i32>>();
                ret = (ret * Self::calculate(&data) as i64) % 1_000_000_007;
                mp.clear();
            } else { *mp.entry(s[i]).or_insert(0) += 1; }
        } 
        
        ret as _
    }
    
    fn calculate(data: &Vec<i32>) -> i32 {
        let n = data.iter().sum::<i32>();
        let mut ret = 1;
        for k in 2 ..= n {
            ret = (ret * k as i64) % 1_000_000_007;
        }
        
        for d in data {
            for k in 2 ..= *d {
                ret = (ret * Self::divide(k) as i64) % 1_000_000_007;
            }
        }
        
        ret as _
    }
    
    fn divide(a: i32) -> i32 {
        let (mut base, mut ret) = (a as i64, 1);
        let mut m = 1_000_000_005;
        while m > 0 {
            if m % 2 == 1 { ret = (ret * base) % 1_000_000_007; }
            m >>= 1;
            base = (base * base) % 1_000_000_007;
        }
        
        ret as _
    }
}