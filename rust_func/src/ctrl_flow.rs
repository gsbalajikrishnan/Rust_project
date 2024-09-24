pub fn func_match()
{
    let arr:[u8;5] = [10,30,55,66,99];

    match arr {
        (a,_,_) if a>b && a>c =>  println!("A is greater"),
        (_,b,_) if b>a && b>c =>  println!("B is greater"),
        (_,_,c) => println!("C is greater"),
    }

   /*  if a>b && a>c
    {
        println!("A is greater");
    }
    else if b>a && b>c
    {
        println!("B is greater");
    }
    else{
        println!("C is greater");
    } */
    //println!("{} {} {}",a,b,c);
}