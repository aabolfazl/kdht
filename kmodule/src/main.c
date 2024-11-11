#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>

MODULE_LICENSE("GPL");
MODULE_AUTHOR("Abolfazl Abbasi");
MODULE_DESCRIPTION("kernel space implementation of Kademlia routing system");
MODULE_VERSION("1.0");

static int __init lkm_example_init(void) {
    printk(KERN_INFO "Hello, Kademlia.\n");
    return 0;
}

static void __exit lkm_example_exit(void) {

}

module_init(lkm_example_init);
module_exit(lkm_example_exit);
