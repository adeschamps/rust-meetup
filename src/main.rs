
#[macro_use]
mod temp;
use temp::*;

use std::marker::PhantomData;
use std::fmt;
use std::fmt::{Display, Formatter};

use std::ops::{Add, Sub};


#[derive(Debug)]
struct Distance<T: LengthUnit> {
    meters: f64,
    unit: PhantomData<T>,
}

trait LengthUnit: Sized {
    fn unit_name() -> &'static str;

    fn in_meters() -> f64;

    fn new(value: f64) -> Distance<Self> {
        Distance {
            meters: value * Self::in_meters(),
            unit: PhantomData,
        }
    }
}

impl<T> Display for Distance<T>
    where T: LengthUnit
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {}", self.meters / T::in_meters(), T::unit_name())
    }
}





#[derive(Debug)]
struct Meter;

impl LengthUnit for Meter {
    fn unit_name() -> &'static str {
        "meters"
    }

    fn in_meters() -> f64 {
        1.
    }
}

macro_rules! meter {
    ( $value:expr ) => { Meter::new($value) };
}

macro_rules! inch {
    ( $value:expr ) => { Inch::new($value) };
}






#[derive(Debug)]
struct Inch;

impl LengthUnit for Inch {
    fn unit_name() -> &'static str {
        "inches"
    }

    fn in_meters() -> f64 {
        0.0254
    }
}



struct Test(f64);

temp_display!(Test "test {}");


macro_rules! impl_op {
    ( $trait_name:ident $fn_name:ident $op:tt ) => {
        impl<T1, T2> $trait_name<Distance<T2>> for Distance<T1>
            where T1: LengthUnit,
                  T2: LengthUnit
        {
            type Output = Distance<T1>;

            fn $fn_name(self, rhs: Distance<T2>) -> Self::Output {
                Distance {
                    meters: self.meters $op rhs.meters,
                    unit: PhantomData
                }
            }
        }
    }
}

impl_op!( Add add + );
impl_op!( Sub sub - );

/*
impl<T1, T2> Add<Distance<T2>> for Distance<T1>
    where T1: LengthUnit,
          T2: LengthUnit
{
    type Output = Distance<T1>;

    fn add(self, rhs: Distance<T2>) -> Self::Output {
        Distance {
            meters: self.meters + rhs.meters,
            unit: PhantomData
        }
    }
}
*/


fn main() {

    let temp = Celsius(10.);
    // let temp = Fahrenheit(10.);
    println!("Is {} below freezing? {}", temp, below_freezing(temp));


    let length_one = inch!(2.);
    let length_two = meter!(2.);
    println!("The total is {}", length_one - length_two);
}
