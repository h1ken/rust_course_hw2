struct _Thermometer {
    current_temp: i32,
}

impl _Thermometer {
    fn _get_temperature(&self) -> i32 {
        self.current_temp
    }
}

struct _Socket {
    status: bool,
    power: i32,
    description: String,
}

impl _Socket {
    fn _get_description(&self) -> String {
        todo!()
    }

    fn _switch(mut self) {
        self.status = !self.status;
    }

    fn _get_consuming_power(&self) -> i32 {
        self.power
    }
}

fn main() {
    println!("Hello, world!");
}
