// Copyright (c) 2017-2018, Substratum LLC (https://substratum.net) and/or its affiliates. All rights reserved.
use std::net::IpAddr;
use std::net::SocketAddr;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive (Eq, Hash, Deserialize, Serialize)]
pub struct NodeAddr {
    ip_addr: IpAddr,
    ports: Vec<u16>
}

impl NodeAddr {
    pub fn new (ip_addr: &IpAddr, ports: &Vec<u16>) -> NodeAddr {
        let mut ports = ports.clone();
        ports.sort();
        ports.dedup();

        NodeAddr {
            ip_addr: *ip_addr,
            ports
        }
    }

    pub fn ip_addr (&self) -> IpAddr {
        self.ip_addr
    }

    pub fn ports (&self) -> Vec<u16> {
        self.ports.clone ()
    }
}

impl<'a> From<&'a SocketAddr> for NodeAddr {
    fn from(socket_addr: &'a SocketAddr) -> Self {
        NodeAddr::new (&socket_addr.ip (), &vec! (socket_addr.port ()))
    }
}

impl Into<Vec<SocketAddr>> for NodeAddr {
    fn into(self) -> Vec<SocketAddr> {
        self.ports ().iter().map(| port | {
            SocketAddr::new(self.ip_addr (), *port)
        }).collect()
    }
}

impl Clone for NodeAddr {
    fn clone(&self) -> Self {
        NodeAddr::new(&self.ip_addr (), &self.ports ())
    }
}

impl Debug for NodeAddr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write! (f, "{}:{:?}", self.ip_addr (), self.ports ())
    }
}

impl PartialEq for NodeAddr {
    fn eq(&self, other: &NodeAddr) -> bool {
        self.ip_addr ().eq(&other.ip_addr ()) && self.ports ().eq (&other.ports ())
    }
}

#[cfg (test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn can_create_from_socket_addr() {
        let subject = NodeAddr::from (&SocketAddr::from_str("9.8.7.6:543").unwrap());

        assert_eq!(subject.ip_addr (), IpAddr::from_str("9.8.7.6").unwrap());
        assert_eq!(subject.ports (), vec!(543));
    }

    #[test]
    fn can_convert_to_vector_of_socket_addrs() {
        let ip_addr = IpAddr::from_str("2.5.8.1").unwrap();
        let ports = vec! (9,6);
        let subject = NodeAddr::new(&ip_addr, &ports);

        let result: Vec<SocketAddr> = subject.into ();

        assert_eq! (result, vec! (SocketAddr::from_str("2.5.8.1:6").unwrap (),
            SocketAddr::from_str ("2.5.8.1:9").unwrap ()));
    }

    #[test]
    fn can_clone_node_addr() {
        let ip_addr = IpAddr::from_str("2.5.8.1").unwrap();
        let ports = vec! (9,6);
        let subject = NodeAddr::new(&ip_addr, &ports);

        let result = subject.clone ();

        assert_eq!(result.ip_addr (), ip_addr);
        assert_eq!(result.ports (), vec! (6, 9));
    }

    #[test]
    fn node_addrs_can_be_compared() {
        let a = NodeAddr::new(&IpAddr::from_str("1.2.3.4").unwrap(), &vec!(5,6));
        let b = NodeAddr::new(&IpAddr::from_str("1.2.3.4").unwrap(), &vec!(5,6));
        let c = NodeAddr::new(&IpAddr::from_str("1.2.3.4").unwrap(), &vec!(6,5));
        let d = NodeAddr::new(&IpAddr::from_str("1.2.3.5").unwrap(), &vec!(5,6));
        let e = NodeAddr::new(&IpAddr::from_str("1.2.3.4").unwrap(), &vec!(9));
        let f = NodeAddr::new(&IpAddr::from_str("1.2.3.4").unwrap(), &vec!(5,6,5));

        assert_eq!(a.eq(&a), true);
        assert_eq!(a.eq(&b), true);
        assert_eq!(a.eq(&c), true);
        assert_eq!(a.eq(&d), false);
        assert_eq!(a.eq(&e), false);
        assert_eq!(a.eq(&f), true);
    }

    #[test]
    fn node_addrs_produces_debug_string () {
        let ip_addr = IpAddr::from_str("2.5.8.1").unwrap();
        let ports = vec! (9,6);
        let subject = NodeAddr::new(&ip_addr, &ports);

        let result = format! ("{:?}", subject);

        assert_eq! (result, "2.5.8.1:[6, 9]");
    }
}