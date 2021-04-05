pub struct ChaoticMap {
    r: f64,
    terms: Vec<f64>,
}

impl ChaoticMap {
    pub fn new(x_0: f64, r: f64) -> Self {
        ChaoticMap {
            r,
            terms: vec![x_0],
        }
    }

    pub fn get_term(&mut self, n: usize) -> f64 {
        while n >= self.terms.len() {
            let last_term = self.terms[self.terms.len() - 1];
            self.terms.push(self.r * last_term * (1 as f64 - last_term));
        }
        return self.terms[n];
    }
}
