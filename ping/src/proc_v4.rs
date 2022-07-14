use crate::prelude::*;
use crate::SOCKET;

#[derive(Debug)]
pub struct Receiver {
    pub packet: Icmp,
    pub timeval: Rc<RefCell<Timeval>>,
}

impl Receiver {
    pub fn new(timeval: Rc<RefCell<Timeval>>) -> Self {
        Self {
            packet: Icmp::new(),
            timeval,
        }
    }
    pub fn proc_v4(&mut self) {
        // receive_from
        let mut buf = [MaybeUninit::<u8>::uninit(); 4096];

        let (n, sa) = SOCKET.get().unwrap().recv_from(&mut buf).unwrap();

        // time 
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();
        self.timeval.borrow_mut().time_receive = time;

        // output
        let rtt = self.timeval.borrow_mut().rtt_calc();
        let peer = sa.as_socket_ipv4().unwrap().ip().clone();

        println!("reply from: {}, rtt (ms): {:.3}", peer, rtt);

        if n > 0 {
        }
    }
}

/*

#include	"ping.h"

void
proc_v4(char *ptr, ssize_t len, struct msghdr *msg, struct timeval *tvrecv)
{
    int				hlen1, icmplen;
    double			rtt;
    struct ip		*ip;
    struct icmp		*icmp;
    struct timeval	*tvsend;

    ip = (struct ip *) ptr;		/* start of IP header */
    hlen1 = ip->ip_hl << 2;		/* length of IP header */
    if (ip->ip_p != IPPROTO_ICMP)
        return;				/* not ICMP */

    icmp = (struct icmp *) (ptr + hlen1);	/* start of ICMP header */
    if ( (icmplen = len - hlen1) < 8)
        return;				/* malformed packet */

    if (icmp->icmp_type == ICMP_ECHOREPLY) {
        if (icmp->icmp_id != pid)
            return;			/* not a response to our ECHO_REQUEST */
        if (icmplen < 16)
            return;			/* not enough data to use */

        tvsend = (struct timeval *) icmp->icmp_data;
        tv_sub(tvrecv, tvsend);
        rtt = tvrecv->tv_sec * 1000.0 + tvrecv->tv_usec / 1000.0;

        printf("%d bytes from %s: seq=%u, ttl=%d, rtt=%.3f ms\n",
                icmplen, Sock_ntop_host(pr->sarecv, pr->salen),
                icmp->icmp_seq, ip->ip_ttl, rtt);

    } else if (verbose) {
        printf("  %d bytes from %s: type = %d, code = %d\n",
                icmplen, Sock_ntop_host(pr->sarecv, pr->salen),
                icmp->icmp_type, icmp->icmp_code);
    }
}

*/
