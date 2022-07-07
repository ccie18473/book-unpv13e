mod sum;

use socket2::*;
use sscanf::scanf;
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

            let mut args: sum::Args = sum::Args { arg1: 0, arg2: 0 };
            (args.arg1, args.arg2) = scanf!(input, "{i64} {i64}").unwrap();

            let mut result: sum::Result = sum::Result { sum: 0 };
            result.sum = args.arg1 + args.arg2;

            println!("arg1: {}, arg2: {}", args.arg1, args.arg2);

            sockfd
                .write(&(result.sum.to_be_bytes()))
                .expect("cannot write");
            println!("sending to client: {}", result.sum);
            sockfd.flush().expect("cannot flush");
        } else if n == 0 {
            break;
        }
    }
}

/*

#include	"unp.h"
#include	"sum.h"

void
str_echo(int sockfd)
{
    ssize_t			n;
    struct args		args;
    struct result	result;

    for ( ; ; ) {
        if ( (n = Readn(sockfd, &args, sizeof(args))) == 0)
            return;		/* connection closed by other end */

        result.sum = args.arg1 + args.arg2;
        Writen(sockfd, &result, sizeof(result));
    }
}

*/
