use std::fmt::Debug;

pub trait ToDebug {
    fn to_debug(&self) -> String;
    fn to_debug_pretty(&self) -> String;
}

impl<T> ToDebug for T
where
    T: Debug,
{
    fn to_debug(&self) -> String {
        format!("{:?}", self)
    }

    fn to_debug_pretty(&self) -> String {
        format!("{:?}", self)
    }
}
