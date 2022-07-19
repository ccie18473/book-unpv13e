use std::{
    sync::{Arc, Mutex, MutexGuard},
    thread,
    time::Duration,
};

const NLOOP: u32 = 100;

fn main() {
    //
    let counter_arc_mtx = Arc::new(Mutex::new(0));
    //
    let counter_arc_clone = Arc::clone(&counter_arc_mtx);
    let handler_1 = thread::spawn(move || {
        doit(counter_arc_clone);
    });
    //
    let counter_arc_clone = Arc::clone(&counter_arc_mtx);
    let handler_2 = thread::spawn(move || {
        doit(counter_arc_clone);
    });
    //
    handler_1.join().unwrap();
    handler_2.join().unwrap();
    //
}

pub fn doit(counter_arc_clone: Arc<Mutex<i32>>) {
    let mut val;

    for _i in 0..NLOOP {
        {
            // mutex lock
            let mut counter = counter_arc_clone.lock().unwrap();

            val = *counter;
            println!("{:?}: {:?}", thread::current().id(), val + 1);
            *counter = val + 1;
            // mutex unlock
        }
        thread::sleep(Duration::from_millis(100));
    }
}

/*

#include	"unpthread.h"

#define	NLOOP 5000

int				counter;		/* incremented by threads */
pthread_mutex_t	counter_mutex = PTHREAD_MUTEX_INITIALIZER;

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
        Pthread_mutex_lock(&counter_mutex);

        val = counter;
        printf("%d: %d\n", pthread_self(), val + 1);
        counter = val + 1;

        Pthread_mutex_unlock(&counter_mutex);
    }

    return(NULL);
}

*/
