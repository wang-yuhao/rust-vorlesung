// extern crate timely;
// use timely::dataflow::operators::{ToStream, input};

// fn main(){
//     timely::execute_from_args(std::env::args(), |worker| {
// let index = worker.index();
// let mut input = input::Handle::new();
// // creation of dataflow network
// let probe = worker.dataflow(|scope| {
// let _s1 = input.to_stream(scope); // interactive
// let _s2 = (0..9).to_stream(scope); // predefined
// });
// for round in 0..10 {
// if index == 0 {
// input.send(round); // induce data into DF network
// }
// input.advance_to(round + 1);
// worker.step_while(|| probe.less_than(input.time()));
// }
// });
// }

// fn less_than


extern crate timely;
use timely::dataflow::operators::{ToStream, input};
fn main() {
timely::execute_from_args(std::env::args(), |worker| {
// interactive source w/ timestamp type, data type
let mut input = input::Handle::<(), String>::new();
worker.dataflow(|scope| {
let _s1 = input.to_stream(scope); // interactive
let _s2 = (0..9).to_stream(scope); // predefined
});
}).unwrap(); 

let mut input = input::Handle::new();
worker.dataflow(|scope| {
input.to_stream(scope)
.inspect(|x| println!("{}", x))
});
}