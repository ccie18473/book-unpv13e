use socket2::*;
use std::io::prelude::*;
use std::io::Stdin;

const MAXLINE: usize = 4096;

pub fn str_cli(fd: &Stdin, sockfd: &mut Socket) {
    loop {
        let mut recvline = [0; MAXLINE];
        let mut sendline = String::new();
        let n = fd.read_line(&mut sendline).expect("cannot read from stdin");
        if n > 0 {
            sockfd
                .write(&mut sendline[..n].as_bytes())
                .expect("cannot write");
            println!("sending to server: {}", sendline);
            sockfd.flush().expect("cannot flush");

            sockfd.read(&mut recvline).expect("cannot read");
            let mut sum: [u8; 8] = [0; 8];
            sum.copy_from_slice(&recvline[..8]);
            println!("receiving from server: {}", i64::from_be_bytes(sum));
        } else if n == 0 {
            break;
        }
    }
}

/*

#include	"unp.h"
#include	"sum.h"

void
str_cli(FILE *fp, int sockfd)
{
    char			sendline[MAXLINE];
    struct args		args;
    struct result	result;

    while (Fgets(sendline, MAXLINE, fp) != NULL) {

        if (sscanf(sendline, "%ld%ld", &args.arg1, &args.arg2) != 2) {
            printf("invalid input: %s", sendline);
            continue;
        }
        Writen(sockfd, &args, sizeof(args));

        if (Readn(sockfd, &result, sizeof(result)) == 0)
            err_quit("str_cli: server terminated prematurely");

        printf("%ld\n", result.sum);
    }
}

*/
