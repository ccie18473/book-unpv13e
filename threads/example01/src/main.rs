use std::{thread, time::Duration};

const NLOOP: u32 = 100;
static mut COUNTER: i32 = 0;

fn main() {
    //
    let handler_1 = thread::spawn(move || {
        doit();
    });
    //
    let handler_2 = thread::spawn(move || {
        doit();
    });
    //
    handler_1.join().unwrap();
    handler_2.join().unwrap();
    //
}

pub fn doit() {
    let mut val;

    for _i in 0..NLOOP {
        unsafe {
            val = COUNTER;
        }
        println!("{:?}: {:?}", thread::current().id(), val + 1);
        unsafe {
            COUNTER = val + 1;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

/*

#include	"unpthread.h"

#define	NLOOP 5000

int				counter;		/* incremented by threads */

void	*doit(void *);

int
main(int argc, char **argv)
{
    pthread_t	tidA, tidB;

    Pthread_create(&tidA, NULL, &doit, NULL);
    Pthread_create(&tidB, NULL, &doit, NULL);

        /* 4wait for both threads to terminate */
    Pthread_join(tidA, NULL);
    Pthread_join(tidB, NULL);

    exit(0);
}

void *
doit(void *vptr)
{
    int		i, val;

    /*
     * Each thread fetches, prints, and increments the counter NLOOP times.
     * The value of the counter should increase monotonically.
     */

    for (i = 0; i < NLOOP; i++) {
        val = counter;
        printf("%d: %d\n", pthread_self(), val + 1);
        counter = val + 1;
    }

    return(NULL);
}

*/
