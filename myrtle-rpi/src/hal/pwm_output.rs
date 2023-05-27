use libmyrtle::{DataSource, NodeData};

pub struct PwmOutput {
    cur_state: f32,
    channel: i32
}

const PWM_PERIOD : i32 = 1000000;

impl PwmOutput {
    pub fn new(channel : i32) -> PwmOutput {
        std::fs::write("/sys/class/pwm/pwmchip0/export", channel.to_string()).unwrap_or(());

        std::fs::write(
            format!("/sys/class/pwm/pwmchip0/pwm{}/period", channel),
            PWM_PERIOD.to_string()
        ).unwrap_or(());
        std::fs::write(
            format!("/sys/class/pwm/pwmchip0/pwm{}/duty_cycle", channel),
            "0"
        ).unwrap_or(());
        std::fs::write(
            format!("/sys/class/pwm/pwmchip0/pwm{}/enable", channel),
            "1"
        ).unwrap_or(());

        PwmOutput {
            cur_state: 0.0,
            channel,
        }
    }
}

impl Drop for PwmOutput {
    fn drop(&mut self) {
        std::fs::write("/sys/class/pwm/pwmchip0/unexport", self.channel.to_string()).unwrap_or(());
    }
}

impl DataSource for PwmOutput {
    fn poll(&mut self) -> libmyrtle::NodeData {
        return NodeData::Float(self.cur_state);
    }

    fn can_push(&self) -> bool {
        true
    }

    fn push(&mut self, data: libmyrtle::NodeData) -> () {
        let last_state = self.cur_state;

        match data {
            NodeData::Float(f) => self.cur_state = f,
            _ => {}
        }

        if last_state != self.cur_state {
            //println!("Current PWM value: {}", self.cur_state);

            std::fs::write(
                format!("/sys/class/pwm/pwmchip0/pwm{}/duty_cycle", self.channel),
                ((self.cur_state * (PWM_PERIOD as f32)) as i32).to_string()
            ).unwrap_or(());
        }
    }

    fn can_open(&self) -> bool {
        true
    }

    fn open(&mut self) -> () {
        //println!("Opened output pin");
    }

    fn close(&mut self) -> () {
        //println!("Closed output pin");
    }
}
