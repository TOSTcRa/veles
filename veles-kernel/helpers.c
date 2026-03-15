#include <linux/binfmts.h>

char * get_buf(struct linux_binprm * bprm) {
  return bprm->buf;
}
