#pragma version(1)

#pragma rs java_package_name(com.android.rs.test)

struct Inner_1 {
  uint32_t y;
};

struct Outer {
  struct Inner_1 current;
};

extern uint32_t uint32_ret;

extern struct Outer *outer;

void outer_y(void) {
  uint32_ret = outer->current.y;
}

