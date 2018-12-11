extern crate timely;

use timely::dataflow
    ::{channels::pact::Pipeline,
        operators::{Concat,Exchange, Inspect,
                    feedback::{ConnectLoop, LoopVariable},
                    generic::operator,
                    input::{self, Input}}};

fn main(){
    timely::execute_from_args(std::env::args(), |worker|{
        let index = worker.index();
        println!("Here is worker {}, reporting {} peers. ", index, worker,peers());

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
                                    if &d<10 { session.give(d+1);}
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