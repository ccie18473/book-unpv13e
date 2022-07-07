mod str_cli;

use socket2::*;
use std::io;
use std::net::SocketAddr;
use std::{env, process::exit};

const SERV_PORT: usize = 9877;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: <IPaddress>\n");
        exit(1);
    }

    let mut sockfd = Socket::new(Domain::IPV4, Type::STREAM, None).expect("cannot connect");
    let servaddr = format!("{}:{}", args[1], SERV_PORT);
    let servaddr: SocketAddr = servaddr.parse().expect("cannot parse socket address");

    sockfd.connect(&servaddr.into()).expect("cannot connect");
    
    let fd = io::stdin();

    str_cli::str_cli(&fd, &mut sockfd);

    exit(0);
}

/*

#include	"unp.h"

int
main(int argc, char **argv)
{
    int					sockfd;
    struct sockaddr_in	servaddr;

    if (argc != 2)
        err_quit("usage: tcpcli <IPaddress>");

    sockfd = Socket(AF_INET, SOCK_STREAM, 0);

    bzero(&servaddr, sizeof(servaddr));
    servaddr.sin_family = AF_INET;
    servaddr.sin_port = htons(SERV_PORT);
    Inet_pton(AF_INET, argv[1], &servaddr.sin_addr);

    Connect(sockfd, (SA *) &servaddr, sizeof(servaddr));

    str_cli(stdin, sockfd);		/* do it all */

    exit(0);
}

*/
