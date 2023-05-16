use enum_dispatch::enum_dispatch;

#[enum_dispatch(Request, InnerSign)]
trait Execute {
    fn execute(&self);
}

#[enum_dispatch]
enum Request {
    EvmSign,
    BtcSign,
    InnerSign,
}

#[enum_dispatch]
pub enum InnerSign {
    Rename,
    Create,
    Delete,
}

impl EvmSign {
    pub fn new(request: u32) -> Self {
        Self { request }
    }
}

struct EvmSign {
    request: u32,
}

struct BtcSign {
    request: String,
}

pub struct Rename {
    id: u32,
    name: String,
}

pub struct Create {
    id: u32,
    name: String,
}

pub struct Delete {
    id: u32,
    name: String,
}

impl Execute for EvmSign {
    fn execute(&self) {
        println!("MyImplementorA");
    }
}

impl Execute for BtcSign {
    fn execute(&self) {
        println!("MyImplementorB");
    }
}

impl Execute for Rename {
    fn execute(&self) {
        println!("{}", self.name);
    }
}

impl Execute for Create {
    fn execute(&self) {
        println!("Create");
    }
}

impl Execute for Delete {
    fn execute(&self) {
        println!("Delete");
    }
}

#[test]
fn main() {
    let a: Request = EvmSign::new(12).into();

    a.execute();

    let a: Request = BtcSign {
        request: "42".to_string(),
    }
    .into();

    a.execute();

    let a: Request = InnerSign::Rename(Rename {
        id: 12,
        name: "42".to_string(),
    })
    .into();

    a.execute();
}
