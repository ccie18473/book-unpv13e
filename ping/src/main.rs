mod ping;
mod proc_v4;
mod readloop;
mod send_v4;
mod sig_alrm;
mod tv_sub;

mod prelude {
    pub use crate::ping::*;
    pub use crate::proc_v4::*;
    pub use crate::readloop::*;
    pub use crate::send_v4::*;
    pub use crate::sig_alrm::*;
    pub use crate::tv_sub::*;
    pub use bincode::serialize;
    pub use nix::sys::signal;
    pub use nix::unistd::alarm;
    pub use once_cell::sync::OnceCell;
    pub use serde_derive::Serialize;
    pub use socket2::*;
    pub use std::mem::MaybeUninit;
    pub use std::net::SocketAddr;
    pub use std::time::SystemTime;
    pub use std::{env, process::exit};
    pub use std::cell::{RefCell};
    pub use std::rc::Rc;
}

use prelude::*;

static SOCKET: OnceCell<Socket> = OnceCell::new();
static SOCKET_RAW: OnceCell<SockAddr> = OnceCell::new();
static mut SENDER: OnceCell<Sender> = OnceCell::new();

fn main() {
    // get argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("usage: <IPaddress>\n");
        exit(1);
    }
    // string socket (a.b.c.d:port)
    let so_addr_str = format!("{}:{}", args[1], 0);
    // normal socket
    let so_addr: SocketAddr = so_addr_str.parse().expect("cannot parse socket address");
    // raw socket
    let so_addr_raw: SockAddr = so_addr.into();
    // create socket
    let sockfd =
        Socket::new(Domain::IPV4, Type::RAW, Some(Protocol::ICMPV4)).expect("cannot create socket");
    // register signal handling
    let sig_action = signal::SigAction::new(
        signal::SigHandler::Handler(sig_alrm::sig_alrm),
        signal::SaFlags::SA_RESTART,
        signal::SigSet::empty(),
    );
    unsafe {
        signal::sigaction(signal::SIGALRM, &sig_action).unwrap();
    }

    // global variables
    SOCKET_RAW.set(so_addr_raw).unwrap();
    SOCKET.set(sockfd).unwrap();
    // init time struct
    let timeval = Rc::new(RefCell::new(Timeval::new()));
    // init sender
    let sender = Sender::new(timeval.clone());
    unsafe {
        SENDER.set(sender).unwrap();
    }
    // init receiver
    let receiver = Receiver::new(timeval.clone());
    // start loop
    readloop::readloop(receiver);

    exit(0);
}

/*

#include	"ping.h"

struct proto	proto_v4 = { proc_v4, send_v4, NULL, NULL, NULL, 0, IPPROTO_ICMP };

#ifdef	IPV6
struct proto	proto_v6 = { proc_v6, send_v6, init_v6, NULL, NULL, 0, IPPROTO_ICMPV6 };
#endif

int	datalen = 56;		/* data that goes with ICMP echo request */

int
main(int argc, char **argv)
{
    int				c;
    struct addrinfo	*ai;
    char *h;

    opterr = 0;		/* don't want getopt() writing to stderr */
    while ( (c = getopt(argc, argv, "v")) != -1) {
        switch (c) {
        case 'v':
            verbose++;
            break;

        case '?':
            err_quit("unrecognized option: %c", c);
        }
    }

    if (optind != argc-1)
        err_quit("usage: ping [ -v ] <hostname>");
    host = argv[optind];

    pid = getpid() & 0xffff;	/* ICMP ID field is 16 bits */
    Signal(SIGALRM, sig_alrm);

    ai = Host_serv(host, NULL, 0, 0);

    h = Sock_ntop_host(ai->ai_addr, ai->ai_addrlen);
    printf("PING %s (%s): %d data bytes\n",
            ai->ai_canonname ? ai->ai_canonname : h,
            h, datalen);

        /* 4initialize according to protocol */
    if (ai->ai_family == AF_INET) {
        pr = &proto_v4;
#ifdef	IPV6
    } else if (ai->ai_family == AF_INET6) {
        pr = &proto_v6;
        if (IN6_IS_ADDR_V4MAPPED(&(((struct sockaddr_in6 *)
                                 ai->ai_addr)->sin6_addr)))
            err_quit("cannot ping IPv4-mapped IPv6 address");
#endif
    } else
        err_quit("unknown address family %d", ai->ai_family);

    pr->sasend = ai->ai_addr;
    pr->sarecv = Calloc(1, ai->ai_addrlen);
    pr->salen = ai->ai_addrlen;

    readloop();

    exit(0);
}

*/
