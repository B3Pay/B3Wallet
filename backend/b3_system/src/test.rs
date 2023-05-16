trait Argument {
    type Child;

    fn print(&self);
    fn execute<F>(&self, callback: F)
    where
        F: FnOnce(&Self::Child);
}

#[derive(Debug, Clone)]
pub enum Inner {
    A(u32),
    B(String),
    C(bool),
}

impl Argument for Inner {
    type Child = Inner;

    fn print(&self) {
        match self {
            Inner::A(a) => println!("Printing A: {}", a),
            Inner::B(b) => println!("Printing B: {}", b),
            Inner::C(c) => println!("Printing C: {}", c),
        }
    }

    fn execute<F>(&self, callback: F)
    where
        F: FnOnce(&Self::Child),
    {
        match self {
            Inner::A(a) => {
                let child = Inner::A(*a);
                callback(&child);
            }
            Inner::B(b) => {
                let child = Inner::B(b.to_string());
                callback(&child);
            }
            Inner::C(c) => {
                let child = Inner::C(*c);
                callback(&child);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Outer {
    A(Inner),
    B(Vec<u8>),
    C(String),
}

impl Argument for Outer {
    type Child = Outer;

    fn print(&self) {
        match self {
            Outer::A(a) => a.print(),
            Outer::B(b) => println!("Printing B: {:?}", b),
            Outer::C(c) => println!("Printing C: {:?}", c),
        }
    }

    fn execute<F>(&self, callback: F)
    where
        F: FnOnce(&Self::Child),
    {
        match self {
            Outer::A(a) => {
                let child = Outer::A(a.clone());
                callback(&child);
            }
            Outer::B(b) => {
                let child = Outer::B(b.clone());
                callback(&child);
            }
            Outer::C(c) => {
                let child = Outer::C(c.clone());
                callback(&child);
            }
        }
    }
}

#[test]
fn main() {
    let inner_a = Inner::A(42);

    inner_a.execute(|arg| println!("Callback: {:?}", arg));

    let inner_b = Inner::B("Hello".to_string());

    inner_b.execute(|arg| match arg {
        Inner::A(a) => println!("Callback: A({})", a),
        Inner::B(b) => println!("Callback: B({})", b),
        Inner::C(c) => println!("Callback: C({})", c),
    });

    let inner_c = Inner::C(true);

    inner_c.execute(|arg| match arg {
        Inner::A(a) => println!("Callback: A({})", a),
        Inner::B(b) => println!("Callback: B({})", b),
        Inner::C(c) => println!("Callback: C({})", c),
    });
}
