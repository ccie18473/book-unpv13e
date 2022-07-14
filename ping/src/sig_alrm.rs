use crate::prelude::*;
use crate::SENDER;

pub extern "C" fn sig_alrm(_: i32) {
    unsafe {
        SENDER.get_mut().unwrap().send_v4();
    }
    alarm::set(1);
}

/*

#include	"ping.h"

void
sig_alrm(int signo)
{
    (*pr->fsend)();

    alarm(1);
    return;
}

*/
