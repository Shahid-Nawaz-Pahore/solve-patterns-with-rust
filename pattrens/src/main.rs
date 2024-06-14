fn main() {
    println!("******Display all possible pattrens uisng Rust Language*****");
    println!("<----- Pattren 1 ----->");
    pattren1();
    println!("<----- Pattren 2 ----->");
    pattren2();
    println!("<----- Pattren 4 ----->");
    pattren3();
    println!("<----- Pattren 5 ----->");
    pattren4();
    println!("<----- Pattren 6 ----->");
    pattren5();
    println!("<----- Pattren 7 ----->");
    pattren6();
    println!("<----- Pattren 8 ----->");
    pattren7();
    println!("<----- Pattren 9 ----->");
    pattren8();
}
fn pattren1(){
    for i in 0..5 {
        for j in 0..5 {
            print!("* ")
        }
        println!()
    }
}

fn pattren2(){
    for i in 0..5 {
        for j in 0..i+1 {
            print!("* ")
        }
        println!()
    }
}

fn pattren3(){
    for i in 0..5 {
        for j in 0..5-i {
            print!("* ")
        }
        println!()
    }
}

fn pattren4(){
    for i in 0..5 {
        for j in 0..5-i {
            print!("{} ",j)
        }
        println!()
    }
}

fn pattren5(){
    for i in 0..5 {
        for j in 0..5-i {
            print!("{} ",j)
        }
        println!()
    }
}

fn pattren6(){
    for i in 0..10 {
        for j in 0..10-i-1 {
            print!(" ")
        }
        for j in 0..2*i+1 {
            print!("*")
        }
        for j in 0..10-i-1 {
            print!(" ")
        }
        println!()
    }
}

fn pattren7(){
    for i in 0..10 {
        for j in 0..i {
            print!(" ")
        }
        for j in 0..20-2*i-1 {
            print!("*")
        }
        for j in 0..i {
            print!(" ")
        }
        println!()
    }
}

fn pattren8(){
    for i in 0..9 {
        for j in 0..10-i-1 {
            print!(" ")
        }
        for j in 0..2*i+1 {
            print!("*")
        }
        for j in 0..10-i-1 {
            print!(" ")
        }
        println!()
    }
    for i in 0..10 {
        for j in 0..i {
            print!(" ")
        }
        for j in 0..20-2*i-1 {
            print!("*")
        }
        for j in 0..i {
            print!(" ")
        }
        println!()
    }
}