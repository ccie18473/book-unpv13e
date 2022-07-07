mod sigchldwait;
mod signal;
mod str_echo;

use nix::sys::signal::SIGCHLD;
use nix::unistd::{fork, ForkResult};
use socket2::*;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::process::{exit, id};

const SERV_PORT: u16 = 9877;

fn main() {
    let listenfd = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
    let servaddr: SockAddr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, SERV_PORT).into();

    listenfd.bind(&servaddr).expect("cannot bind");
    listenfd.listen(128).expect("cannot listen");

    signal::signal(SIGCHLD, sigchldwait::sig_chld);

    loop {
        let (mut connfd, _cliaddr) = listenfd.accept().expect("cannot accept");

        match unsafe { fork() } {
            Ok(ForkResult::Parent { child }) => {
                println!("parent pid: {}. child pid: {}", id(), child);
            }
            Ok(ForkResult::Child) => {
                println!("child pid: {}.", id());
                str_echo::str_echo(&mut connfd);
                exit(0);
            }
            Err(err) => println!("cannot fork: {}", err),
        }
    }
}

/*

#include	"unp.h"

int
main(int argc, char **argv)
{
    int					listenfd, connfd;
    pid_t				childpid;
    socklen_t			clilen;
    struct sockaddr_in	cliaddr, servaddr;
    void				sig_chld(int);

    listenfd = Socket(AF_INET, SOCK_STREAM, 0);

    bzero(&servaddr, sizeof(servaddr));
    servaddr.sin_family      = AF_INET;
    servaddr.sin_addr.s_addr = htonl(INADDR_ANY);
    servaddr.sin_port        = htons(SERV_PORT);

    Bind(listenfd, (SA *) &servaddr, sizeof(servaddr));

    Listen(listenfd, LISTENQ);

    Signal(SIGCHLD, sig_chld);

    for ( ; ; ) {
        clilen = sizeof(cliaddr);
        connfd = Accept(listenfd, (SA *) &cliaddr, &clilen);

        if ( (childpid = Fork()) == 0) {	/* child process */
            Close(listenfd);	/* close listening socket */
            str_echo(connfd);	/* process the request */
            exit(0);
        }
        Close(connfd);			/* parent closes connected socket */
    }
}

*/
