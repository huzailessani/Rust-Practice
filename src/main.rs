fn main() {
    println!("Hello, world!");

    let mut x = 45;
    println!("{}", x);

    let x: i8 = -128;
    println!("{}", x);

    let x = 4;
    println!("{}", x);

    let n = 5;
    if n < 5 {
        println!("n is less than 5")
    } else if n > 5 {
        println!("n is greater than 5")
    } else {
        println!("n is = 5")
    }

    let mut m = 0;
    loop {
        m += 1;
        println!("{}", m);

        if m == 9 {
            continue;
        }

        if m == 10 {
            break;
        }
    }

    let mut z = 1;
    while z <= 40 {
        // z += 1;
        // if z > 10 {
        //     break;
        // }
        // println!("{}", z)

        if z % 3 == 0 {
            println!("z is {}", z)
        }
        z += 1;
    }

    let mut animal = vec!["rabbit", "dog", "leopard"];

    animal.push("gorilla");
    animal.remove(0);

    let mut number = 1..11;

    for (index, i) in animal.iter().enumerate() {
        println!("The animal number is {} The animal name is {}", index, i)
    }
    animal.push("bat");
    animal.remove(2);
    println!("{:?}", animal);

    {
        enum Direction {
            Up,
            Down,
            Left,
            Right,
        }
        {
            let playerdirection: Direction = Direction::Up;

            match playerdirection {
                Direction::Up => println!("we are heading up"),
                Direction::Down => println!("we are heading up"),
                Direction::Left => println!("we are heading up"),
                Direction::Right => println!("we are heading up"),
            }
        }
    }
    let tuple = (2, 3, 5, "huzail", true, (4, "hello world", 6));
    let (a, b, c, d, e, f) = tuple;
    println!("{}", &a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{:?}", tuple);
    println!("{}", (tuple.5).1);

    for i in 1..20 {
        if is_even(i) {
            println!("It is even {}", i)
        } else {
            println!("It is odd {}", i)
        }
    }

    fn is_even(num: u8) -> bool {
        return num % 2 == 0;
    }
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    let mut bg = Color {
        red: 255,
        blue: 8,
        green: 3,
    };
    bg.blue = 7;

    println!("{} {} {} ", bg.blue, bg.green, bg.red);

    struct Clr(u8, u8, u8);
    let mut red = Clr(3, 4, 5);
    println!("{} {} {} ", red.0, red.1, red.2);

    let number = [2; 10];
    let Number = vec![1, 2, 3, 4, 5];
    for n in number.iter() {
        println!("{:?}", n);
    }

    let mut mystr = String::from("Hello ye kese ho");
    println!("{} ", mystr.is_empty());
    println!("{} ", mystr.len());
    println!("{} ", mystr);
    println!("{} ", mystr.contains("Hello"));

    mystr.push_str(" hello");
    println!("{}", mystr);

    for z in mystr.split_ascii_whitespace() {
        println!("{}", z)
    }
    use std::io;
    let mut input = String::new();
    println!("please say something");

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("succes {}", input);
        }
        Err(e) => {
            println!("failure");
        }
    }
    let name = String::from("Hello yr kese ho");
    println!(
        "{}",
        match name.chars().nth(3) {
            Some(c) => c.to_string(),
            None => "no char".to_string(),
        }
    );
    enum Day {
        Mon,
        Tue,
        Wed,
        Thurs,
        Fri,
        Sat,
        Sun,
    }

    impl Day {
        fn is_Weekday(&self) -> bool {
            match self {
                &Day::Sat | &Day::Sun => return false,
                _ => return true,
            }
        }
    }
    let d = Day::Thurs;
    println!("is day a weekday {}", d.is_Weekday());

    struct Person {
        Huzail: i8,
        Usairim: i8,
    }
    let mut Color = Person {
        Huzail: 20,
        Usairim: 30,
    };

    println!("{}", Color.Huzail);

    // fn is_num_even(20);

    let mut huzail = vec!["huzail", "usairim", "altaf", "shahnila"];
    for h in huzail.iter() {
        println!("{} ", h)
    }
    huzail.push("value");
    println!("{:?}", huzail);

    for a in 0..30 {
        if is_even_or(a) {
            println!("{}", a);
        } else {
            (" no {}", a);
        }
    }

    let mut nam = 0..40;
    fn is_even_or(nam: i8) -> bool {
        return nam % 5 == 0;
    }

    enum Month {
        Jan,
        Feb,
        March,
        April,
        May,
        June,
        July,
    }

    impl Month {
        fn is_holiday(&self) -> bool {
            match self {
                &Month::July | &Month::June => return false,
                _ => true,
            }
        }
    }
    let m = Month::June;
    println!("{}", m.is_holiday());

enum Huzail {
Isani,
Essani,
}

let Sirname : Huzail = Huzail::Isani;

match Sirname {
    Huzail::Essani => println!("this is huzail isani"),
    Huzail::Isani => println!("this is huzail essani"),
    }


let mut output = String::new();
println!("please say something");
match io::stdin().read_line(&mut output){
    Ok(_)=>{
        println!("success you said {}" , output);
    }
    Err(e)=>{ println!("failure")},
}

enum Holiday {
    Holidey , 
    Normalday,
    Labourday,

}
impl Holiday {
    fn Holidays(&self)->bool{
        match self {
            &Holiday::Holidey | &Holiday::Labourday => return false,
            _=> true,
        }
       
    }
}
let h = Holiday::Holidey;
println!("{}" , h.Holidays())
    
}





