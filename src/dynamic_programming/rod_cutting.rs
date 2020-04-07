use std::cmp::max;

pub fn recursive_rod(prices: &Vec<i32>, n: i32) -> i32 {
    if n == 0 {
        return 0
    }
    let mut q = -1;
    for i in 0..n {
        q = max(q, &prices[i as usize] + recursive_rod(&prices, n - i -1));
    }
    return q;
}

pub fn bottom_up_dynamic_rod(prices: &Vec<i32>, n: i32) -> i32 {
    let mut rods: Vec<i32> = Vec::new();
    rods.push(0);
    let mut q;
    for j in 1..n as usize+1{
        q = -1;
        for i in 1..j+1 {
            q = max(q, prices[i-1] + rods[j - i]);
        }
        rods.push(q);
    }
    return rods[n as usize];
}

pub fn top_down_dynamic_rod(prices: &Vec<i32>, n: i32) -> i32 {
    let mut rods: Vec<i32> = Vec::new();
    for i in 0..n {
        rods.push(-1);
    }
    return top_down_dynamic_rod_aux(&prices, n, &mut rods);
}

fn top_down_dynamic_rod_aux(prices: &Vec<i32>, n: i32,mut rods: &mut Vec<i32>) -> i32 {
    let k = n as usize;
    let mut q;
    if n == 0 {
        return 0;
    }
    if rods[k-1] >= 0{
        return rods[k-1]
    }

    q = -1;
    for i in 1..k+1 {
        q = max(q, prices[i - 1] + top_down_dynamic_rod_aux(&prices, n - i as i32, &mut rods));
    }
    rods[k-1] = q;
    return q;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let v: Vec<i32> = vec![1, 5, 8, 9, 10, 17, 17, 20, 24, 30];
        let r1 = recursive_rod(&v, 4);
        let r2 = bottom_up_dynamic_rod(&v, 4);
        let r3 = top_down_dynamic_rod(&v, 4);
        assert_eq!(r1, r2, "Recursive method not equal to bottom up method.");
        assert_eq!(r2, r3, "Bottum up method not equal to top down method");
    }
}