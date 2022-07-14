#[derive(Debug)]
pub struct Timeval {
    pub time_send: u128,
    pub time_receive: u128,
}

impl Timeval {
    pub fn new() -> Self {
        Self {
            time_send: 0,
            time_receive: 0,
        }
    }
    pub fn rtt_calc(&mut self) -> f64 {
        let rtt = self.time_receive - self.time_send;
        let rtt = rtt as f64 / 1000.0;
        rtt
    }
}

/*

#include	"unp.h"

void
tv_sub(struct timeval *out, struct timeval *in)
{
    if ( (out->tv_usec -= in->tv_usec) < 0) {	/* out -= in */
        --out->tv_sec;
        out->tv_usec += 1000000;
    }
    out->tv_sec -= in->tv_sec;
}

*/
