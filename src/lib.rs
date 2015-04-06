#![feature(plugin)]
#![feature(convert)]
#![feature(core)]
#![plugin(regex_macros)]

extern crate regex;
extern crate rustc_serialize;
extern crate byteorder;
extern crate core;

#[macro_use]
extern crate log;

use std::fmt::Display;
use std::str::FromStr;
use std::io::Cursor;
use std::iter::range_step_inclusive;

use rustc_serialize::hex::{FromHex, ToHex};
use byteorder::{LittleEndian, BigEndian, WriteBytesExt, ReadBytesExt};
use core::num::{ToPrimitive, FromPrimitive, from_f64};

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

#[derive(Debug)]
pub struct LineString<T> {
    pub points: Vec<Point<T>>
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point{ x: x, y: y}
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> LineString<T> {
    pub fn new_empty() -> LineString<T> {
        let points: Vec<Point<T>> = Vec::new();
        LineString{ points: points }
    }
    pub fn push_point(&mut self, newpoint: Point<T>) {
        self.points.push(newpoint);
    }

    pub fn npoints(&self) -> usize {
        self.points.len()
    }
}

impl<T: PartialEq> PartialEq for LineString<T> {
    fn eq(&self, other: &LineString<T>) -> bool {
        self.points == other.points
    }
}

pub trait WKGeom {

    fn to_wkt(&self) -> String;
    fn from_wkt(&str) -> Result<Self, String>;

    fn to_wkb(&self) -> Result<Vec<u8>, String>;
    fn from_wkb(Vec<u8>) -> Result<Self, String>;

    fn to_wkb_hexstring(&self) -> Result<String, String>;
    fn from_wkb_hexstring(String) -> Result<Self, String>;
}

impl<T: Display+FromStr> Point<T> {
    fn point_coords(&self) -> String {
        format!("{} {}", self.x, self.y)
    }
}

impl<T: Display+FromStr+ToPrimitive+FromPrimitive> WKGeom for Point<T> {
    fn to_wkt(&self) -> String {
        format!("POINT ({})", self.point_coords())
    }

    fn from_wkt(wkt: &str) -> Result<Point<T>, String> {
        let wkt = wkt.trim();
        let re = regex!(r"POINT *\( *(.*?) +(.*?) *\)");
        let cap = try!(re.captures(wkt).ok_or("Cannot match point regex".to_string()));
        let x_str = try!(cap.at(1).ok_or("Cannot find x".to_string()));
        let x = try!(T::from_str(x_str).or(Err(format!("Could not convert {} from string", x_str))));
        let y_str  = try!(cap.at(2).ok_or("Cannot find y".to_string()));
        let y = try!(T::from_str(y_str).or(Err(format!("Could not convert {} from string", y_str))));

        return Ok(Point::new(x, y));

    }

    fn to_wkb(&self) -> Result<Vec<u8>, String> {
        let mut results: Vec<u8> = Vec::new();
        
        // We only optput Little Endian
        results.write_u8(1);

        // point 
        results.write_u32::<LittleEndian>(1);

        // The x and the y
        results.write_f64::<LittleEndian>(try!(self.x.to_f64().ok_or("Could not convert X to f64".to_string())));
        results.write_f64::<LittleEndian>(try!(self.y.to_f64().ok_or("Could not convert Y to f64".to_string())));

        Ok(results)
    }

    fn from_wkb(input: Vec<u8>) -> Result<Point<T>, String> {
        if input.len() != 21 {
            return Err(format!("Too short length of {} instead of 21", input.len()));
        }
        let mut cursor = Cursor::new(input);
        cursor.set_position(0);
        let little_endianness = match try!(cursor.read_u8().or(Err("Couldn't read"))) {
            0 => { false },
            1 => { true },
            x => { return Err(format!("Invalid endianness, got {} instead of 0 or 1", x)) }
        };

        let geom_type = try!(match little_endianness {
            true => cursor.read_u32::<LittleEndian>(),
            false => cursor.read_u32::<BigEndian>()
        }.or(Err("Could not read geom type")));
        if geom_type != 1 {
            return Err(format!("Unknown geom type. Got {}, expected 1", geom_type));
        }
        let x: f64 = try!(match little_endianness {
            true => cursor.read_f64::<LittleEndian>(),
            false => cursor.read_f64::<BigEndian>()
        }.or(Err("Could not parse out X")));
        let y: f64 = try!(match little_endianness {
            true => cursor.read_f64::<LittleEndian>(),
            false => cursor.read_f64::<BigEndian>()
        }.or(Err("Could not parse out Y")));

        let x: T = try!(from_f64(x).ok_or(format!("Could not convert X={} from f64", x)));
        let y: T = try!(from_f64(y).ok_or(format!("Could not convert Y={} from f64", y)));

        Ok(Point{ x:x, y: y })
        
    }

    fn to_wkb_hexstring(&self) -> Result<String, String> {
        Ok(try!(self.to_wkb()).as_slice().to_hex())
    }

    fn from_wkb_hexstring(input: String) -> Result<Self, String> {
        let bin_string = try!(input.as_slice().from_hex().or(Err("Could not convert from hex".to_string())));
        Point::from_wkb(bin_string)
    }
}

impl<T: Display+FromStr+ToPrimitive+FromPrimitive> WKGeom for LineString<T> {
    fn to_wkt(&self) -> String {
        format!("LINESTRING ({})", self.points.iter().map(|ref x| { x.point_coords() }).collect::<Vec<String>>().connect(", "))
    }

    fn from_wkt(wkt: &str) -> Result<LineString<T>, String> {
        let wkt = wkt.trim();
        let re = regex!(r"LINESTRING *\( *(.*?) +(.*?)(?: *, *(.*?) (.*?))* *\)");
        let cap = try!(re.captures(wkt).ok_or("Cannot match point regex".to_string()));

        let mut result: LineString<T> = LineString::new_empty();

        for i in range_step_inclusive(1, cap.len()-1, 2) {
            let x_str = try!(cap.at(i).ok_or("Cannot find x".to_string()));
            let x = try!(T::from_str(x_str).or(Err(format!("Could not convert {} from string", x_str))));
            let y_str  = try!(cap.at(i+1).ok_or("Cannot find y".to_string()));
            let y = try!(T::from_str(y_str).or(Err(format!("Could not convert {} from string", y_str))));
            
            result.push_point(Point::new(x, y));
        }

        Ok(result)
    }

    fn to_wkb(&self) -> Result<Vec<u8>, String> {
        let mut result: Vec<u8> = Vec::new();
        
        // We only optput Little Endian
        result.write_u8(1);

        // LineString 
        result.write_u32::<LittleEndian>(2);

        // how many points do we have?
        result.write_u32::<LittleEndian>(try!(self.points.len().to_u32().ok_or("Could not convert length to u32?!")));

        for point in self.points.iter() {
            // The x and the y
            result.write_f64::<LittleEndian>(try!(point.x.to_f64().ok_or("Could not convert X to f64".to_string())));
            result.write_f64::<LittleEndian>(try!(point.y.to_f64().ok_or("Could not convert Y to f64".to_string())));
        }

        Ok(result)
    }

    fn from_wkb(input: Vec<u8>) -> Result<Self, String> {
        let mut cursor = Cursor::new(input);
        cursor.set_position(0);
        let little_endianness = match try!(cursor.read_u8().or(Err("Couldn't read"))) {
            0 => { false },
            1 => { true },
            x => { return Err(format!("Invalid endianness, got {} instead of 0 or 1", x)) }
        };

        let geom_type = try!(match little_endianness {
            true => cursor.read_u32::<LittleEndian>(),
            false => cursor.read_u32::<BigEndian>()
        }.or(Err("Could not read geom type")));
        if geom_type != 2 {
            return Err(format!("Unknown geom type. Got {}, expected 2", geom_type));
        }

        let num_points = try!(match little_endianness {
            true => cursor.read_u32::<LittleEndian>(),
            false => cursor.read_u32::<BigEndian>()
        }.or(Err("Could not read num points type")));

        let mut result: LineString<T> = LineString::new_empty();


        for i in 0..num_points {
            let x: f64 = try!(match little_endianness {
                true => cursor.read_f64::<LittleEndian>(),
                false => cursor.read_f64::<BigEndian>()
            }.or(Err("Could not parse out X")));
            let y: f64 = try!(match little_endianness {
                true => cursor.read_f64::<LittleEndian>(),
                false => cursor.read_f64::<BigEndian>()
            }.or(Err("Could not parse out Y")));

            let x: T = try!(from_f64(x).ok_or(format!("Could not convert X={} from f64", x)));
            let y: T = try!(from_f64(y).ok_or(format!("Could not convert Y={} from f64", y)));

            result.push_point(Point{ x:x, y: y })
        }

        Ok(result)

        
    }

    fn to_wkb_hexstring(&self) -> Result<String, String> {
        Ok(try!(self.to_wkb()).as_slice().to_hex())
    }

    fn from_wkb_hexstring(input: String) -> Result<Self, String> {
        let bin_string = try!(input.as_slice().from_hex().or(Err("Could not convert from hex".to_string())));
        LineString::from_wkb(bin_string)
    }
}
