// // extern crate timely;
// // use timely::dataflow::operators::{ToStream, input};

// // fn main(){
// //     timely::execute_from_args(std::env::args(), |worker| {
// // let index = worker.index();
// // let mut input = input::Handle::new();
// // // creation of dataflow network
// // let probe = worker.dataflow(|scope| {
// // let _s1 = input.to_stream(scope); // interactive
// // let _s2 = (0..9).to_stream(scope); // predefined
// // });
// // for round in 0..10 {
// // if index == 0 {
// // input.send(round); // induce data into DF network
// // }
// // input.advance_to(round + 1);
// // worker.step_while(|| probe.less_than(input.time()));
// // }
// // });
// // }

// // fn less_than


// //extern crate timely;
// //use timely::dataflow::operators::{ToStream, input};
// //fn main() {
// //timely::execute_from_args(std::env::args(), |worker| {
// //// interactive source w/ timestamp type, data type
// //let mut input = input::Handle::<(), String>::new();
// //worker.dataflow(|scope| {
// //let _s1 = input.to_stream(scope); // interactive
// //let _s2 = (0..9).to_stream(scope); // predefined
// //});
// //}).unwrap(); 
// //
// //let mut input = input::Handle::new();
// //worker.dataflow(|scope| {
// //input.to_stream(scope)
// //.inspect(|x| println!("{}", x))
// //});
// //}


// extern crate timely;

// use timely:: dataflow::operators
//     ::{Concat,Exchange,Inspect,Map,
//         feedback::{ConnectLoop,LoopVariable},
//         input::{self,Input}};

// fn main(){
//     timely::execute_from_args(std::env::args(), |worker| {
    
//         let index = worker.index();
//         println!("Here is worker {}, reporting {} peers.",
//                  index,worker.peers());

//         let mut input = input::Handle::new();

//         worker.dataflow(|scope| {
//             //loop at most 9 times, increment timestamp by 1 every iteration
//             let (handle, stream) = scope.loop_variable(9,1);

//             scope.input_from(&mut input)
//                 .concat(&stream)
//                 //move is going to invoke the Copy trait
//                 .inspect(move |x| println!("worker {}: {}", index, x))
//                 .map(|x| x + 1)
//                 .exchange(|&x| x)
//                 .connect_loop(handle)
//         } );


//         //first worker starts counting
//         if index == 0 {
//             input.send(1);
//         }
//     } ).unwrap();
// }






extern crate timely;

use timely::dataflow
    ::{channels::pact::Pipeline,
        operators::{Concat,Exchange, Inspect,
                    feedback::{ConnectLoop, LoopVariable},
                    generic::Operator,
                    input::{self, Input}}};

fn main(){
    timely::execute_from_args(std::env::args(), |worker|{
        let index = worker.index();
        println!("Here is worker {}, reporting {} peers. ", index, worker.peers());

        let mut input = input::Handle::new();

        worker.dataflow(|scope|{
            //loop at most 20 times, increment timestamp by 2 every iteration
            let (handle, stream) = scope.loop_variable(20,2);

            scope.input_from(&mut input)
                .concat(&stream)
                .inspect(move |x| println!("worker {}: {}", index, x))
                .unary(Pipeline,"limitadder",|_capability, _opinfo|
                        |input, output|{
                            while let Some((time,data)) = input.next(){
                                let mut session = output.session(&time);
                                for &d in data.iter(){
                                    if d<10 { session.give(d+1);}
                                }
                            }
                        })
                .exchange(|&x| x)
                .connect_loop(handle);
        });

        if index == 0{
            input.send(1);
        }
    }).unwrap();
}