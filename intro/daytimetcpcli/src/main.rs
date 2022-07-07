use socket2::*;
use std::io::prelude::*;
use std::net::SocketAddr;
use std::{env, process};

const MAXLINE: usize = 4096;
const DAYTIME: usize = 13;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!(
            "usage: <IPaddress>\nexamples: 129.6.15.27, 129.6.15.28, 129.6.15.29, 129.6.15.30"
        );
        process::exit(1);
    }

    let mut sockfd = Socket::new(Domain::IPV4, Type::STREAM, None).expect("cannot connect");
    let servaddr = format!("{}:{}", args[1], DAYTIME);
    let servaddr: SocketAddr = servaddr.parse().expect("cannot parse socket address");
    let mut recvline = [0; MAXLINE];

    sockfd.connect(&servaddr.into()).expect("cannot connect");
    let n = sockfd.read(&mut recvline).expect("cannot read");

    let reply = &recvline[0..n];
    let reply = String::from_utf8(reply.to_vec()).expect("cannot convert");
    println!("\n{:?}\n", reply.trim());
}

/*

#include	"unp.h"

int
main(int argc, char **argv)
{
    int					sockfd, n;
    char				recvline[MAXLINE + 1];
    struct sockaddr_in	servaddr;

    if (argc != 2)
        err_quit("usage: a.out <IPaddress>");

    if ( (sockfd = socket(AF_INET, SOCK_STREAM, 0)) < 0)
        err_sys("socket error");

    bzero(&servaddr, sizeof(servaddr));
    servaddr.sin_family = AF_INET;
    servaddr.sin_port   = htons(13);	/* daytime server */
    if (inet_pton(AF_INET, argv[1], &servaddr.sin_addr) <= 0)
        err_quit("inet_pton error for %s", argv[1]);

    if (connect(sockfd, (SA *) &servaddr, sizeof(servaddr)) < 0)
        err_sys("connect error");

    while ( (n = read(sockfd, recvline, MAXLINE)) > 0) {
        recvline[n] = 0;	/* null terminate */
        if (fputs(recvline, stdout) == EOF)
            err_sys("fputs error");
    }
    if (n < 0)
        err_sys("read error");

    exit(0);
}

*/
