use socket2::*;
use sscanf::scanf;
use sprintf::sprintf;
use std::io::prelude::*;
use std::str;

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
            let input = str::from_utf8(&buf[..n]).unwrap().trim();
            let args = scanf!(input, "{i64} {i64}");
            let (arg1, arg2) = args.unwrap();
            let sum = arg1 + arg2;
            println!("arg1: {}, arg2: {}", arg1, arg2);
            let line = sprintf!("%d", sum).unwrap();

            sockfd.write(line.as_bytes()).expect("cannot write");
            println!("sending to client: {}", sum);
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
    long		arg1, arg2;
    ssize_t		n;
    char		line[MAXLINE];

    for ( ; ; ) {
        if ( (n = Readline(sockfd, line, MAXLINE)) == 0)
            return;		/* connection closed by other end */

        if (sscanf(line, "%ld%ld", &arg1, &arg2) == 2)
            snprintf(line, sizeof(line), "%ld\n", arg1 + arg2);
        else
            snprintf(line, sizeof(line), "input error\n");

        n = strlen(line);
        Writen(sockfd, line, n);
    }
}

*/
