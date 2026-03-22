#include <asm/processor.h>
#include <linux/binfmts.h>
#include <linux/fs.h>
#include <linux/ptrace.h>
#include <linux/sched/task_stack.h>
#include <linux/uaccess.h>

char *get_buf(struct linux_binprm *bprm) { return bprm->buf; }

struct file *veles_get_file(struct linux_binprm *bprm) { return bprm->file; }

ssize_t veles_get_to_user(struct file *file, unsigned long addr, loff_t pos,
                          size_t len) {
  while (len > 0) {
    size_t chunk = len > 512 ? 512 : len;
    char buf[512];
    ssize_t ret = kernel_read(file, buf, chunk, &pos);
    if (ret <= 0) {
      return ret;
    }
    ssize_t res = copy_to_user((void __user *)addr, buf, ret);
    if (res != 0) {
      return -14;
    }
    addr += ret;
    len -= ret;
  }

  return 0;
}

void veles_start_thread(unsigned long ip, unsigned long sp) {
  start_thread(current_pt_regs(), ip, sp);
}
