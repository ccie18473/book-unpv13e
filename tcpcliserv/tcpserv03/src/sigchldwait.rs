use nix::sys::wait::wait;

pub extern "C" fn sig_chld(_: libc::c_int) {
    let status = wait();

    match status {
        Ok(status) => {
            println!("child terminated {:?}\n", status);
        }
        Err(_) => {
            println!("waitpid() failed\n");
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

    pid = wait(&stat);
    printf("child %d terminated\n", pid);
    return;
}

*/
