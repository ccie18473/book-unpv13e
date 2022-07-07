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
        let (mut connfd, cliaddr) = listenfd.accept().expect("cannot accept");
        let cli_sa = cliaddr.as_socket_ipv4().expect("cannot get client socket info");
        println!("connection from {}, port {}", cli_sa.ip(), cli_sa.port());

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
	socklen_t			len;
	struct sockaddr_in	servaddr, cliaddr;
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
		len = sizeof(cliaddr);
		connfd = Accept(listenfd, (SA *) &cliaddr, &len);
		printf("connection from %s, port %d\n",
			   Inet_ntop(AF_INET, &cliaddr.sin_addr, buff, sizeof(buff)),
			   ntohs(cliaddr.sin_port));

        ticks = time(NULL);
        snprintf(buff, sizeof(buff), "%.24s\r\n", ctime(&ticks));
        Write(connfd, buff, strlen(buff));

		Close(connfd);
	}
}

*/
