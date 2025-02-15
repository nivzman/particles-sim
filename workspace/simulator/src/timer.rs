use std::ops::Div;
use std::time::Duration;

pub struct Timer {
    sum: Duration,
    count: u32,
    chunk_size_for_avg: u32
}

pub struct ActiveMeasurement<'a> {
    timer: &'a mut Timer,
    start_time: std::time::Instant,
}

impl Timer {
    pub fn new(chunk_size_for_avg: u32) -> Self {
        Timer {
            sum: Duration::from_millis(0),
            count: 0,
            chunk_size_for_avg
        }
    }

    pub fn start(&mut self) -> ActiveMeasurement {
        ActiveMeasurement {
            timer: self,
            start_time: std::time::Instant::now()
        }
    }

    pub fn consume_average_time(&mut self) -> Option<Duration> {
        if self.count != 0 && self.count >= self.chunk_size_for_avg {
            let avg = self.sum.div(self.count);
            self.count = 0;
            self.sum = Duration::from_millis(0);
            Some(avg)
        } else {
            None
        }
    }
}

impl ActiveMeasurement<'_> {
    pub fn end(self) {
        self.timer.count += 1;
        self.timer.sum += self.start_time.elapsed();
    }
}
