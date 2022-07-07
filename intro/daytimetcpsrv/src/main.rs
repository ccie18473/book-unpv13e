use socket2::*;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::process::Command;

const DAYTIME: u16 = 13;

fn main() {
    let listenfd = Socket::new(Domain::IPV4, Type::STREAM, None).unwrap();
    let servaddr: SockAddr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, DAYTIME).into();

    listenfd.bind(&servaddr).expect("cannot bind");
    listenfd.listen(128).expect("cannot listen");

    loop {
        let (mut connfd, _sa) = listenfd.accept().expect("cannot accept");

        let date = Command::new("date").output().expect("cannot run cmd");
        let date = String::from_utf8(date.stdout).expect("cannot convert cmd");

        connfd.write(date.as_bytes()).expect("cannot send");
        connfd.flush().expect("cannot flush");
    }
}

/*

#include	"unp.h"
#include	<time.h>

int
main(int argc, char **argv)
{
    int					listenfd, connfd;
    struct sockaddr_in	servaddr;
    char				buff[MAXLINE];
    time_t				ticks;

    listenfd = Socket(AF_INET, SOCK_STREAM, 0);

    bzero(&servaddr, sizeof(servaddr));
    servaddr.sin_family      = AF_INET;
    servaddr.sin_addr.s_addr = htonl(INADDR_ANY);
    servaddr.sin_port        = htons(13);	/* daytime server */

    Bind(listenfd, (SA *) &servaddr, sizeof(servaddr));

    Listen(listenfd, LISTENQ);

    for ( ; ; ) {
        connfd = Accept(listenfd, (SA *) NULL, NULL);

        ticks = time(NULL);
        snprintf(buff, sizeof(buff), "%.24s\r\n", ctime(&ticks));
        Write(connfd, buff, strlen(buff));

        Close(connfd);
    }
}

*/
