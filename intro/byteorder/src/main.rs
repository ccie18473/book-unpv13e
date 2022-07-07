fn main() {
    union Union {
        s: i16,
        c: [u8; 2],
    }

    let mut v = Union { s: 0 };
    v.s = 0x0102;
    unsafe {
        if v.c[0] == 1 && v.c[1] == 2 {
            println!("\nbig-endian\n");
        } else if v.c[0] == 2 && v.c[1] == 1 {
            println!("\nlittle-endian\n");
        } else {
            println!("unknown")
        }
    }
}

/*

#include	"unp.h"

int
main(int argc, char **argv)
{
    union {
      short  s;
      char   c[sizeof(short)];
    } un;

    un.s = 0x0102;
    printf("%s: ", CPU_VENDOR_OS);
    if (sizeof(short) == 2) {
        if (un.c[0] == 1 && un.c[1] == 2)
            printf("big-endian\n");
        else if (un.c[0] == 2 && un.c[1] == 1)
            printf("little-endian\n");
        else
            printf("unknown\n");
    } else
        printf("sizeof(short) = %d\n", sizeof(short));

    exit(0);
}

*/
