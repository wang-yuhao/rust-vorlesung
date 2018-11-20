enum polynomeial{
    coefficients(i32,i32,i32,i32),
    exponents(i32,i32,i32,i32),
}

fn newton(){

}

fn main(){
    let mut coe: polynomeial = polynomeial::coefficients(1,-2,-11,1);
    let mut exp: polynomeial = polynomeial::exponents(3,2,1,0);
    println!("{}",coe );
}