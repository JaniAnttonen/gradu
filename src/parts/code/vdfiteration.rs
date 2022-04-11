impl Iterator for VDF {
    type Item = VDFResult;
    fn next(&mut self) -> Option<VDFResult> {
        if self.result.iterations < self.upper_bound {
            self.result.iterations += 1;
            self.result.result = self.result.result.clone().next_square();
            Some(self.result.clone())
        } else {
            None
        }
    }
}
