use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub struct Celsius(pub f64);

/*
impl Display for Celsius {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} C", self.0)
    }
}
*/

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Celsius {
        Celsius((f.0 - 32.) * 5. / 9.)
    }
}




#[derive(Debug, Copy, Clone)]
pub struct Fahrenheit(pub f64);

/*
impl Display for Fahrenheit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} F", self.0)
    }
}
*/


macro_rules! temp_display {
    ( $t:ident $fmt:tt ) => {
        impl Display for $t {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, $fmt, self.0)
            }
        }
    }
}

temp_display!(Celsius "{} C");
temp_display!(Fahrenheit "{} F");



pub fn below_freezing<T>(temp: T) -> bool
    where T: Into<Celsius>
{
    let temp: Celsius = temp.into();
    temp.0 < 0.
}
