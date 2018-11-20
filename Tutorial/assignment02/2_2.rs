fn main(){
    let mut arr:[i32;8] = [3,5,6,8,2,7,1,4];
    for i in &arr{
     print!("{}", i)}
     println!();
    arr.swap(2,5);
    quicksort_prepare(&mut arr);
    for i in &arr{
     print!("{}", i)}

}

fn quicksort_prepare(arr: &mut [i32;8]){
    let hi = arr.len() as isize - 1;
    quicksort_helper(arr,0,hi);
}

fn quicksort_helper(arr: &mut [i32;8],lo: isize,hi: isize){
    if lo <= hi{
        let pivot = partition(arr,lo,hi);
        quicksort_helper(arr,lo,pivot-1);
        quicksort_helper(arr,pivot + 1, hi);
    }
}


fn partition(arr: &mut [i32;8], lo: isize, __hi: isize) -> isize{
    let pivot = arr[__hi as usize];
    let mut i = lo;

    for j in lo..__hi{
        if arr[j as usize] < pivot{
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }
    arr.swap(i as usize, __hi as usize);
    i
}