fn quicksort_basic(arr: &mut Vec<i32>){
    let hi = arr.len() as isize - 1;
    quicksort_helper(arr, 0, hi);
}



fn quicksort_helper(arr: &mut Vec<i32>, lo: isize, hi: isize){
    if lo <= hi{
        let pivot = partition(arr,lo,hi);
        quicksort_helper(arr,lo,hi);
        quicksort_helper(arr,pivot + 1, hi);
    }
}

fn partition(arr: &mut Vec<i32>, lo: isize, hi: isize) -> isize{
    let pivot = arr[hi as usize];
    let mut i = lo;

    for j in lo..hi{
        if arr[j as usize] < pivot{
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }
    arr.swap(i as usize, hi as usize);
    i
}


fn main(){
    let mut arr = vec![3,6,7,5,2,1,4,8];
   // print!{"{:?}",quicksort_basic(&mut arr)};
   quicksort_basic(&mut arr);
}