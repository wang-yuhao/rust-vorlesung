// // let m1: HashMap<i32, String> = hmap![];
// // let m2 = hmap!["key1" => 1, "key2" => 2, "key3" => 3];
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


// macro_rules! myvec {
//     ($(
//         $elem:expr
//     ),
//     *
//     ) => {
//         {
//             let mut v = Vec::new();
//             $(
//                 v.push($elem);
//             )*
//             v
//         }
//     }
// }

fn main(){
    let m1: HashMap<i32, String> = hmap![];
    let m2 = hmap!["key1" => 1, "key2" => 2, "key3" => 3];
    println!("{:?}",m2);
}









// fn main(){
//     let mut contacts = HashMap::new();

//     contacts.insert("Daniel", "798-1364");
//     contacts.insert("Ashley", "645-7689");
//     contacts.insert("Katie", "435-8291");
//     contacts.insert("Robert", "956-1745");

//     let temp = contacts.get("Daniel");
//     assert_eq!(Some(&"798-1364"), temp); 

//     let temp2 = contacts.get(&"Daniel");
//     assert_eq!(Some(&"798-1364"), temp2);
// }

