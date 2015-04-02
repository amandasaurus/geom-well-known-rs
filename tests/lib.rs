extern crate geom_well_known;

use geom_well_known::{Point, ToWKT, FromWKT};

#[test]
fn point_obj() {
    let point = Point::new(0f32, 0f32);
    assert_eq!(point.x, 0f32);
    assert_eq!(point.to_wkt(), "POINT (0 0)".to_string());
    let new_point_o = Point::from_wkt("POINT (1 3)");
    //assert!(new_point_o.is_ok());
    let new_point : Point<isize> = new_point_o.unwrap();
    assert_eq!(new_point.x, 1);
    assert_eq!(new_point.y, 3);
}

//#[test]
//fn lines() {
//    let mut lines = LineString::new();
//    lines.add_point(Point::new(0f32, 0f32));
//    lines.add_point(Point::new(0f32, 1f32));
//    assert_eq!(lines.to_wkt(), "LINESTRING (0 0, 0 1)".to_string());
//    assert_eq!(lines.length(), 1);
//    //let line = LineString::from_wkt("LINESTRING (0 0, 1 1)");
//}