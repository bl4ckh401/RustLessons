use std::{vec, collections::HashMap};

fn main(){
    //Scalar Types variables:
    //unsigned u8, u16, u32, u64, u128
    let unsigned:u8 = 10;

    //signed u8, u16, u32, u64, u128
    let signed:i8 = -15;

    //float - decimal
    let float:f32 = 1.0;
    println!("Unsigned: {} Signed: {} Float: {}" , unsigned, signed, float);

    //char char - can only be
    let letter = "c";
    let emoji = "\u{1F600}";
    println!("Hello World {} {}", emoji, letter);

    //boolean True & False
    let is_true = true;

    println!("isTrue: {}", is_true);

    //Arrays in Rust

    let arr:[u8; 3] = [1,2,3];
    let other_arr:[u8; 5] = [100;5];

    //print array size

    println!("index: {} length:{}", arr[0], arr.len());

    //print other array
    println!("{:?}", other_arr);

    //Tuples

    let tuple: (u8, bool, f32) = (5, true, 4.0);
    let tuple2 = (3,5);

    //Print structure of Array and other objects
    println!("First: {} Secone: {} Third:{}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    let (a,b,c) = tuple;
    println!("First: {} Secone: {} Third:{}", a, b, c);

    

    //Mutability

    let mut num = 5;
    num = 3;

    println!("isEven: {}", is_even(num));

    //Array, Slices, Borrowing

    let arr:[u8; 3] = [0,1,2]; // length at compile time
    let slice = &arr[1 .. 3]; // dont know the length


    borrowing_slice(arr, slice);


    //Strings

    let str: &str = "Hello World";
    let mut String:String = String::from("Hello World");

    let slice = &String[..6];
    slice.len();

    String.push('1');
    String.push_str("! Bob");
    String = String.replace("Hello", "Bye");

    println!("{}", String);

    //Contol Structures
    //If-else if else Statement

    let n=3;

    if n > 0{
        println!("Greater than 0")
    } else if n < 0 {
        println!("Less than 0")
    } else {
        println!("Number must be 0")
    }

    //For Loop

    for i in 0 .. 6 {
        println!("{}", i);
    }

    //While

    let mut i = 0;
    while i < 4{
        println!("{}", i);
        i+=1;
        if i == 3{
            println!("exit {}", i);
            break;
        }
    }

    //Match(Switch)
    let x = 5;
    match x {
        0=>println!("0"),
        1 | 2=>println!("1, 2"),
        3 | 4=>println!("3, 4"),
        5 | 6=>println!("5, 6"),
        _ => println!("Default")
    }

    //Struct
    struct Bird {
        name: String,
        attack: u64
    }

    impl Bird {
        fn print_name(&self){
            println!("{}", self.name);
        }
    }

    let name = String::from("Bird");


    let bird = Bird { name, attack:5 };
    bird.print_name();

    //trait

    trait Animal{
        fn can_fly(&self) -> bool;
        fn is_animal(&self) -> bool {
            true
        }
    }

    impl Animal for Bird {
        fn can_fly(&self) -> bool {
            true
        } 
        fn is_animal(&self) -> bool {
            false
        }
    }

    println!("{} {}", bird.can_fly(), bird.is_animal());

    //Enums

    #[derive(Debug)]
    enum MyEnum {
        A,
        B(i32),
        C{x:i32, y:i32}
    }

    let a:MyEnum = MyEnum::A;
    let b:MyEnum = MyEnum::B(5);
    let c:MyEnum = MyEnum::C { x: (4), y: (8) };

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    if let MyEnum::B(val) = b{
        println!("{}", val)
    }

    if let MyEnum::C{x, y} = c{
        println!("{} {}", x, y)
    }

    //Vectors

    let mut vec: Vec<i64> = vec![1,2,3,4,5];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove((0));
    println!("{:?}", vec);

    //Map

    let mut map = HashMap::new();

    map.insert(0, "Hi");
    map.insert(1, "Hi2");
    println!("{:?}", map);

    match  map.get(&0) {
        Some(str1) => println!("{}", str1),
        None => println!("Does Not Exist")
    }
    match  map.get(&2) {
        Some(str) => println!("{}", str),
        _ => println!("Does Not Exist")
    }

    map.remove(&0);
    println!("{:?}", map);

    #[derive(Debug)]
    enum MyError {
        Error1
    }

    //Options
    fn divide(dividend:i32, divisor:i32) -> Result<i32, MyError>{
        if dividend%divisor != 0{
            Err(MyError::Error1)
        } else {
            Ok(dividend / divisor)
        }
    }

    // let divide1 = divide(4,2);
    // let divide2 = divide(2,3);

    //unwrapping a Some variant will extract the value wrapped
    // println!("{:?} unwraps to {}", divide1, divide1.unwrap());

    //unwrapping a None variant will extract the value wrapped
    // println!("{:?} unwraps to {}", divide2, divide2.unwrap());

}

//Functions in Rust
pub fn is_even(num:u8) -> bool{

    let digit: u8 = num % 2;
    digit == 0 //return bool

}

fn borrowing_slice(arr:[u8; 3], slice:&[u8]){
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}