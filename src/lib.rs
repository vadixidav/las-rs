//! Reads and writes point cloud data stored in the ASPRS `las` file format.
//!
//! The las file format [as defined by
//! ASPRS](http://www.asprs.org/Committee-General/LASer-LAS-File-Format-Exchange-Activities.html)
//! is the de-facto standard for transmission of point cloud data collected by
//! [LiDAR](https://en.wikipedia.org/wiki/Lidar) sensors. This is a Rust library that reads and writes
//! `las` files.
//!
//! # Reading points
//!
//! To read a point from a las file, use `las::Reader::next_point()` (see [the Rust
//! Book](https://doc.rust-lang.org/book/if-let.html) for information on that cryptic `if let`
//! syntax):
//!
//! ```
//! let mut reader = las::Reader::from_path("data/1.0_0.las").unwrap();
//! let point = reader.next_point().unwrap();
//! if let Some(point) = point {
//!     println!("Read point (x={}, y={}, z={})", point.x, point.y, point.z);
//! } else {
//!     println!("End of file");
//! }
//! ```
//!
//! A `Reader` can be used to iterate over all points in the file:
//!
//! ```
//! # let mut reader = las::Reader::from_path("data/1.0_0.las").unwrap();
//! for point in reader {
//!     println!("Another point!");
//! }
//! ```
//!
//! # Writing points
//!
//! Writing points follows a similar pattern, using `las::Writer`:
//!
//! ```
//! let mut writer = las::Writer::from_path("/dev/null");
//! writer.write_point(las::Point::new());
//! writer.close().unwrap();
//! ```
//!
//! You can configure a `Writer` using the builder pattern:
//!
//! ```
//! let writer = las::Writer::from_path("/dev/null")
//!     .version(1, 2)
//!     .point_format(las::PointFormat(1))
//!     .offsets(1000.0, 2000.0, 100.0)
//!     .scale_factors(0.01, 0.01, 0.01);
//! ```
//!
//! Of course, see each structure's documentation for a complete listing of available methods.
//!
//! # Previous art
//!
//! Several software libraries already exist to read and write las files:
//!
//! - [LAStools](http://www.cs.unc.edu/~isenburg/lastools/) is a sortof open source library that was
//! the first major player in the space, and remains highly used. LAStools is written in C++ and it
//! has (as its name implies) many tools on top of simple format reading and writing. It does not
//! really present a usable software API, so it is more of a toolkit than an upstream dependency.
//! - [libLAS](http://www.liblas.org/) is another C++ library and set of executables that mirrors
//! much of the functionality of LAStools, but with a bit cleaner software engineering. libLAS is
//! not under active development, and so is a bit feature-lacking, e.g. it does not support las
//! formats 1.3 and 1.4. It has been largely superseded by PDAL (see below).
//! - [Laspy](http://laspy.readthedocs.org/en/latest/) is a native Python las reader and writer.
//! - [PDAL](http://www.pdal.io/) is a higher-level point cloud data format translation library
//! that includes las support, including support for versions 1.3 and 1.4. Like LAStools, PDAL
//! inclues a broad suite of additional tooling above and beyond simple data translation. PDAL is
//! under active development.
//!
//! There are also numerous commercial products that can read and write las files.
//!
//! # Why another las library?
//!
//! This project started as a hobby project as a way to explore both Rust and the las format.
//! However, due to the lack of a simple las library with a stable and modern C api, reading las
//! data in Rust required writing something from scratch.
//!
//! In the long term, it might be possible to provide C bindings to this library, to provide that
//! modern C api that other projects can use.

#![deny(missing_copy_implementations, missing_debug_implementations, missing_docs, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_extern_crates,
        unused_import_braces, unused_qualifications)]

extern crate byteorder;
extern crate time;

mod io;
mod scale;
pub mod error;
pub mod file;
pub mod header;
pub mod point;
pub mod reader;
pub mod vlr;
pub mod writer;

pub use error::Error;
pub use file::File;
pub use header::PointFormat;
pub use point::Point;
pub use reader::Reader;
pub use writer::Writer;

use std::result;

/// Crate-specific resuls.
pub type Result<T> = result::Result<T, Error>;
