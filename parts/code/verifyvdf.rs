pub fn verify(&self) -> bool {
  // Check first that the result isn't larger than the RSA modulus
  if self.proof > self.modulus {
      return false;
  }
  let r =
      Int::from(2).pow_mod(&Int::from(self.output.iterations), &self.cap);
  self.output.result
      == (self.proof.pow_mod(&self.cap, &self.modulus)
          * self.generator.pow_mod(&r, &self.modulus))
          % &self.modulus
}
