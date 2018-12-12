
use std::collections::HashMap;

macro_rules! hmap {
    (   $(
        $elem:expr => $num:expr
    ),
      *  
    ) =>    {
        {
            let mut hm = HashMap::new();
            $(
                hm.insert($elem,$num);
            )*
            hm
        }
    }
}

#[allow(unused_macros)]
macro_rules! hmap_cap {
    {@unit $($x:tt)*} => {()};
    { @count $($rest:expr),* } => {
        {
            let_slice:&[()] = &[$(hmap_cap!(@unit $rest)),*];
            _slice.len()
        }
    };

    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hmap_cap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key,$value);
            )*
            _map
        }
    };
}

fn main(){
    let m1: HashMap<i32,String> = hmap![];
    println!("m1: {:#?}",m1 );
    let m2 = hmap!["christmas" => 25, "presents" => 24, "vacation" => 22];
    println!("m2: {:#?}",m2 );

    assert_eq!(m2["vacation"], 22);
    assert!(!m2.contains_key("cookies"))
    
}