use crate::prelude::*;

use crate::{SOCKET, SOCKET_RAW};

#[derive(Debug)]
pub struct Sender {
    pub packet: Icmp,
    pub timeval: Rc<RefCell<Timeval>>,
}

impl Sender {
    pub fn new(timeval: Rc<RefCell<Timeval>>) -> Self {
        Self {
            packet: Icmp::new(),
            timeval,
        }
    }
    pub fn send_v4(&mut self) {
        // send_to
        self.packet.icmp_type = 8;
        self.packet.icmp_code = 0;
        self.packet.icmp_checksum = [0; 2];
        self.packet.icmp_ident = 1;
        self.packet.icmp_seq += 1;
        self.packet.data = [255; 8];

        let mut buf = bincode::serialize(&self.packet).unwrap();
        let buf = crate::cksum_calc(&mut buf);

        SOCKET
            .get()
            .unwrap()
            .send_to(&buf, &SOCKET_RAW.get().unwrap())
            .expect("cannot send raw");

        // time
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_micros();

        self.timeval.borrow_mut().time_send = time;
    }
}

/*

#include	"ping.h"

void
send_v4(void)
{
    int			len;
    struct icmp	*icmp;

    icmp = (struct icmp *) sendbuf;
    icmp->icmp_type = ICMP_ECHO;
    icmp->icmp_code = 0;
    icmp->icmp_id = pid;
    icmp->icmp_seq = nsent++;
    memset(icmp->icmp_data, 0xa5, datalen);	/* fill with pattern */
    Gettimeofday((struct timeval *) icmp->icmp_data, NULL);

    len = 8 + datalen;		/* checksum ICMP header and data */
    icmp->icmp_cksum = 0;
    icmp->icmp_cksum = in_cksum((u_short *) icmp, len);

    Sendto(sockfd, sendbuf, len, 0, pr->sasend, pr->salen);
}

*/
