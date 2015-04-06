extern crate geom_well_known;

use geom_well_known::{LineString, Point, WKGeom};

#[test]
fn point_obj() {
    let point = Point::new(0f32, 0f32);
    assert_eq!(point.x, 0f32);
    assert_eq!(point.to_wkt(), "POINT (0 0)".to_string());
    assert_eq!(point.to_wkt(), "POINT (0 0)".to_string());


    let new_point_o = Point::from_wkt("POINT (1 3)");
    assert!(new_point_o.is_ok());
    let new_point : Point<isize> = new_point_o.unwrap();
    assert_eq!(new_point.x, 1);
    assert_eq!(new_point.y, 3);

    let point = Point::new(2f64, 4f64);
    let hex_wkb = "010100000000000000000000400000000000001040".to_string();
    assert_eq!(point.to_wkb_hexstring().unwrap(), hex_wkb);

    let new_point = Point::from_wkb_hexstring(hex_wkb).unwrap();
    assert_eq!(new_point, point);
}

#[test]
fn lines() {
    let mut line = LineString::new_empty();
    line.push_point(Point::new(0f64, 0f64));
    line.push_point(Point::new(0f64, 1f64));
    assert_eq!(line.to_wkt(), "LINESTRING (0 0, 0 1)".to_string());
    assert_eq!(line.to_wkb_hexstring().unwrap(), "010200000002000000000000000000000000000000000000000000000000000000000000000000f03f".to_string());
    
    let new_line: LineString<f64> = LineString::from_wkb(line.to_wkb().unwrap()).unwrap();
    assert_eq!(new_line, line);
    //let wkt = "LINESTRING( 0 0,   0 1)";
    //let new_line: LineString<f64> = LineString::from_wkt(wkt).unwrap();
    //assert_eq!(line, new_line)

}
