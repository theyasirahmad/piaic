fn main() {
    
    
    // println!("Hello, world!");
    
    // array immutable
    // let fruits = ["apple", "mango", "banana"];
    // println!("{:?}",fruits );

    // let mut vegs = ["apple", "mango", "banana"];
    // println!("{:?}",vegs );

    // vegs[0]="potato";
    // vegs[1]="tomato";

    // tuple 

    // let mut a = (1,2,3,"apple",5);

    // println!("{:?}",a ); 

    // a.3 = "banana";

    // println!("{:?}",a );

    // expression returns single value
    // statement k end  mein terminator 

    // fn first()-> i32{
    //     5
    // }
    // let a = first();
    // println!(" Value of A is {}",a );

    // let x=5+5;
    // let y ={
    //     let x=6;
    //     x+1
    // };

    // println!("{}",y )

// control flow --  conditional statements
// if else 

    // let x = 10;
    // let y = 15;

    // if x<y{
    //     println!("bela ciao");
    // }


    // fn marks_sheet(){

    //     let marks = 50;

    //     if marks>=80{
    //         println!("A+");
    //     }
    //     else if marks>=70 && marks < 80 {
    //         println!("A");
    //     }
    //     else if marks>=60 && marks < 70 {
    //         println!("B");
    //     }
    //     else if marks>=50 && marks < 60 {
    //         println!("C");
    //     }
    //     else{
    //         println!("fail");
    //     }
    // }
    
    // marks_sheet();



    // LOOPS

    // loop 
    // for
    // while 

    
    // loop
    
    // let mut x = 0;
    // loop{
    //     println!("{}",x );
    //     x = x + 1;


    //     if x>100000{
    //         break;
    //     }
    // }


    // for loop

    // for i in 1..100{
    //  println!("{}",i );
    // }

    // let fruits = ["apple",  "banana", "mangoes"];

    // for element in fruits.iter(){
    //     println!("{}",element );
    // }

    // println!("");

    // let fruits2 = ["apple",  "banana", "mangoes"];

    // for index in 0..3{
    //     println!("{}",fruits2[index] );
    // }

    // for in reverse

    // let fruits = ["apple",  "banana", "mangoes"];

    // for element in fruits.iter().rev(){
    //     println!("{}",element );
    // }

    // println!("");

    // let fruits2 = ["apple",  "banana", "mangoes"];

    // for index in (0..3).rev(){
    //     println!("{}",fruits2[index] );
    //     println!("{}",index );
    // }


    // while loop

    let mut x = 0 ;
    while x<10{
        println!("hellos");
        println!("{}",x );
        x= x+1 ;
    }

}


