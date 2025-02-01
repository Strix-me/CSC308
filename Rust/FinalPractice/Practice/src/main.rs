fn main() {
    //println!("Hello, world!");
    // Rust is Zero Indexed
    // let age = 15;



    // match age {
    //18 | 19 => println!("You are an adult"),
    // 10..=15 => println!("You are 10!"),
    //   _=> println!("Invalid Option!")
    //}

    // let account_balance: Option<i32> = None;

    // match account_balance{
    //     Some(value) => println!("Value was retrieved!: {} ", value),
    //     None => println!("Nothing was recieved")
    // }


    macro_rules! testing {
        ($arg:expr) => {

            println!("Testing a macro!: {}", $arg);
        };
    }

     testing!("Wassio");
}
