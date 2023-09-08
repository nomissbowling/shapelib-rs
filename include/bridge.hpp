/**
 *  bridge.hpp
 */

#ifndef __BRIDGE_H__
#define __BRIDGE_H__

// no more use (only for test)
class ShapeC {
protected:
  bool valid;
  char *fname;
  char *encode;
  void *hSHP; // SHPHandle hSHP;
  void *hDBF; // DBFHandle hDBF;
  int c;
public:
  bool is_valid();
  void dispose();
  virtual ~ShapeC();
  ShapeC(const char *fn, const char *enc);

  void disp_record_inf();
  void *alloc_gci(); // GrpContoursInf
  void free_gci(void *p_gci); // GrpContoursInf
  void get_shape(void *p_gci); // GrpContoursInf

  static ShapeC from(int a, int b);
  int to_int();
};

extern "C" {
int c(int a, int b);
char *sdup(const char *src);
}

#endif // __BRIDGE_H__
