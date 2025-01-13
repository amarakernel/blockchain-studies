 //1.temel yapı

 fn main(){
    println!("Merhaba, dünya!");
 }

 //2.variables
 //2 çeşit variables kullanıyoruz
 //default:immutable - değiştirilemez
 //mutable: değiştirilebilir

 let x = 5; 
 let muy y = 10; //değiştirilebilir
 y= 15; //x için yazılamaz

 //veri tipleri
 //tamsayılar: i8,i16,i32,i64,i128,u8
 //ondalık: f32,f64
 // bool,char,&str

 let num: i32 = 42;
 let greeting: &str = "Merhaba!";

 //collections
 //tuple - demet

 let tuple: (i32,f64,&str) = (42,3.14,"rust");
 let (x,y,z) = tuple; //parçalama

 //array - dizi

 let array = [1,2,3,4,5];
 let first = array[0]; //erişim

 //founctions

 fn topla(a:i32,b:i32)->i32{
    a+b
 }

 fn main(){
    let sonuc = topla(5,10);
    printlné("Sonuç: {}", sonuc);
 }

 //control flow
 //if - else
 let x = 10;
 if x>5{
    println!("x 5ten büyük.");
 } else {
    println!("x 5ten küçük.");
 }

 //match

 let num = 3;
 match num {
    1 => println!("bir"),
    2|3 => println!("iki veya üç");
    _ => println!("başka bir sayı");
 }

 //loops
 //for

 let nums = [1, 2, 3];
for num in nums.iter() {
    println!("{}", num);
}

//while

let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}

//loop

loop {
    println!("Sonsuz döngü!");
    break; // Çıkış
}

//sahiplik
//string değişkeni stack'te değil heap'te tutulur ve
//değerin sahibidir. Başka bir değişkene atama yapıldığında
//değer kopyalanmaz adres yolu kopyalanır ve 2 farklı değişken
//heap'te aynı değeri gösterdiğinde bu büyük bir problem olacağı için
//rust'ta kapsam dışına çıkma durumunda değer serbest bırakılır.

let s1 = String::from("Merhaba");
// let s2 = s1; // Hatalı! `s1` artık geçerli değil
let s2 = s1.clone(); // Doğru: Kopyalama

//borçlanma

fn yazdir(s: &String) {
    println!("{}", s);
}

let s1 = String::from("Merhaba");
yazdir(&s1); // `&` ile borçlanma


//CARGO NOTES

cargo new proje_adi

//bağımlılıkları indirmek için
//derlemek için

cargo build

//derlemek ve çalıştırmak için

cargo run


//middle rust cheat-sheet

//bir değişkene aynı anda birden fazla immutable referans
//verebilirsin ama sadece bir tane mutable referans verebilirsin

let mut x = 10;
let y = &x; //immutable ref
let x = &mut x; //hata immutable ve mutable aynı anda kullanılamaz

//sahiplik sayesinde rust geçersiz bellek erişimlerini
//"dangling pointer", engellemiş olur

//lifetimes

//rust, referansların ömürlerini takip eder ve referansların valid olduğu 
//süre boyunca geçerli olduklarını garanti eder
//lifetimes çoğu durumda rust tarafından çıkarımlanır ama karmaşık
//durumlarda explicit olarak belirtilir

fn longes<'a>(s1: &'a str, s2: &'a str)->&'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

//structs

struct Point {
    x: i32,
    y:i32,
}

let p = Point {x:10, y:20};
println!("point: ({}, {}", p.x, p.y);

//tuple structs ve unit structs 

struct Color(i32, i32, i32); // Tuple struct
struct Unit;                // Unit struct

//impl blokları ile metot eklemek

impl Point {
    fn distance(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

//enumerations - enums

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

let dir = Direction::Up;

match dir {
    Direction::Up => println!("Up!"),
    Direction::Down => println!("Down!"),
    _ => println!("Other direction"),
}


//enum ile veri taşıma

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

let msg = Message::Move { x: 10, y: 20 };

match msg {
    Message::Move { x, y } => println!("Move to {}, {}", x, y),
    _ => println!("Other message"),
}

//pattern matching
//if let ve while let

if let Some(value) = some_option {
    println!("Value is: {}", value);
}

//destructing

let Point { x, y } = p;
println!("x = {}, y = {}", x, y);

//match guards

match num {
    x if x > 10 => println!("Greater than 10"),
    _ => println!("Less than or equal to 10"),
}

//collections
//vektörler

let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);

for i in &v {
    println!("{}", i);
}


//hashmap 

use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("Key1", 10);
map.insert("Key2", 20);

if let Some(value) = map.get("Key1") {
    println!("Value: {}", value);
}

//error handling
//result

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

match divide(10, 0) {
    Ok(result) => println!("Result: {}", result),
    Err(err) => println!("Error: {}", err),
}

//? operatörü

fn read_file() -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string("file.txt")?;
    Ok(content)
}

//traits

trait Greet {
    fn greet(&self) -> String;
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

let person = Person { name: String::from("Alice") };
println!("{}", person.greet());


//concurrency - çoklu iş parçacığı
//thread

use std::thread;

let handle = thread::spawn(|| {
    for i in 1..10 {
        println!("Thread: {}", i);
    }
});

handle.join().unwrap();

//mutex ve arc

use std::sync::{Arc, Mutex};
use std::thread;

let data = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let data = Arc::clone(&data);
    let handle = thread::spawn(move || {
        let mut num = data.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *data.lock().unwrap());


//iterator ve closure
//iterator 

let v = vec![1, 2, 3];
let sum: i32 = v.iter().sum();
println!("Sum: {}", sum);

//map ve filter 

let v = vec![1, 2, 3, 4];
let even: Vec<i32> = v.into_iter().filter(|x| x % 2 == 0).collect();
println!("{:?}", even);

//closure tanımı

let add = |a, b| a + b;
println!("{}", add(5, 3));


//modül kullanımı

mod my_module {
    pub fn greet() {
        println!("Hello from module!");
    }
}

fn main() {
    my_module::greet();
}


//unsafe blok kullanımı 
//dereferencing raw pointers

unsafe {
    let raw_p = &10 as *const i32;
    println!("Raw pointer: {}", *raw_p);
}
