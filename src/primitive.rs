use self::Primitive::*;
use crate::error::Error;
use crate::error::OperatorError;

type Result<T> = ::std::result::Result<T, Error>;

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
    pub fn negate(&self) -> Result<Self> {
        match self {
            Boolean(i) => Ok(Boolean(!*i)),
            left => Self::error(left, None, OperatorError::Negate),
        }
    }

    pub fn minus(&self) -> Result<Self> {
        match self {
            Integer(v) => Ok(Integer(-*v)),
            Float(v) => Ok(Float(-*v)),
            l => Self::error(l, None, OperatorError::UnarySub),
        }
    }

    pub fn plus(&self) -> Result<Self> {
        match self {
            Integer(v) => Ok(Integer((-1 * *v))),
            Float(v) => Ok(Float((-1.0 * *v))),
            l => Self::error(l, None, OperatorError::UnaryPlus),
        }
    }

    pub fn and(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Boolean(left), Boolean(right)) => (*left && *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Add)?,
        };
        Ok(res)
    }

    pub fn or(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Boolean(left), Boolean(right)) => (*left || *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Or)?,
        };
        Ok(res)
    }

    pub fn eq(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left == *right).into(),
            (Boolean(left), Boolean(right)) => (*left == *right).into(),
            (Float(left), Float(right)) => (*left == *right).into(),
            (Float(left), Integer(right)) => (*left == (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) == *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Equal)?,
        };
        Ok(res)
    }

    pub fn not_eq(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left != *right).into(),
            (Boolean(left), Boolean(right)) => (*left != *right).into(),
            (Float(left), Float(right)) => (*left != *right).into(),
            (Float(left), Integer(right)) => (*left != (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) != *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::NotEqual)?,
        };
        Ok(res)
    }

    pub fn less_than(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left < *right).into(),
            (Float(left), Float(right)) => (*left < *right).into(),
            (Float(left), Integer(right)) => (*left < (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) < *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::LessThan)?,
        };
        Ok(res)
    }

    pub fn less_than_equal(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left <= *right).into(),
            (Float(left), Float(right)) => (*left <= *right).into(),
            (Float(left), Integer(right)) => (*left <= (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) <= *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::LessThanEqual)?,
        };
        Ok(res)
    }

    pub fn greater_than(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left > *right).into(),
            (Float(left), Float(right)) => (*left > *right).into(),
            (Float(left), Integer(right)) => (*left > (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) > *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::GreaterThan)?,
        };
        Ok(res)
    }

    pub fn greater_than_equal(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left >= *right).into(),
            (Float(left), Float(right)) => (*left >= *right).into(),
            (Float(left), Integer(right)) => (*left >= (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) >= *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::GreaterThanEqual)?,
        };
        Ok(res)
    }

    pub fn is(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(_), Integer(_)) => (true).into(),
            (Float(_), Float(_)) => (true).into(),
            (Boolean(_), Boolean(_)) => (true).into(),
            (Str(_), Str(_)) => (true).into(),
            (left, right) => (false).into(),
        };
        Ok(res)
    }

    pub fn add(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (left + right).into(),
            (Float(left), Float(right)) => (left + right).into(),
            (Float(left), Integer(right)) => (left + (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) + right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Add)?,
        };
        Ok(res)
    }

    pub fn sub(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (left - right).into(),
            (Float(left), Float(right)) => (left - right).into(),
            (Float(left), Integer(right)) => (left - (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) - right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Sub)?,
        };
        Ok(res)
    }

    pub fn mul(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (left * right).into(),
            (Float(left), Float(right)) => (left * right).into(),
            (Float(left), Integer(right)) => (left * (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) * right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Mul)?,
        };
        Ok(res)
    }

    pub fn real_div(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => ((*left as f64) / (*right as f64)).into(),
            (Float(left), Float(right)) => (left / right).into(),
            (Float(left), Integer(right)) => (left / (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) / right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::RealDiv)?,
        };
        Ok(res)
    }

    pub fn int_div(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (left / right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::IntDiv)?,
        };
        Ok(res)
    }

    pub fn module(&self, other: &Self) -> Result<Self> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (left % right).into(),
            (Float(left), Float(right)) => (left % right).into(),
            (Integer(left), Float(right)) => ((*left as f64) % right).into(),
            (Float(left), Float(right)) => (left % (*right as f64)).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Module)?,
        };
        Ok(res)
    }

    fn error<T>(left: &Self, right: Option<&Self>, op: OperatorError) -> Result<T> {
        Err(Error::InvalidOperation(op, left.clone(), right.cloned()))
    }
}

mod tests {
    use super::*;

    #[test]
    fn negate() {
        let truth = Primitive::Boolean(true);
        assert_eq!(Primitive::Boolean(false), truth.negate().unwrap())
    }

    #[test]
    fn and() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(false), truth.and(&falsy).unwrap())
    }

    #[test]
    fn or() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(true), truth.or(&falsy).unwrap())
    }

    #[test]
    fn eq_boolean() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(false), truth.eq(&falsy).unwrap())
    }

    #[test]
    fn eq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn eq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn eq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn eq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn noteq_boolean() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(true), truth.not_eq(&falsy).unwrap())
    }

    #[test]
    fn noteq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn noteq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn noteq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn noteq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn less_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn less_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn less_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn less_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn less_eq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn less_eq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn less_eq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn less_eq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn greater_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn greater_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn greater_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn greater_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn greater_eq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn greater_eq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn greater_eq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn greater_eq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn sum_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Integer(2), a.add(&b).unwrap())
    }

    #[test]
    fn sum_float() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(2.0), a.add(&b).unwrap())
    }

    #[test]
    fn sum_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(2.0), a.add(&b).unwrap())
    }

    #[test]
    fn sum_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(2.0), a.add(&b).unwrap())
    }

    #[test]
    fn sub_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Integer(0), a.sub(&b).unwrap())
    }

    #[test]
    fn sub_float() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(0.0), a.sub(&b).unwrap())
    }

    #[test]
    fn sub_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(0.0), a.sub(&b).unwrap())
    }

    #[test]
    fn sub_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(0.0), a.sub(&b).unwrap())
    }

    #[test]
    fn mul_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Integer(1), a.mul(&b).unwrap())
    }

    #[test]
    fn mul_float() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(1.0), a.mul(&b).unwrap())
    }

    #[test]
    fn mul_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(1.0), a.mul(&b).unwrap())
    }

    #[test]
    fn mul_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(1.0), a.mul(&b).unwrap())
    }

    #[test]
    fn div_real_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(1.0), a.real_div(&b).unwrap())
    }

    #[test]
    fn div_real_float() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(1.0), a.real_div(&b).unwrap())
    }

    #[test]
    fn div_real_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Float(1.0), a.real_div(&b).unwrap())
    }

    #[test]
    fn div_real_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Float(1.0), a.real_div(&b).unwrap())
    }

    #[test]
    fn div_int_int() {
        let a = Primitive::Integer(10);
        let b = Primitive::Integer(2);
        assert_eq!(Primitive::Integer(5), a.int_div(&b).unwrap())
    }

    #[test]
    fn is() {
        let a = Primitive::Integer(10);
        let b = Primitive::Integer(2);
        assert_eq!(Primitive::Boolean(true), a.is(&b).unwrap())
    }

    #[test]
    fn is_false_result() {
        let a = Primitive::Integer(10);
        let b = Primitive::Float(2.0);
        assert_eq!(Primitive::Boolean(false), a.is(&b).unwrap())
    }

    #[test]
    fn unary_minus() {
        let a = Primitive::Integer(10);
        assert_eq!(Primitive::Integer(-10), a.minus().unwrap())
    }

    #[test]
    fn unary_plus() {
        let a = Primitive::Integer(-10);
        assert_eq!(Primitive::Integer(10), a.plus().unwrap())
    }

    #[test]
    fn module() {
        let a = Primitive::Integer(10);
        let b = Primitive::Integer(2);
        assert_eq!(Primitive::Integer(0), a.module(&b).unwrap())
    }
}
