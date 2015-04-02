#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::fmt::Display;
use std::str::FromStr;

pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point{ x: x, y: y}
    }
}

pub trait ToWKT {
    fn to_wkt(&self) -> String;
}

pub trait FromWKT {
    fn from_wkt(&str) -> Result<Self, String>;
}

impl<T: Display> ToWKT for Point<T> {
    fn to_wkt(&self) -> String {
        format!("POINT ({} {})", self.x, self.y)
    }
}

impl<T: FromStr> FromWKT for Point<T> {
    fn from_wkt(wkt: &str) -> Result<Point<T>, String> {
        let wkt = wkt.trim();
        let re = regex!(r"POINT *\( *(.*?) +(.*?) *\)");
        let cap = try!(re.captures(wkt).ok_or("Cannot match point regex".to_string()));
        let x_str = try!(cap.at(1).ok_or("Cannot find x".to_string()));
        let x = match T::from_str(x_str) {
            Err(_) => { return Err(format!("Could not convert {} from string", x_str)) },
            Ok(x) => { x }
        };
        //let y = try!(T::from_str(try!(cap.at(2).ok_or("Cannot find y"))));
        let y_str  = try!(cap.at(2).ok_or("Cannot find y".to_string()));
        let y = match T::from_str(y_str) {
            Err(_) => {  return Err("Could not convert from string".to_string()) },
            Ok(y) => { y }
        };
        return Ok(Point::new(x, y));

    }
}
