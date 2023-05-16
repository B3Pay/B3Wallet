use enum_dispatch::enum_dispatch;

struct EvmSignRequest<T: Request> {
    _request: T,
}

impl<T: Request> Execute<T> for EvmSignRequest<T> {
    fn execute(&self, _value: T) {
        println!("MyImplementorA");
    }
}
impl<T: Request> EvmSignRequest<T> {
    pub fn new(value: T) -> Self {
        Self { _request: value }
    }
}

struct BtcSignRequest<T: Request> {
    _request: T,
}

impl<T: Request> Execute<T> for BtcSignRequest<T> {
    fn execute(&self, _value: T) {
        println!("MyImplementorA");
    }
}
impl<T: Request> BtcSignRequest<T> {
    pub fn new(value: T) -> Self {
        Self { _request: value }
    }
}

#[enum_dispatch]
trait Execute<T: Request> {
    fn execute(&self, value: T);
}

// #[enum_dispatch(Execute)]
enum Outer<T: Request> {
    EvmSignRequest(EvmSignRequest<T>),
    BtcSignRequest(BtcSignRequest<T>),
    // ICP(Inner),
}

trait Request {}
impl Request for u32 {}

#[test]
fn main() {
    let anyfoo: Outer<u32> = Outer::EvmSignRequest(EvmSignRequest::new(1));

    // anyfoo.execute(1);
}
