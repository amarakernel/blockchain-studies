use std::io;

enum Operation{
    Add {x: f64, y: f64}, //ekle 
    Substract {x: f64, y: f64}, //çıkar
    Multiply {x: f64, y: f64}, //çarp
    Divide {x: f64, y: f64}, //böl
}

fn calculate(opt: Operation){
    match opt {
        Operation::Add {x,y} => { println!("Calculated: {}", x+y);},
        Operation::Substract {x,y} => {println!("Calculated: {}", x-y);},
        Operation::Multiply {x,y} => {println!("Calculated: {}", x*y);},
        Operation::Divide {x,y} => {println!("Calculated: {}", x/y);},
    }
}

fn main(){

    
    let mut opti = String::new();
    let mut first_number = String::new();
    let mut sec_number = String::new();

    println!("Enter the option.(Lowercase please.");

    io::stdin()
        .read_line(&mut opti)
        .expect("Please enter the acceptable option.");


        let opti = opti.trim();

        
    println!("Enter the first number.");

    io::stdin()
        .read_line(&mut first_number)
        .expect("Please try again.");
        
        let f_num: f64 = first_number.trim().parse().expect("Please try again.");

    println!("Enter the second number.");

    io::stdin()
        .read_line(&mut sec_number)
        .expect("Please try again.");

        let s_num: f64 = sec_number.trim().parse().expect("Please try again.");


    

    if opti == "add" {
      let opt = Operation::Add{x: f_num,y: s_num};
      calculate(opt);
    }

    else if opti == "substract"{
      let opt = Operation::Substract{x: f_num,y: s_num};
      calculate(opt);
    }

    else if opti == "multiply"{
      let opt = Operation::Multiply{x: f_num,y: s_num};
      calculate(opt);
    }

    else if opti == "divide"{
      let opt = Operation::Divide { x: f_num, y: s_num};
      calculate(opt);
    }
    else {
        println!("Please try again.")
    }

      
  
}
