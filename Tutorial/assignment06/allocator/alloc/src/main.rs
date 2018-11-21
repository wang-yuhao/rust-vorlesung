extern crate libc;
extern crate rand;
use rand;

use libc::{c_void, size_t, uint64_t};



#[link(name = "alloc")]
extern "C" {
    fn heapalloc(size: size_t) -> *mut c_void;
    fn heapfree(ptr: *mut c_void);
    fn rdtsc() -> uint64_t;
}

struct Mem<'a>(&'a mut [u8]);

impl<'a> Mem<'a> {
    fn new(size: usize) -> Mem<'a> {
        unsafe {
            let ptr = heapalloc(size as size_t);
            Mem(std::slice::from_raw_parts_mut(ptr as *mut u8, size))
        }
    }
}

impl<'a> Drop for Mem<'a> {
    fn drop(&mut self) {
        unsafe {
            heapfree(self.0 as *mut _ as *mut c_void);
        }
    }
}


struct Cycles{start: u64}
impl Cycles{
    fn start()->Cycles{
        unsafe{
            Cycles{start:rdtsc()}
        }
    }

    fn stop(self)->u64{
        unsafe{rdtsc()-self.start}
    }
}


fn print_histogram(h: Vec<u64>){
    let factor = h.iter().max().unwrap()/50;
    for exp in 4..14{
        println!("{:2}: {}",exp, "+".repeat((h[exp-4]/factor) as usize));
    }
}


fn main() {
    //Creat random numbers between 4 and including 13
    let step = Range::new(4, 14);
    let mut rng = rand::thread_rng();

    //vector to collect time measurements
    let mut cycles: Vec<Vec<u64>> = std::iter::repeat(vec![]).take(10).collect();

    //Vector to collect the memory blocks 
    //account for measuring errors
    for _ in 0..100_000{
        //Randomly set the size of the allocated memory block between
        //16B and 4KB
        let exp = step.ind_sample(&mut rng);
        let blocksize = 2usize.pow(exp);

        //Start time measurement
        let time = Cycles::start();

        //Allocate memory block.
        let m = Mem::new(blocksize);

        //Pseudo-Output: Do console output conditionally under a
        //condition that never holds, so that the complier cannot
        //optimize away the read accesses to the memory block
        if exp == 100{
            for b in m.0.iter(){
                print!("{:02x}", b);
            }
        }

        //stop time measurment
        let time = time.stop();
        blocks.push(m);

        //Add time measurement to correct bin of vector
        cycles[(exp-4) as usize].push(time);
    }

    //Average measurement for all blocksizes
    let histogram: Vec<u64> = cycles.iter()
        .map(|vec| vec.iter().sum::<u64>()/vec.len() as u64)
        .collect();
    print_histogram(histogram);
    // let m = Mem::new(16);
    // for b in m.0.iter() {
    //     print!("{:02x} ", b); } println!();
}
