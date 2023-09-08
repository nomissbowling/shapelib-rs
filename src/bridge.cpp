/// bridge.cpp
/// no more use (only for test)

#include <iomanip>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <map>

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cmath>

using namespace std;

#include <../shapelib/include/shapefil.h>

#include <bridge.hpp>

typedef vector<double> V4d; // cv::Vec4d
// typedef double V4d[4]; // cv::Vec4d
typedef struct { int x, y; } Pt; // cv::Point <i32> or <T>
typedef struct { double x, y; } Pt2d; // cv::Point2d

typedef vector<V4d> Bounds4d; // cv::Vec4d
typedef vector<Pt> Contour2; // cv::Point
typedef vector<Contour2> Contours2;
typedef map<int, Contours2> MapContours;
typedef vector<Pt2d> Contour2d; // cv::Point2d
typedef vector<Contour2d> Contours2d;
typedef map<int, Contours2d> ShpContours;
typedef vector<string> StrFields;
typedef map<int, StrFields> RecFields;

typedef struct _GrpContoursInf {
  Bounds4d minmax;
  ShpContours shp;
  RecFields rec;
  double scale;
  Pt2d offset; // cv::Point2d
  vector<Pt2d> mm; // cv::Point2d
  vector<size_t> grpContours;
  MapContours grpScaledContours;
} GrpContoursInf;

/// dup str
char *sdup(const char *src)
{
  if(!src) return NULL;
  size_t sl = strlen(src);
  char *dst = (char *)malloc(sl + 1);
  if(dst){
    strncpy(dst, src, sl);
    dst[sl] = '\0';
  }
  return dst;
}

#define SHP() ((SHPHandle)hSHP)
#define DBF() ((DBFHandle)hDBF)

/// validation
bool ShapeC::is_valid()
{
  return valid;
}

/// dispose
void ShapeC::dispose()
{
  if(hDBF){ DBFClose(DBF()); hDBF = NULL; }
  if(hSHP){ SHPClose(SHP()); hSHP = NULL; }
  if(encode){ free(encode); encode = NULL; }
  if(fname){ free(fname); fname = NULL; }
  valid = false;
}

/// destruct
ShapeC::~ShapeC()
{
  dispose();
}

/// construct
ShapeC::ShapeC(const char *fn, const char *enc) : valid(false),
  fname(sdup(fn)), encode(sdup(enc)), hSHP(NULL), hDBF(NULL), c(0)
{
  if(!fname || !encode) return;

  hSHP = (void *)SHPOpen(fn, "rb");
  if(!hSHP){
    fprintf(stderr, "can not open: %s.shp\n", fn);
    return;
  }
  hDBF = (void *)DBFOpen(fn, "rb");
  if(!hDBF){
    fprintf(stderr, "can not open: %s.dbf\n", fn);
    return;
  }
  valid = true;
}

/// disp_record_inf
void ShapeC::disp_record_inf()
{
/*
DBF: text file fixed length string db (only 1 line fields info binary header)
<....><----length=19----><----length=19---->
00..667777<-prec=11->77778888<-prec=11->8888
        7.40200000000e+03  3.45700000000e+03
*/
  int records = DBFGetRecordCount(DBF()), fields = DBFGetFieldCount(DBF());
  fprintf(stdout, "DBF: records: %d, fields: %d\n", records, fields);
  for(int fld = 0; fld < fields; ++fld){
    char name[12];
    int fw, fd;
    DBFFieldType ft = DBFGetFieldInfo(DBF(), fld, name, &fw, &fd);
    fprintf(stdout, " fld: %d %d [%s] %d %d\n", fld, ft, name, fw, fd);
    // ft: FTString, FTInteger, FTDouble, FTLogical, FTInvalid
  }
/* ver83
DBF: records: 1907, fields: 9
 fld: 0 0 [JCODE] 254 0 # PPNNN
 fld: 1 0 [KEN] 10 0 # pref
 fld: 2 0 [SICHO] 20 0 #
 fld: 3 0 [GUN] 20 0 # *gun
 fld: 4 0 [SEIREI] 20 0 # *shi
 fld: 5 0 [SIKUCHOSON] 20 0 # *ku
 fld: 6 0 [CITY_ENG] 254 0 # *-shi, *-ku
 fld: 7 2 [P_NUM] 19 11
 fld: 8 2 [H_NUM] 19 11
*/
/* ver84
DBF: records: 1907, fields: 11
 fld: 0 0 [JCODE] 254 0 # PPNNN
 fld: 1 0 [KEN] 254 0 # pref
 fld: 2 0 [SICHO] 254 0 #
 fld: 3 0 [GUN] 20 0 # *gun
 fld: 4 0 [SEIREI] 20 0 # *shi
 fld: 5 0 [SIKUCHOSON] 20 0 # *ku
 fld: 6 0 [CITY_ENG] 254 0 # *-shi, *-ku
 fld: 7 2 [P_NUM] 10 0
 fld: 8 2 [H_NUM] 10 0
 fld: 9 2 [Shape_Leng] 19 11
 fld: 10 2 [Shape_Area] 19 11
*/
}

/// alloc_gci
void *ShapeC::alloc_gci()
{
  return malloc(sizeof(GrpContoursInf));
}

/// free_gci
void ShapeC::free_gci(void *p_gci)
{
  free(p_gci);
}

/// get_shape
void ShapeC::get_shape(void *p_gci)
{
/*
  GrpContoursInf *gci = (GrpContoursInf *)p_gci;
  gci->minmax.reserve(2);
  gci->minmax[0].reserve(4);
  gci->minmax[1].reserve(4);
  int entities, shapeType;
  SHPGetInfo(SHP(), &entities, &shapeType,
    &gci->minmax[0][0], &gci->minmax[1][0]);
  fprintf(stdout, "SHP: entities: %d, shapeType: %d\n", entities, shapeType);
  fprintf(stdout, "minBound X:%11.6f, Y:%11.6f, Z:%11.6f, M:%11.6f\n",
    gci->minmax[0][0], gci->minmax[0][1], gci->minmax[0][2], gci->minmax[0][3]);
  fprintf(stdout, "maxBound X:%11.6f, Y:%11.6f, Z:%11.6f, M:%11.6f\n",
    gci->minmax[1][0], gci->minmax[1][1], gci->minmax[1][2], gci->minmax[1][3]);
*/
/*
SHP: entities: 1907, shapeType: 5 (SHPT_POLYGON)
minBound X: 122.933913, Y:  24.045616, Z:   0.000000, M:   0.000000
maxBound X: 153.986675, Y:  45.557239, Z:   0.000000, M:   0.000000
*/
}

/// construct (move)
ShapeC ShapeC::from(int a, int b)
{
  ShapeC s(NULL, NULL);
  s.c = a * b;
  return s;
}

/// test
int ShapeC::to_int()
{
  return c;
}

/// test
int c(int a, int b)
{
  ShapeC s = ShapeC::from(a, b);
  return s.to_int();
}
