This crate provides methods to read & write (Extended) Well Known Text ((E)WKT)
and (Extended) Well Known Binary ((E)WKB), the text & binary encoding of GIS
geometry objects.

[![Build Status](https://travis-ci.org/rory/geom-well-known-rs.svg)](https://travis-ci.org/rory/geom-well-known-rs)

Licenced under GNU GPL v3 (or later). See LICENCE file.

This is alpha quality software, not all features, or geometry types are
supported. This table lists current features supported.

| Geometry Type  | read WKT | write WKT | read WKB | write WKB |
|----------------|----------|-----------|----------|-----------|
| Point          |   [x]    |     [x]   |   [x]    |    [x]    |
| MultiPoint     |   [ ]    |     [ ]   |   [ ]    |    [ ]    |
| Linestring     |   [x]    |     [x]   |   [.]    |    [x]    |
| MultiLinestring|   [ ]    |     [ ]   |   [ ]    |    [ ]    |
| Polygon        |   [ ]    |     [ ]   |   [ ]    |    [ ]    |
| MultiPolygon   |   [ ]    |     [ ]   |   [ ]    |    [ ]    |

