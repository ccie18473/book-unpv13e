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
            println!(
                "receiving from server: {}",
                String::from_utf8(recvline.to_vec()).expect("cannot convert")
            );
        } else if n == 0 {
            break;
        }
    }
}

/*

#include	"unp.h"

void
str_cli(FILE *fp, int sockfd)
{
    char	sendline[MAXLINE], recvline[MAXLINE];

    while (Fgets(sendline, MAXLINE, fp) != NULL) {

        Writen(sockfd, sendline, strlen(sendline));

        if (Readline(sockfd, recvline, MAXLINE) == 0)
            err_quit("str_cli: server terminated prematurely");

        Fputs(recvline, stdout);
    }
}

*/
