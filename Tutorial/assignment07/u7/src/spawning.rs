fn main(){
    for i in 0..10 {
std::thread::spawn(move || {
println!("{}", i);
});
}
}
