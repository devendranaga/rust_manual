struct Timestamp {
    year : u32,
    mon : u32,
    day : u32,
    hour : u32,
    min : u32,
    sec : u32,
}

impl Timestamp {
    fn clear(&mut self) {
        self.year = 0;
        self.mon = 0;
        self.day = 0;
        self.hour = 0;
        self.min = 0;
        self.sec = 0;
    }

    fn print(&self) {
        println!("year: {} mon: {} day: {} hour: {} min: {} sec: {}",
                        self.year, self.mon, self.day, self.hour,
                        self.min, self.sec);
    }
}

struct EventLogData {
    evt_type : u32,
    ts : Timestamp,
}

impl EventLogData {
    fn clear(&mut self) {
        self.evt_type = 0;
        self.ts.clear();
    }

    fn print(&self) {
        println!("evt_type: {}", self.evt_type);
        self.ts.print();
    }

    fn set_type(&mut self, val : u32) {
        self.evt_type = val;
    }
}

fn main() {
    let mut evt_data = EventLogData {
        evt_type : 0,
        ts : Timestamp {
            year : 2018,
            mon : 1,
            day : 1,
            hour : 1,
            min : 1,
            sec: 1,
        },
    };

    println!("before clear\n");
    evt_data.print();
    evt_data.clear();

    println!("after clear\n");
    evt_data.set_type(2);
    evt_data.print();
}
