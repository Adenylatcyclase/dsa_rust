use std::cmp::max;

pub fn recursive_rod(prices: &[i32], n: i32) -> (Vec<i32>, i32) {
    let mut v = Vec::new();
    if n == 0 {
        return (v, 0);
    }
    let mut q = -1;
    
    for (i, value) in prices.iter().take(n as usize).copied().enumerate() {
        let (vt, qt) = recursive_rod(&prices, n - i as i32 -1);
        if qt + value > q {
            q = qt +value;
            v = vt;
            v.push(i as i32+ 1);
        }
    }
    return (v, q);
}

pub fn bottom_up_dynamic_rod(prices: &[i32], n: i32) -> (Vec<i32>, i32) {
    let mut rods: Vec<(Vec<i32>, i32)> = Vec::new();
    rods.push((Vec::new(), 0));
    let mut q;
    for j in 1..=n as usize{
        let mut v = Vec::new();
        q = -1;
        for i in 1..=j {
            let (vt, qt) = &rods[j - i];
            if qt + prices[i-1] > q {
                q = qt + prices[i-1];
                v = vt.clone();
                v.push(i as i32);
            }
        }
        rods.push((v, q));
    }
    let t = &rods[n as usize];
    (t.0.clone(), t.1)
}

pub fn top_down_dynamic_rod(prices: &[i32], n: i32) -> (Vec<i32>, i32) {
    let mut rods = vec![(Vec::new(), -1); n as usize];
    return top_down_dynamic_rod_aux(&prices, n, &mut rods);
}

fn top_down_dynamic_rod_aux(prices: &[i32], n: i32,mut rods: &mut Vec<(Vec<i32>, i32)>) -> (Vec<i32>, i32) {
    let mut q;
    if n == 0 {
        return (Vec::new(), 0);
    }
    
    if rods.last().unwrap().1 >= 0{
        let t = rods.last().unwrap();
        return (t.0.clone(), t.1);
    }

    q = -1;
    let mut v = Vec::new();
    for (i, value) in prices.iter().take(n as usize).copied().enumerate() {
        let (vt, qt) = top_down_dynamic_rod_aux(&prices, n - i as i32 - 1, &mut rods);
        if qt + value > q {
            q = qt + value;
            v = vt.clone();
            v.push(i as i32 + 1);
        }
        
    }
    rods.last_mut().replace(&mut (v.clone(), q));
    (v, q)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let v: Vec<i32> = vec![1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        let (v1, r1) = recursive_rod(&v, 4);
        let (v2,r2) = bottom_up_dynamic_rod(&v, 4);
        let (v3, r3) = top_down_dynamic_rod(&v, 4);
        assert_eq!(10, r1, "Wrong answer");
        assert_vec(&vec![2, 2], &v1);
        assert_eq!(r1, r2, "Recursive method not equal to bottom up method.");
        assert_eq!(r2, r3, "Bottum up method not equal to top down method");
        println!("{}", print_vec(&v1));
        println!("{}", print_vec(&v2));
        println!("{}", print_vec(&v3));
        assert_vec(&v1, &v2);
        assert_vec(&v2, &v3);
    }

    fn print_vec(v: &Vec<i32>) -> String {
        let mut s = "[".to_string();
        let mut b = false;
        for vv in v.iter() {
            if b {
                s.push_str(", ");
            } else {
                b = true;
            }
            s.push_str(&vv.to_string());
        }
        s.push_str("]");
        return s;
    }
    fn assert_vec(v1: &Vec<i32>, v2: &Vec<i32>) {
        assert_eq!(v1.len(), v2.len(), "Vectors of different lengths");
        for i in 0..v1.len() {
            assert_eq!(v1[i], v2[i], "{}", format!("Element unequal at {}", i));
        }
    }
}