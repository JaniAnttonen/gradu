pub fn new(
  modulus: &Int,
  generator: &Int,
  result: &VDFResult,
  cap: &Int,
) -> Self {
  let mut proof = Int::one();
  let mut r = Int::one();
  let mut b: Int;
  let two: &Int = &Int::from(2);

  for _ in 0..result.iterations {
      b = two * &r / cap;
      r = (two * &r) % cap;
      proof = proof.pow_mod(two, modulus) * generator.pow_mod(&b, modulus);
      proof %= modulus;
  }

  VDFProof {
      modulus: modulus.clone(),
      generator: generator.clone(),
      output: result.clone(),
      cap: cap.clone(),
      proof,
  }
}
