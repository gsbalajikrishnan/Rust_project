pub fn func_match()
{
    let a = 10;
    let b = 20;
    let c = 30;

    match (a,b,c) {
        (a,_,_) if a>b && a>c =>  println!("A is greater"),
        (_,b,_) if b>a && b>c =>  println!("B is greater"),
        (_,_,c) => println!("C is greater"),
    }
}