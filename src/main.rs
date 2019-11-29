use std::ops::Drop;
use sec7::ToyVec;

#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);

impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug, Clone, Copy)]
struct Parent2(usize, Child2, Child2);

#[derive(Debug, Clone, Copy)]
struct Child2(usize);

fn main() {
    println!("Hello, world!");
    {
        let p1 = Parent(1, Child(11), Child(12));
        {
            let p2 = Parent(2, Child(21), Child(22));
            println!("(a) p1: {:?}, p2: {:?}", p1, p2);
        }
        println!("(b) p1: {:?}", p1);
        let p3 = Parent(3, Child(31), Child(32));
        println!("(c) p1: {:?}, p3: {:?}", p1, p3);
        {
            let p4 = p3;
            println!("p4: {:?}", p4);
            //println!("p3: {:?}",p3);
        }
        f1(&p1);
        println!("(c) p1: {:?}", p1);
    }
    {
        let p1 = Parent2(1, Child2(11), Child2(12));
        let p2 = p1;
        println!("{:?}", p2);
        println!("{:?}", p1);
    }
    {
        let mut v = ToyVec::new();
        v.push("Java".to_string());
        v.push("Rust".to_string());
        if let Some(s) = v.get(1) {
            println!("{}", s);
        }
        let mut iter = v.iter();
        if let Some(s) = iter.next() {
            println!("{}",s);
        }
    }
}

fn f1(p: &Parent) {
    println!("p: {:?}", p);
}
