

fn main(){
    fn len<’a>(s: &’a str) -> usize { s.len() }
fn main() {
let t: String = "Higher-Ranked Trait Bounds".to_string();
let l = len(&t); // lifetime of &t used as param to len
println!("{} has {} characters.", t, l);
}
}
