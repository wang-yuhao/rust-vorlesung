macro_rules! find_min {
    ($x:expr) => { $x:expr  };
    (($x:expr),$($y:expr),+) => {   std::cmp::min($x,find_min($($y),+))};
}

fn main(){
    println!("{}",find_min(1i32);
    println!("{}",find_min(1i32,5i32 * 2));
    println!("{}",find_min(1i32,5i32 * 2,8i32));
}