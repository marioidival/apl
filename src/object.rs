use crate::ast::Number;
use crate::error::{Error, OperatorError};
use crate::primitive::Primitive;

type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Primitive(Primitive),
    Builtin(Builtin),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Builtin {
    Print(fn(Object) -> Result<Object>),
    Input(fn() -> Result<Object>),
}

impl From<i64> for Object {
    fn from(n: i64) -> Self {
        Object::Primitive(Primitive::Integer(n))
    }
}

impl From<f64> for Object {
    fn from(n: f64) -> Self {
        Object::Primitive(Primitive::Float(n))
    }
}

impl From<String> for Object {
    fn from(n: String) -> Self {
        Object::Primitive(Primitive::Str(n))
    }
}

impl From<bool> for Object {
    fn from(n: bool) -> Self {
        Object::Primitive(Primitive::Boolean(n))
    }
}

impl From<Number> for Object {
    fn from(s: Number) -> Self {
        match s {
            Number::Integer { value } => Object::Primitive(Primitive::Integer(value)),
            Number::Float { value } => Object::Primitive(Primitive::Float(value)),
        }
    }
}

impl From<Primitive> for Object {
    fn from(s: Primitive) -> Self {
        Object::Primitive(s)
    }
}

impl Object {
    pub fn negate(&self) -> Result<Self> {
        if let Object::Primitive(p) = self {
            return Ok(p.negate()?.into());
        }
        self.error(None, OperatorError::Negate)
    }

    pub fn unary_plus(&self) -> Result<Self> {
        if let Object::Primitive(p) = self {
            return Ok(p.plus()?.into());
        }
        self.error(None, OperatorError::UnarySub)
    }

    pub fn unary_minus(&self) -> Result<Self> {
        if let Object::Primitive(p) = self {
            return Ok(p.minus()?.into());
        }
        self.error(None, OperatorError::UnarySub)
    }

    pub fn add(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.add(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::Add),
        }
    }

    pub fn subtract(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.sub(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::Sub),
        }
    }

    pub fn multiply(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.mul(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::Mul),
        }
    }

    pub fn int_divide(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.int_div(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::IntDiv),
        }
    }

    pub fn real_divide(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.real_div(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::RealDiv),
        }
    }

    pub fn module(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.module(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::Module),
        }
    }

    pub fn and(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.and(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::And),
        }
    }

    pub fn or(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.or(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::Or),
        }
    }

    pub fn is(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.is(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::Is),
        }
    }

    pub fn less_than(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.less_than(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::LessThan),
        }
    }

    pub fn greater_than(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.greater_than(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::GreaterThan),
        }
    }

    pub fn less_than_equal(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.less_than_equal(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::LessThanEqual),
        }
    }

    pub fn greater_than_equal(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.greater_than_equal(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::GreaterThanEqual),
        }
    }

    pub fn equal(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.eq(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::Equal),
        }
    }

    pub fn not_equal(&self, other: &Self) -> Result<Self> {
        match (self, other) {
            (Object::Primitive(l), Object::Primitive(r)) => Ok(l.not_eq(r)?.into()),
            (_, r) => self.error(Some(r), OperatorError::NotEqual),
        }
    }

    fn error<T>(&self, right: Option<&Self>, op: OperatorError) -> Result<T> {
        Err(Error::InvalidType(op, self.clone(), right.cloned()))
    }
}

mod tests {
    use super::*;

    #[test]
    fn negate() {
        let truth = Object::Primitive(Primitive::Boolean(true));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            truth.negate().unwrap()
        )
    }

    #[test]
    fn and() {
        let truth = Object::Primitive(Primitive::Boolean(true));
        let falsy = Object::Primitive(Primitive::Boolean(false));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            truth.and(&falsy).unwrap()
        )
    }

    #[test]
    fn or() {
        let truth = Object::Primitive(Primitive::Boolean(true));
        let falsy = Object::Primitive(Primitive::Boolean(false));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            truth.or(&falsy).unwrap()
        )
    }

    #[test]
    fn eq_boolean() {
        let truth = Object::Primitive(Primitive::Boolean(true));
        let falsy = Object::Primitive(Primitive::Boolean(false));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            truth.equal(&falsy).unwrap()
        )
    }

    #[test]
    fn eq_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.equal(&b).unwrap()
        )
    }

    #[test]
    fn eq_float() {
        let a = Object::Primitive(Primitive::Float(1.2));
        let b = Object::Primitive(Primitive::Float(1.2));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.equal(&b).unwrap()
        )
    }

    #[test]
    fn eq_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.equal(&b).unwrap()
        )
    }

    #[test]
    fn eq_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.equal(&b).unwrap()
        )
    }

    #[test]
    fn noteq_boolean() {
        let truth = Object::Primitive(Primitive::Boolean(true));
        let falsy = Object::Primitive(Primitive::Boolean(false));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            truth.not_equal(&falsy).unwrap()
        )
    }

    #[test]
    fn noteq_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.not_equal(&b).unwrap()
        )
    }

    #[test]
    fn noteq_float() {
        let a = Object::Primitive(Primitive::Float(1.2));
        let b = Object::Primitive(Primitive::Float(1.2));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.not_equal(&b).unwrap()
        )
    }

    #[test]
    fn noteq_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.not_equal(&b).unwrap()
        )
    }

    #[test]
    fn noteq_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));

        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.not_equal(&b).unwrap()
        )
    }

    #[test]
    fn less_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.less_than(&b).unwrap()
        )
    }

    #[test]
    fn less_float() {
        let a = Object::Primitive(Primitive::Float(1.2));
        let b = Object::Primitive(Primitive::Float(1.2));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.less_than(&b).unwrap()
        )
    }

    #[test]
    fn less_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.less_than(&b).unwrap()
        )
    }

    #[test]
    fn less_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.less_than(&b).unwrap()
        )
    }

    #[test]
    fn less_eq_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.less_than_equal(&b).unwrap()
        )
    }

    #[test]
    fn less_eq_float() {
        let a = Object::Primitive(Primitive::Float(1.2));
        let b = Object::Primitive(Primitive::Float(1.2));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.less_than_equal(&b).unwrap()
        )
    }

    #[test]
    fn less_eq_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.less_than_equal(&b).unwrap()
        )
    }

    #[test]
    fn less_eq_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.less_than_equal(&b).unwrap()
        )
    }

    #[test]
    fn greater_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.greater_than(&b).unwrap()
        )
    }

    #[test]
    fn greater_float() {
        let a = Object::Primitive(Primitive::Float(1.2));
        let b = Object::Primitive(Primitive::Float(1.2));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.greater_than(&b).unwrap()
        )
    }

    #[test]
    fn greater_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.greater_than(&b).unwrap()
        )
    }

    #[test]
    fn greater_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.greater_than(&b).unwrap()
        )
    }

    #[test]
    fn greater_eq_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.greater_than_equal(&b).unwrap()
        )
    }

    #[test]
    fn greater_eq_float() {
        let a = Object::Primitive(Primitive::Float(1.2));
        let b = Object::Primitive(Primitive::Float(1.2));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.greater_than_equal(&b).unwrap()
        )
    }

    #[test]
    fn greater_eq_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.greater_than_equal(&b).unwrap()
        )
    }

    #[test]
    fn greater_eq_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.greater_than_equal(&b).unwrap()
        )
    }

    #[test]
    fn sum_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(Object::Primitive(Primitive::Integer(2)), a.add(&b).unwrap())
    }

    #[test]
    fn sum_float() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(Object::Primitive(Primitive::Float(2.0)), a.add(&b).unwrap())
    }

    #[test]
    fn sum_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(Object::Primitive(Primitive::Float(2.0)), a.add(&b).unwrap())
    }

    #[test]
    fn sum_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(Object::Primitive(Primitive::Float(2.0)), a.add(&b).unwrap())
    }

    #[test]
    fn sub_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Integer(0)),
            a.subtract(&b).unwrap()
        )
    }

    #[test]
    fn sub_float() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Float(0.0)),
            a.subtract(&b).unwrap()
        )
    }

    #[test]
    fn sub_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Float(0.0)),
            a.subtract(&b).unwrap()
        )
    }

    #[test]
    fn sub_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Float(0.0)),
            a.subtract(&b).unwrap()
        )
    }

    #[test]
    fn mul_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Integer(1)),
            a.multiply(&b).unwrap()
        )
    }

    #[test]
    fn mul_float() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Float(1.0)),
            a.multiply(&b).unwrap()
        )
    }

    #[test]
    fn mul_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Float(1.0)),
            a.multiply(&b).unwrap()
        )
    }

    #[test]
    fn mul_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Float(1.0)),
            a.multiply(&b).unwrap()
        )
    }

    #[test]
    fn div_real_int() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Float(1.0)),
            a.real_divide(&b).unwrap()
        )
    }

    #[test]
    fn div_real_float() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Float(1.0)),
            a.real_divide(&b).unwrap()
        )
    }

    #[test]
    fn div_real_int_float() {
        let a = Object::Primitive(Primitive::Integer(1));
        let b = Object::Primitive(Primitive::Float(1.0));
        assert_eq!(
            Object::Primitive(Primitive::Float(1.0)),
            a.real_divide(&b).unwrap()
        )
    }

    #[test]
    fn div_real_float_int() {
        let a = Object::Primitive(Primitive::Float(1.0));
        let b = Object::Primitive(Primitive::Integer(1));
        assert_eq!(
            Object::Primitive(Primitive::Float(1.0)),
            a.real_divide(&b).unwrap()
        )
    }

    #[test]
    fn div_int_int() {
        let a = Object::Primitive(Primitive::Integer(10));
        let b = Object::Primitive(Primitive::Integer(2));
        assert_eq!(
            Object::Primitive(Primitive::Integer(5)),
            a.int_divide(&b).unwrap()
        )
    }

    #[test]
    fn is() {
        let a = Object::Primitive(Primitive::Integer(10));
        let b = Object::Primitive(Primitive::Integer(2));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(true)),
            a.is(&b).unwrap()
        )
    }

    #[test]
    fn is_false_result() {
        let a = Object::Primitive(Primitive::Integer(10));
        let b = Object::Primitive(Primitive::Float(2.0));
        assert_eq!(
            Object::Primitive(Primitive::Boolean(false)),
            a.is(&b).unwrap()
        )
    }

    #[test]
    fn unary_minus() {
        let a = Object::Primitive(Primitive::Integer(10));
        assert_eq!(
            Object::Primitive(Primitive::Integer(-10)),
            a.unary_minus().unwrap()
        )
    }

    #[test]
    fn unary_plus() {
        let a = Object::Primitive(Primitive::Integer(-10));
        assert_eq!(
            Object::Primitive(Primitive::Integer(10)),
            a.unary_plus().unwrap()
        )
    }

    #[test]
    fn module() {
        let a = Object::Primitive(Primitive::Integer(10));
        let b = Object::Primitive(Primitive::Integer(2));
        assert_eq!(
            Object::Primitive(Primitive::Integer(0)),
            a.module(&b).unwrap()
        )
    }
}
