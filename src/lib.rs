#![doc(html_root_url = "https://docs.rs/shapelib-rs/0.2.0")]
//! Rust crate shapelib-rs supports ESRI J shapefile (C bindings)
//!
//! # Requirements
//!
//! - [ OSGeo ]( https://OSGeo.org/ )
//! - [ OSGeo shapelib (C) ]( https://github.com/OSGeo/shapelib )
//! - [ shapelib ]( http://shapelib.maptools.org/ )
//! - [ ESRI J shapefile ]( https://www.esrij.com/products/japan-shp/ )
//!
//! link shapelib_i.lib
//!

pub mod shape;

#[cfg(test)]
mod tests {
  use super::shape::*; // {ShapeC, c, sdup, get_pref_city, u8zs, i8zs, ShapeF};
  use std::path::PathBuf;
  use asciiz::u8z;

  /// with [-- --nocapture] or with [-- --show-output]
  #[test]
  fn check_shapelib() {
unsafe{
    let mut shp = ShapeC::from(7, 3);
    assert!(shp.to_int() == 21);
    assert_eq!(c(5, 3), 15);
}
    assert_eq!(u8zs(b"aB\0".to_vec()).unwrap(),
      String::from_utf8(b"aB".to_vec()).unwrap());
  }

  #[test]
  fn check_path() {
    let s_path: String = if cfg!(docsrs) {
      std::env::var("OUT_DIR").unwrap()
    }else{
      ".".to_string()
    }; // to keep lifetime
    let o_path: &str = s_path.as_str();
    if o_path != "." { return; }
    let bp = PathBuf::from(o_path).join("shp").join("ESRIJ_com_japan_ver84");
    println!("{}", bp.join("japan_ver84.shp").to_str().unwrap());
    println!("{}", bp.join("japan_ver84.dbf").to_str().unwrap());
    let s = bp.join("japan_ver84"); // to keep lifetime
    let u = s.to_str().unwrap(); // to keep lifetime
    let p = u8z::U8zBuf::from_u8array(u.as_bytes()); // to keep lifetime
    let e = u8z::U8zBuf::from_u8array(b"cp932"); // to keep lifetime
unsafe {
    let mut shp = ShapeC::new(p.as_i8p(), e.as_i8p());
    assert!(shp.is_valid());
/*
    shp.disp_record_inf();
    let gci = shp.alloc_gci(); // same as mut
    shp.get_shape(gci);
    shp.free_gci(gci);
*/
    drop(shp);
}
    let shp = ShapeF::new(u, "cp932").unwrap();
    shp.disp_record_inf().unwrap();
    let _sci = shp.get_shp_contours().unwrap();
    drop(shp);
  }
}
