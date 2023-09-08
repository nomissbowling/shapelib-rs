//! shape
//!
//! to control publication imported modules
//!
//! # Requirements
//!
//! ./shapelib/include/shapefil.h
//! ./shapelib/lib/shapelib_i.lib
//! ./shapelib/bin/shapelib.dll
//!
//! ./include/bridge.hpp
//! ./src/bridge.cpp

#![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_attributes)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

mod cshapefil;
use cshapefil::*;

mod cppbridge;
pub use cppbridge::{ShapeC, c};

use std::error::Error;
use std::collections::BTreeMap;
use asciiz::u8z;
use encoding_rs;

/// trait Drop
impl Drop for ShapeC {
  /// dispose
  fn drop(&mut self) {
    unsafe { self.dispose(); }
  }
}

/// get pref and city from PPNNN assume JCODE
pub fn get_pref_city(s: &str) -> Result<(i32, i32), Box<dyn Error>> {
  if s.len() < 5 { return Err(format!("expect PPNNN but '{}'", s).into()); }
  let pref = i32::from_str_radix(&s[0..2], 10)?; // s[0..2].parse::<i32>()?;
  let city = i32::from_str_radix(&s[2..5], 10)?; // s[2..5].parse::<i32>()?;
  Ok((pref, city))
}

/// get value of a[i] (a as *mut type)
#[macro_export]
macro_rules! ptr_array {
  ($a: expr, $i: expr) => {
    std::slice::from_raw_parts($a, $i as usize + 1)[$i as usize]
  };
}
pub use ptr_array;

/// create String from u8z (as u8 vector)
pub fn u8zs(b: Vec<u8>) -> Result<String, Box<dyn Error>> {
  let l = b.iter().position(|&c| c == 0).ok_or("not terminated by null")?;
  Ok(String::from_utf8(b[0..l].to_vec())?)
}

/// create String from i8z (as *const i8)
pub fn i8zs(b: *const i8, enc: &str) -> Result<String, Box<dyn Error>> {
unsafe {
  let mut l: usize = 0;
  loop {
    let c = std::slice::from_raw_parts((b as usize + l) as *const i8, 1);
    if c[0] == 0 as i8 { break; }
    l += 1;
  }
  let s = std::slice::from_raw_parts(b as *const u8, l);
  match enc {
  "cp932" => {
    let (cow, enc_used, had_errors) = encoding_rs::SHIFT_JIS.decode(s);
    Ok(cow.into_owned()) // Ok(String::from_utf8((&cow[..]).into())?)
  },
  _ => Ok(String::from_utf8(s.to_vec())?)
  }
}
}

/// V4d
pub type V4d = [f64; 4];

/// Pt
#[derive(Debug)]
pub struct Pt {
  pub x: i32,
  pub y: i32
}

/// Pt2d
#[derive(Debug)]
pub struct Pt2d {
  pub x: f64,
  pub y: f64
}

/// Bounds4d
pub type Bounds4d = Vec<V4d>;

/// Contour2
pub type Contour2 = Vec<Pt>;

/// Contours2
pub type Contours2 = Vec<Contour2>;

/// MapContours
pub type MapContours = BTreeMap<i32, Contours2>;

/// Contour2d
pub type Contour2d = Vec<Pt2d>;

/// Contours2d
pub type Contours2d = Vec<Contour2d>;

/// ShpContours
pub type ShpContours = BTreeMap<i32, Contours2d>;

/// StrFields
pub type StrFields = Vec<String>;

/// RecFields
pub type RecFields = BTreeMap<i32, StrFields>;

/// GrpContoursInf
#[derive(Debug)]
pub struct GrpContoursInf {
  /// minmax
  pub minmax: Bounds4d,
  /// shp
  pub shp: ShpContours,
  /// rec
  pub rec: RecFields,
  /// scale
  pub scale: f64,
  /// offset
  pub offset: Pt2d,
  /// mm
  pub mm: Vec<Pt2d>,
  /// grpContours
  pub grpContours: Vec<usize>,
  /// grpScaledContours
  pub grpScaledContours: MapContours
}

/// GrpContoursInf
impl GrpContoursInf {
  /// constructor
  pub fn new() -> Result<GrpContoursInf, Box<dyn Error>> {
    Ok(GrpContoursInf{
      minmax: vec![[0.0; 4], [0.0; 4]],
      shp: vec![].into_iter().collect(),
      rec: vec![].into_iter().collect(),
      scale: 0.0,
      offset: Pt2d{x: 0.0, y: 0.0},
      mm: vec![],
      grpContours: vec![],
      grpScaledContours: vec![].into_iter().collect()})
  }
}

/// ShapeF
#[derive(Debug)]
pub struct ShapeF {
  /// fname
  pub fname: String,
  /// enc
  pub enc: String,
  /// h_shp
  pub h_shp: *mut SHPInfo,
  /// h_dbf
  pub h_dbf: *mut DBFInfo,
  /// valid
  pub valid: bool
}

/// ShapeF
impl ShapeF {
  /// constructor
  pub fn new(fname: &str, enc: &str) -> Result<ShapeF, Box<dyn Error>> {
    let mut shp = ShapeF{fname: fname.to_string(), enc: enc.to_string(),
      h_shp: 0 as *mut SHPInfo, h_dbf: 0 as *mut DBFInfo, valid: false};
    let fnz = u8z::U8zBuf::from_u8array(fname.as_bytes());
    let opt = u8z::U8zBuf::from_u8array(b"rb");
unsafe {
    shp.h_shp = SHPOpen(fnz.as_i8p(), opt.as_i8p());
    shp.h_dbf = DBFOpen(fnz.as_i8p(), opt.as_i8p());
}
    if shp.h_shp == 0 as *mut SHPInfo || shp.h_dbf == 0 as *mut DBFInfo {
      return Err("ShapeF is not initialized".into());
    }
    shp.valid = true;
    Ok(shp)
  }

  /// dispose
  pub fn dispose(&mut self) {
    if self.h_dbf != 0 as *mut DBFInfo {
unsafe {
      DBFClose(self.h_dbf);
}
      self.h_dbf = 0 as *mut DBFInfo;
    }
    if self.h_shp != 0 as *mut SHPInfo {
unsafe {
      SHPClose(self.h_shp);
}
      self.h_shp = 0 as *mut SHPInfo;
    }
    self.valid = false;
  }

  /// disp_record_inf
  pub fn disp_record_inf(&self) -> Result<(), Box<dyn Error>> {
    println!("SHP: {:?}, DBF: {:?}", self.h_shp, self.h_dbf);
    if !self.valid { return Err("ShapeF is not valid".into()); }
unsafe {
    let records = DBFGetRecordCount(self.h_dbf);
    let fields = DBFGetFieldCount(self.h_dbf);
    println!("DBF: records: {}, fields: {}", records, fields);
    for fld in 0..fields {
      let mut name = vec![0u8; 12];
      let mut fwd = (0i32, 0i32); // fw fd
      let ft = DBFGetFieldInfo(self.h_dbf, fld,
        &mut name[0] as *mut u8 as *mut i8, &mut fwd.0, &mut fwd.1);
      println!(" fld: {} {:?} [{}] {} {}", fld, ft, u8zs(name)?, fwd.0, fwd.1);
    }
}
    Ok(())
  }

  /// get_shape
  pub fn get_shape(&self) -> Result<GrpContoursInf, Box<dyn Error>> {
    if !self.valid { return Err("ShapeF is not valid".into()); }
    let mut gci = GrpContoursInf::new()?;
    let mut entities = 0i32;
    let mut shape_type = 0i32;
unsafe {
    SHPGetInfo(self.h_shp, &mut entities, &mut shape_type,
      &mut gci.minmax[0][0], &mut gci.minmax[1][0]);
}
/*
    println!("SHP: entities: {}, shapeType: {}", entities, shape_type);
    println!("minBound X:{:11.6}, Y:{:11.6}, Z:{:11.6}, M:{:11.6}",
      gci.minmax[0][0], gci.minmax[0][1], gci.minmax[0][2], gci.minmax[0][3]);
    println!("maxBound X:{:11.6}, Y:{:11.6}, Z:{:11.6}, M:{:11.6}",
      gci.minmax[1][0], gci.minmax[1][1], gci.minmax[1][2], gci.minmax[1][3]);
*/
    for i in 0..entities {
unsafe {
      let p_shape = SHPReadObject(self.h_shp, i);
      if p_shape == 0 as *mut tagSHPObject {
        println!("error ReadObject at {}", i);
        continue;
      }
      let shape = *p_shape;
      if shape.nSHPType != SHPT_POLYGON as i32 {
        println!("{} not POLYGON at {}\x07", shape.nSHPType, i);
      }
      // SHPComputeExtents(shape); // recompute the extenst
      // SHPRewindObject(self.h_shp, shape); // if necessary
      let id: i32 = shape.nShapeId;
      let parts: i32 = shape.nParts;
      let vertices: i32 = shape.nVertices;
//      println!("[{:4}]{:4}:{:3}:{:6}", i, id, parts, vertices); // 0-1906
      if parts == 0 { println!("parts==0 at {}\x07", i); }

      let fields = DBFGetFieldCount(self.h_dbf);
      let p = DBFIsRecordDeleted(self.h_dbf, id); // DBFMarkRecordDeleted(...);
      if p != 0 {
        println!("no record DBF[{}]\x07", id);
      } else {
        // needless free
        // DBFRead[Integer|Double|String]Attribute(self.h_dbf, id, f);
        // DBFIsAttributeNULL(self.h_dbf, id, f);
        // DBFGetNativeFieldType(self.h_dbf, f);
        let mut flds = Vec::<String>::with_capacity(fields as usize);
        for j in 0..fields {
          let p_i8zs = DBFReadStringAttribute(self.h_dbf, id, j);
          flds.push(i8zs(p_i8zs, self.enc.as_str())?);
//          println!("-[{}]-[{}]", j, flds[j as usize].as_str());
        }
        let (pref, city) = match get_pref_city(flds[0].as_str()) {
        Err(e) => { println!("{} at {}\x07", e, i); (0, 0) },
        Ok(r) => r
        };
/*
  if pref != 26 { SHPDestroyObject(p_shape); continue; } // 26 1177-1212
  if city != 343 { SHPDestroyObject(p_shape); continue; } // 343 1204 etc
  print!("{:4}:", id);
  for j in 0..fields { print!(" [{}]", flds[j as usize]); }
  for j in 0..4 {
    print!(" {:9.4}", DBFReadDoubleAttribute(self.h_dbf, id, j + 7));
  }
  println!("");
*/
        gci.rec.insert(id, flds); // always create new <k, StrFields> of rec
      }
      let mut contours = Vec::<Contour2d>::with_capacity(parts as usize);
      for n in 0..parts {
//  print!(" {:3}[", n);
        let typ: i32 = ptr_array!(shape.panPartType, n);
        let vi_s: usize = ptr_array!(shape.panPartStart, n) as usize; // (i32)
        let vi_e: usize = if n == parts - 1 { vertices }
          else { ptr_array!(shape.panPartStart, n + 1) } as usize; // (i32)
        let mut contour = Vec::<Pt2d>::with_capacity(vi_e - vi_s); // Contour2d
        for vi in vi_s..vi_e {
          let x: f64 = ptr_array!(shape.padfX, vi);
          let y: f64 = ptr_array!(shape.padfY, vi);
          let z: f64 = ptr_array!(shape.padfZ, vi);
          let m: f64 = ptr_array!(shape.padfM, vi); // 0 if not provided
//  print!(" ({:7.2} {:7.2})", x, y);
          contour.push(Pt2d{x: x, y: y});
        }
        contours.push(contour);
//  println!("]");
      }
      gci.shp.insert(id, contours); // always create new <k, Contours2d> of shp
      SHPDestroyObject(p_shape); // SHPDestroyShape() in API manual
}
    }
  // forget leak into_raw from_raw
    Ok(gci)
  }
}

/// trait Drop
impl Drop for ShapeF {
  // dispose
  fn drop(&mut self) {
    self.dispose();
  }
}
