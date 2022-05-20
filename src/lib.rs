/*
 *
 * emwa(smoothing factor)
 *
 * emwa = EMWA(0.5)
 *
 * emwa.add(something_var)
 * emwa.add(something_var)
 * emwa.add(something_var)
 * emwa.add(something_var)
 * ...
 *
 * add( decimal )
 * value () decimal
 *
 *
 */

pub struct EMWA {
    alpha: f64,
    value: f64,
    datapoints: u32,
    time: f64,
}

enum Alpha {
    Static,
    Dynamic,
}

impl EMWA {
    pub fn new(alpha: f64) -> Self {
        EMWA {
            alpha,
            value: 0f64,
            datapoints: 0,
            time: 0f64,
        }
    }

    pub fn add(&mut self, data: f64) -> f64 {
        self.datapoints += 1;
        if self.datapoints == 1 {
            self.value = data;
            self.value
        } else {
            let new_value = self.alpha * data + (1f64 - self.alpha) * self.value;
            self.value = new_value;
            self.value
        }
    }

    fn compute_alpha(time: f64) -> f64 {
        let diff = time - self.time;
        (- diff / self.datapoints).exp()
    }

    pub fn add_with_time(&mut self, data: f64, time: f64) -> f64 {
        self.datapoints += 1;
        if self.datapoints == 1 {
            self.value = data;
            self.value
        } else {
            let new_alpha = self.compute_alpha(time);
            let new_value = new_alpha * data + (1f64 - new_alpha) * self.value;
            self.value = new_value;
            self.value
        }
    }
        

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
