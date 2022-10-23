struct EventData {
    evt_type : u32,
    evt_len : u32,
    ethertype: u16,
}

struct A {
    a1 : u32,
}

struct B {
    b1 : A,
}

fn main() {
    let evt_msg1 = EventData {
        evt_type : 2,
        evt_len : 100,
        ethertype : 0x0800,
    };

    let evt_msg2 = EventData {
        evt_type : 4,
        evt_len : 101,
        ethertype : 0x0801,
    };

    let mut evt_list : Vec<EventData> = Vec::new();

    evt_list.push(evt_msg1);
    evt_list.push(evt_msg2);

    for evt in evt_list {
        println!("evt_type: {} evt_len: {} ethertype: {}",
                    evt.evt_type, evt.evt_len,
                    evt.ethertype);
    }

    let b1 = B {
        b1 : A { a1 : 4 }
    };

    println!("a: {}", b1.b1.a1);
}
