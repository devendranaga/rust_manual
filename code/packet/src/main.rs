struct EthHdr {
    dest_addr : [u8; 6],
    src_addr : [u8; 6],
    ethertype : u16,
}

fn memcpy(dst: &mut [u8], src: &[u8], len: usize) {
    let mut i : usize = 0;

    while i < len {
        dst[i] = src[i];
        i += 1;
    }
}

fn htons(val: &u16) -> u16 {
    return ((val & 0x00FF) << 8) | ((val & 0xFF00) >> 8);
}

fn frame_eth_hdr(addr1 : &[u8], addr2 : &[u8], ethertype : &u16) -> EthHdr {
    let mut eh = EthHdr {
        dest_addr : [0; 6],
        src_addr : [0; 6],
        ethertype : 0,
    };

    memcpy(&mut eh.dest_addr, &addr1, addr1.len());
    memcpy(&mut eh.src_addr, &addr2, addr2.len());
    eh.ethertype = htons(ethertype);

    eh
}

fn print_macaddr(string : String, addr1: &[u8]) {
    let mut i : usize = 0;

    print!("{} : ", string);
    while i < addr1.len() {
        print!("{}", addr1[i]);
        if i != 5 {
            print!("{}", ":");
        }

        i += 1;
    }
    println!("");
}

fn main() {
    let addr1 : [u8; 6] = [0, 1, 2, 3, 4, 5];
    let addr2 : [u8; 6] = [1, 1, 2, 2, 2, 1];
    let ethertype : u16 = 0x0800;
    let eh : EthHdr;

    eh = frame_eth_hdr(&addr1, &addr2, &ethertype);
    print_macaddr("dest_addr".to_string(), &eh.dest_addr);
    print_macaddr("src_addr".to_string(), &eh.src_addr);
    println!("ethertype: {:#06x}", eh.ethertype);
}
