macro_rules! create_func {
    ($func_name:ident) => (
        fn $func_name() {
            println!("function {:?} is called", stringify!($func_name))
        }
    )
}

macro_rules! times{
    // macth like arm for macro
       ($a:expr,$b:expr)=>{
    // macro expand to this code
           {
   // $a and $b will be templated using the value/variable provided to macro
               $a*$b
           }
       }
   }


   macro_rules! add_mut{
    (
  // repeated block
  $($a:expr)
 // seperator
   ,
// zero or more
   *
   )=>{
       { 
   // to handle the case without any arguments
   0
   // block to be repeated
   $(+$a)*
     }
    }
}


 macro_rules! sunny_math {
     {add to  ($a:expr,$b:expr)} => ($a + $b);
     {sub  to  ($a:expr,$b:expr)} => ($a - $b);
     {times to ($a:expr,$b:expr)} => ($a * $b);
     {div to  ($a:expr,$b:expr)} => ($a / $b);
 }

fn main() {
    create_func!(foo);
    foo();

    let x=times!(8.3,2.9);

    println!("{}",x);

    let y=add_mut!(1,2,3,4,5,6,7,8,9,10);
    println!("{}",y);


     println!("{}        {}",sunny_math!(add to (1,2) ),sunny_math!(sub to (4,12) ));
     println!("{}        {} ",sunny_math!(times to (10,2)  ),sunny_math!(div to (10,2)  ));
}
