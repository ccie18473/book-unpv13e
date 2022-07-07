use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;

pub extern "C" fn sig_chld(_: libc::c_int) {
    loop {
        let status = waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG));

        match status {
            Ok(status) => {
                if status == WaitStatus::StillAlive {
                    break;
                }
                println!(
                    "child terminated {:?}\n",
                    status.pid().expect("failed to get pid")
                );
            }
            Err(_) => {
                break;
            }
        }
    }
}

/*

#include	"unp.h"

void
sig_chld(int signo)
{
    pid_t	pid;
    int		stat;

    while ( (pid = waitpid(-1, &stat, WNOHANG)) > 0)
        printf("child %d terminated\n", pid);
    return;
}

*/
