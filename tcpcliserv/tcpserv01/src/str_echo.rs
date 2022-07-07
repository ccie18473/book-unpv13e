use socket2::*;
use std::io::prelude::*;

const MAXLINE: usize = 4096;

pub fn str_echo(sockfd: &mut Socket) {

    loop {
        let mut buf = [0; MAXLINE];
        let n = sockfd.read(&mut buf).expect("cannot read");

        if n > 0 {
            println!(
                "received from client: {}",
                String::from_utf8(buf.to_vec()).expect("cannot convert")
            );
            sockfd.write(&mut buf[..n]).expect("cannot write");
            println!(
                "sending to client: {}",
                String::from_utf8(buf.to_vec()).expect("cannot convert")
            );
            sockfd.flush().expect("cannot flush");
        } else if n == 0 {
            break;
        }
    }
}

/*

#include	"unp.h"

void
str_echo(int sockfd)
{
    ssize_t		n;
    char		buf[MAXLINE];

again:
    while ( (n = read(sockfd, buf, MAXLINE)) > 0)
        Writen(sockfd, buf, n);

    if (n < 0 && errno == EINTR)
        goto again;
    else if (n < 0)
        err_sys("str_echo: read error");
}

*/
