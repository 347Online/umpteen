use std::{
    fmt::Display,
    sync::atomic::{AtomicUsize, Ordering},
};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueData<'d> {
    Empty,
    Boolean(bool),
    Number(f64),
    String(&'d String),
}

#[derive(Debug, Clone, Copy)]
pub struct Value<'v> {
    id: usize,
    data: ValueData<'v>,
}

impl<'v> Value<'v> {
    pub fn new(data: ValueData<'v>) -> Self {
        Value {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            data,
        }
    }

    pub fn is(&self, other: Self) -> bool {
        self.id == other.id
    }

    pub fn number<N: Into<f64>>(num: N) -> Self {
        Value::new(ValueData::Number(num.into()))
    }

    pub fn string(st: &'v String) -> Self {
        Value::new(ValueData::String(st))
    }
}

impl<'v> PartialEq for Value<'v> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<'v> Display for Value<'v> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tmp;

        let repr = match self.data {
            ValueData::Empty => "<Empty>",
            ValueData::Boolean(x) => {
                tmp = x.to_string();
                &tmp
            }
            ValueData::Number(x) => {
                tmp = x.to_string();
                &tmp
            }
            ValueData::String(x) => x,
        };

        write!(f, "{repr}")
    }
}

#[test]
fn test_value_cmp() {
    let num = 100.0;
    let v1 = Value::number(num);
    let v2 = Value::number(num);

    assert_eq!(v1, v2);
    assert!(!v2.is(v1));
    assert!(v1.is(v1));

    let num = f64::NAN;
    let v1 = Value::number(num);
    let v2 = Value::number(num);

    assert_ne!(v1, v2);
    assert!(!v2.is(v1));
    assert!(v1.is(v1));
}
