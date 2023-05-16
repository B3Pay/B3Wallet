use enum_dispatch::enum_dispatch;

#[derive(Debug, Clone)]
struct Btc<T: Request> {
    _bar: T,
}

impl<T: Request> Executable<T> for Btc<T> {
    fn execute<F>(&mut self, callback: F)
    where
        F: FnOnce(&T) -> Result<T, String>,
    {
        println!("BTC");
        callback(&self._bar);
    }
}

#[derive(Debug, Clone)]
struct Evm<T: Request> {
    _bar: T,
}

impl<T: Request> Executable<T> for Evm<T> {
    fn execute<F>(&mut self, callback: F)
    where
        F: FnOnce(&T) -> Result<T, String>,
    {
        println!("EVM");
        callback(&self._bar);
    }
}

trait Request {}
trait EvmRequest {}
trait BtcRequest {}

impl Request for u32 {}
impl Request for String {}
impl Request for Rename {}
impl Request for Create {}
impl Request for Delete {}

#[derive(Debug, Clone)]
struct Inner<T: Request> {
    request: T,
}

#[derive(Debug, Clone)]
struct Rename {
    id: u32,
    name: String,
}
#[derive(Debug, Clone)]
struct Create {
    id: u32,
    name: String,
}
#[derive(Debug, Clone)]
struct Delete {
    id: u32,
}

#[enum_dispatch]
trait Executable<T: Request> {
    fn execute<F>(&mut self, callback: F)
    where
        F: FnOnce(&T) -> Result<T, String>;
}

#[enum_dispatch(Executable)]
#[derive(Debug, Clone)]
enum MainRequest<T: Request> {
    Btc(Btc<T>),
    Evm(Evm<T>),
    Inner(Inner<T>),
}

impl<T: Request> Executable<T> for Inner<T> {
    fn execute<F>(&mut self, callback: F)
    where
        F: FnOnce(&T) -> Result<T, String>,
    {
        println!("INNER");
        callback(&self.request);
    }
}

impl<T: Request> Executable<T> for MainRequest<T> {
    fn execute<F>(&mut self, callback: F)
    where
        F: FnOnce(&T) -> Result<T, String>,
    {
        match self {
            MainRequest::Btc(inner) => inner.execute(callback),
            MainRequest::Evm(inner) => inner.execute(callback),
            MainRequest::Inner(inner) => inner.execute(callback),
        }
    }
}

#[test]
fn main() {
    let mut anyfoo: MainRequest<String> = MainRequest::Evm(Evm {
        _bar: "42".to_string(),
    });

    anyfoo.execute(|bar| {
        println!("EVM: {}", bar);
        Ok(bar.to_owned())
    });

    let mut anyfoo: MainRequest<u32> = MainRequest::Btc(Btc { _bar: 42 });

    anyfoo.execute(|bar| {
        println!("BTC: {}", bar);
        Ok(bar.to_owned())
    });

    let mut anyfoo: MainRequest<String> = MainRequest::Inner(Inner {
        request: "42".to_string(),
    });

    anyfoo.execute(|bar| {
        println!("INNER: {}", bar);
        Ok(bar.to_owned())
    });

    let mut anyfoo: MainRequest<Create> = MainRequest::Inner(Inner {
        request: Create {
            id: 42,
            name: "42".to_string(),
        },
    });

    anyfoo.execute(|bar| {
        println!("INNER: {:?}", bar);
        Ok(bar.to_owned())
    });
}
