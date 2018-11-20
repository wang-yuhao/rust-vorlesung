fn smallest(number: u32) -> u32{
    let mut times = 0;
    let mut collatz = number;
    let mut highest_number = collatz;

    while collatz != 1 {
        if collatz % 2 == 0 {
            collatz = collatz / 2;
        } else {
            collatz = 3 * collatz + 1;
        }
        times = times + 1;
        if collatz > highest_number {
            highest_number = collatz;
        } else {
            highest_number = highest_number;
        }
    }
    return(number,times,highest_number)
   // println!("{},{},{}", number, times, highest_number);
}

fn main() {
    let mut number = 1;
    let v = vec![]
    while number <= 100 {
        println!("{},{},{}", smallest(nummber));
        number = number + 1;
        //println!("{},{},{}", smallest(nummber));
    }
}

// let v = vec![12,15,27,29];
//return (.,.) /(.,.)
//let(t1,t2)collatz(.;)
