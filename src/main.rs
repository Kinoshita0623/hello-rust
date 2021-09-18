use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

struct Rect<T> {
    width: T,
    height: T,
}

struct User {
    name: String,
    description: Option<String>,
}



trait  Printable {
    fn print(&self);
}

impl<T> Printable for Rect<T> where T: std::fmt::Display {
    fn print(self: &Rect<T>) {
        println!("{}:{}", self.width, self.height);
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        return Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}


impl Printable for User {
    fn print(self: &User) {
        let des = if self.description.is_some() {
            self.description.clone().unwrap()
        }else{
            "piyo".to_string()
        };
        println!("name: {}, description:{}", self.name, des);
    }
}

fn main() {
    println!("Hello, world!");
    let x: i32 = 55;
    println!("{}", x.to_string());

    let text = String::from("hogehogepiyopiyo");
    let text2 = text.clone();
    let l = length(text);
    let text_length = "文字列の長さは".to_string() + &l.to_string();
    println!("{}",text_length);
    println!("{}", text2);

    let point = Point{x: 10, y: 20};
    println!("Point x:{}, y:{}", point.x, point.y);

    for i in 0..10 {
        println!("in loop :{}", i);
    }

    let r1: Rect<i32> = Rect{ width: 100, height: 100 };
    let r2: Rect<i64> = Rect{ width: 100, height: 200 };
    r1.print();
    r2.print();


    let u1 = User{ name: String::from("パンタ"), description: Some(String::from("kawaii")) };
    let u2 = User{ name: String::from("パンタ"), description: None };

    
    u1.print();
    u2.print();

    println!("User.description type is {}", type_of(u1.description));

    let array1: &mut [i32; 5] = &mut [2, 1, 3, 5, 4];

    println!("array1={:?}", array1);
    let min_size = find_min_index(array1);
    println!("min size:{}", array1[min_size]);
    //select_sort(array1);
    insert_sort(array1);
    println!("array1={:?}", array1);
    
    let str1: &str = "hogehoge";
    println!("str1 {}", str1);

    let str2: &str = "hogepiyo";
    println!("str2 {}", str2);

    let str3 = str1.to_string() + str2;
    println!("str3 {}", str3);

    let p1 = Point {
        x: 1, y: 2
    };
    let p2 = Point {
        x: 3, y: 4
    };
    let p3 = p2 + p1;
    println!("p3: x={}, y={}", p3.x, p3.y);

}

fn length(str: String) -> usize {
    return str.len();
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn buble_sort<T: PartialOrd>(array: &mut [T]) {
    for i in 0..array.len() - 1 {
        for j in (i + 1..array.len()).rev() {
            if array[j - 1] > array[j] {
                array.swap(j, j - 1);
            }
        }
    }
}

pub fn select_sort<T: PartialOrd + std::fmt::Debug>(array: &mut [T]) {
    let mut min;
    for i in 0..array.len() - 1 {
        min = i;
        for j in i..array.len() {
            if array[min] > array[j] {
                min = j;
            }
        }
        array.swap(i, min);
        //println!("sorting:{:?}, min:{:?}", array, min);
    }   
}

fn find_min_index<T: PartialOrd>(array: &mut [T]) -> usize {
    
    let mut min = 0;
    for i in 0..array.len() {
        if array[min] > array[i] {
            min = i;
        }
    }
    return min;
}

fn insert_sort<T: PartialOrd + std::fmt::Debug>(array: &mut [T]) {
    for i in 1..array.len() {
        //println!("{:?}", array[i]);
        // + 1は次の確定分
        for j in (1..i + 1).rev() {
            // array[i]は挿入しようとしている要素
            if array[j - 1] > array[i] {
                array.swap(j - 1, i);
            } else {
                break;
            }
            
        }
    }
}

