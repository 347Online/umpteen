use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ValueData<'d> {
    Empty,
    Boolean(bool),
    Number(&'d f64),
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

    pub fn number(num: &'v f64) -> Self {
        Value::new(ValueData::Number(num))
    }

    pub fn is(&self, other: Self) -> bool {
        self.id == other.id
    }
}

impl<'v> PartialEq for Value<'v> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[test]
fn test_value_cmp() {
    let num = 100.0;
    let v1 = Value::number(&num);
    let v2 = Value::number(&num);

    assert_eq!(v1, v2);
    assert!(!v2.is(v1));
    assert!(v1.is(v1));

    let num = f64::NAN;
    let v1 = Value::number(&num);
    let v2 = Value::number(&num);

    assert_ne!(v1, v2);
    assert!(!v2.is(v1));
    assert!(v1.is(v1));
}