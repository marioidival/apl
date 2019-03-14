use crate::error::Error;
use crate::error::OperatorError;
use self::Primitive::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Integer(i64),
    Float(f64),
    Str(String),
    Boolean(bool),
}

impl From<i64> for Primitive {
    fn from(n: i64) -> Self {
        Primitive::Integer(n)
    }
}

impl From<f64> for Primitive {
    fn from(n: f64) -> Self {
        Primitive::Float(n)
    }
}

impl From<String> for Primitive {
    fn from(n: String) -> Self {
        Primitive::Str(n)
    }
}

impl From<bool> for Primitive {
    fn from(n: bool) -> Self {
        Primitive::Boolean(n)
    }
}

impl Primitive {
    fn negate(&self) -> Result<Self, Error> {
        match self {
            Boolean(i) => Ok(Boolean(!*i)),
            left => Self::error(left, None, OperatorError::Negate)
        }
    }

    fn and(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Boolean(left), Boolean(right)) => (*left && *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Add)?
        };
        Ok(res)
    }

    fn or(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Boolean(left), Boolean(right)) => (*left || *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Or)?
        };
        Ok(res)
    }

    fn eq(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left == *right).into(),
            (Boolean(left), Boolean(right)) => (*left == *right).into(),
            (Float(left), Float(right)) => (*left == *right).into(),
            (Float(left), Integer(right)) => (*left == (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) == *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Equal)?
        };
        Ok(res)
    }

    fn not_eq(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left != *right).into(),
            (Boolean(left), Boolean(right)) => (*left != *right).into(),
            (Float(left), Float(right)) => (*left != *right).into(),
            (Float(left), Integer(right)) => (*left != (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) != *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::NotEqual)?
        };
        Ok(res)
    }

    fn less_than(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left < *right).into(),
            (Float(left), Float(right)) => (*left < *right).into(),
            (Float(left), Integer(right)) => (*left < (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) < *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::LessThan)?
        };
        Ok(res)
    }

    fn less_than_equal(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left <= *right).into(),
            (Float(left), Float(right)) => (*left <= *right).into(),
            (Float(left), Integer(right)) => (*left <= (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) <= *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::LessThanEqual)?
        };
        Ok(res)
    }

    fn greater_than(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left > *right).into(),
            (Float(left), Float(right)) => (*left > *right).into(),
            (Float(left), Integer(right)) => (*left > (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) > *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::GreaterThan)?
        };
        Ok(res)
    }

    fn greater_than_equal(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left >= *right).into(),
            (Float(left), Float(right)) => (*left >= *right).into(),
            (Float(left), Integer(right)) => (*left >= (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) >= *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::GreaterThanEqual)?
        };
        Ok(res)
    }

    fn add(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (left + right).into(),
            (Float(left), Float(right)) => (left + right).into(),
            (Float(left), Integer(right)) => (left + (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) + right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Add)?,
        };
        Ok(res)
    }

    fn sub(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (left - right).into(),
            (Float(left), Float(right)) => (left - right).into(),
            (Float(left), Integer(right)) => (left - (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) - right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Sub)?,
        };
        Ok(res)
    }

    fn mul(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (left * right).into(),
            (Float(left), Float(right)) => (left * right).into(),
            (Float(left), Integer(right)) => (left * (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) * right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Mul)?,
        };
        Ok(res)
    }

    fn real_div(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => ((*left as f64) / (*right as f64)).into(),
            (Float(left), Float(right)) => (left / right).into(),
            (Float(left), Integer(right)) => (left / (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) / right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::RealDiv)?,
        };
        Ok(res)
    }

    fn int_div(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (left / right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::IntDiv)?,
        };
        Ok(res)
    }

    fn error<T>(left: &Self, right: Option<&Self>, op: OperatorError) -> Result<T, Error> {
        Err(Error::InvalidOperation(op, left.clone(), right.cloned()))
    }
}

mod tests {
    use super::*;

    #[test]
    fn primitive_negate() {
        let truth = Primitive::Boolean(true);
        assert_eq!(Primitive::Boolean(false), truth.negate().unwrap())
    }

    #[test]
    fn primitive_and() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(false), truth.and(&falsy).unwrap())
    }

    #[test]
    fn primitive_or() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(true), truth.or(&falsy).unwrap())
    }

    #[test]
    fn primitive_eq_boolean() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(false), truth.eq(&falsy).unwrap())
    }

    #[test]
    fn primitive_eq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn primitive_eq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn primitive_eq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn primitive_eq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn primitive_noteq_boolean() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(true), truth.not_eq(&falsy).unwrap())
    }

    #[test]
    fn primitive_noteq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn primitive_noteq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn primitive_noteq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn primitive_noteq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn primitive_less_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn primitive_less_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn primitive_less_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn primitive_less_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn primitive_less_eq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_less_eq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_less_eq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_less_eq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_greater_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn primitive_greater_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn primitive_greater_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn primitive_greater_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn primitive_greater_eq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_greater_eq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_greater_eq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_greater_eq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_sum_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Integer(2), a.add(&b).unwrap())
    }

    #[test]
    fn primitive_sum_float() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(2.0), a.add(&b).unwrap())
    }

    #[test]
    fn primitive_sum_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(2.0), a.add(&b).unwrap())
    }

    #[test]
    fn primitive_sum_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(2.0), a.add(&b).unwrap())
    }

    #[test]
    fn primitive_sub_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Integer(0), a.sub(&b).unwrap())
    }

    #[test]
    fn primitive_sub_float() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(0.0), a.sub(&b).unwrap())
    }

    #[test]
    fn primitive_sub_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(0.0), a.sub(&b).unwrap())
    }

    #[test]
    fn primitive_sub_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(0.0), a.sub(&b).unwrap())
    }

    #[test]
    fn primitive_mul_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Integer(1), a.mul(&b).unwrap())
    }

    #[test]
    fn primitive_mul_float() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(1.0), a.mul(&b).unwrap())
    }

    #[test]
    fn primitive_mul_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(1.0), a.mul(&b).unwrap())
    }

    #[test]
    fn primitive_mul_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(1.0), a.mul(&b).unwrap())
    }

    #[test]
    fn primitive_div_real_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(1.0), a.real_div(&b).unwrap())
    }

    #[test]
    fn primitive_div_real_float() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(1.0), a.real_div(&b).unwrap())
    }

    #[test]
    fn primitive_div_real_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(1.0), a.real_div(&b).unwrap())
    }

    #[test]
    fn primitive_div_real_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(1.0), a.real_div(&b).unwrap())
    }

    #[test]
    fn primitive_div_int_int() {
        let a = Primitive::Integer(10);
        let b = Primitive::Integer(2);
        assert_eq!(Primitive::Integer(5), a.int_div(&b).unwrap())
    }
}