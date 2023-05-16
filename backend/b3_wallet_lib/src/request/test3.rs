use enum_dispatch::enum_dispatch;

pub struct EvmSignRequest<T: Request> {
    _bar: T,
}

impl<T: Request> Execute<T> for EvmSignRequest<T> {
    fn do_something(&mut self, _val: T) {
        println!("EvmSignRequest");
    }
}

pub struct BtcSignRequest<T: Request> {
    _bar: T,
}

impl<T: Request> Execute<T> for BtcSignRequest<T> {
    fn do_something(&mut self, _val: T) {
        println!("BtcSignRequest");
    }
}

pub trait Request {}

#[enum_dispatch]
pub trait Execute<T: Request> {
    fn do_something(&mut self, val: T);
}

#[enum_dispatch(Execute)]
pub enum Outer<T: Request> {
    EvmSignRequest(EvmSignRequest<T>),
    BtcSignRequest(BtcSignRequest<T>),
}

impl<T: Request> Outer<T> {
    pub fn do_something(&mut self, val: T) {
        match self {
            Outer::EvmSignRequest(inner) => inner.do_something(val),
            Outer::BtcSignRequest(inner) => inner.do_something(val),
        }
    }
}

impl Request for u32 {}

#[test]
fn main() {
    let mut anyfoo: Outer<u32> = Outer::EvmSignRequest(EvmSignRequest { _bar: 1 });

    anyfoo.do_something(1);

    let mut anyfoo: Outer<u32> = Outer::BtcSignRequest(BtcSignRequest { _bar: 1 });

    anyfoo.do_something(1);
}
