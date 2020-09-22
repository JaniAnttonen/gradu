pub fn run_vdf_worker(
  self,
) -> (Sender<Int>, Receiver<Result<VDFProof, InvalidCapError>>) {
  let (caller_sender, worker_receiver): (Sender<Int>, Receiver<Int>) =
      channel();
  let (worker_sender, caller_receiver) = channel();

  thread::spawn(move || {
      let mut result = self.base.clone();
      let two = Int::from(2);
      let mut iterations: u32 = 0;
      loop {
          result = iter_vdf(result, &self.modulus, &two);
          iterations += 1;

          if iterations == self.upper_bound || iterations == u32::MAX {
              // Upper bound reached, stops iteration and calculates the proof
              debug!(
                  "Upper bound of {:?} reached, generating proof.",
                  iterations
              );

              // Copy pregenerated cap
              let mut self_cap: Int = self.cap.clone();

              // Check if default, check for primality if else
              if self_cap == 0 {
                  self_cap = Generator::new_safe_prime(128);
                  debug!("Cap generated: {:?}", self_cap);
              } else if !self.validate_cap(&self_cap, iterations) {
                  if worker_sender.send(Err(InvalidCapError)).is_err() {
                      error!("Cap not correct!");
                  }
                  break;
              }

              // Generate the VDF proof
              let vdf_result = VDFResult { result, iterations };
              let proof = VDFProof::new(
                  &self.modulus,
                  &self.base,
                  &vdf_result,
                  &self_cap,
              );
              debug!("Proof generated! {:?}", proof);

              // Send proof to caller
              if worker_sender.send(Ok(proof)).is_err() {
                  error!("Failed to send the proof to caller!");
              }

              break;
          } else {
              // Try receiving a cap from the other participant on each iteration
              if let Ok(cap) = worker_receiver.try_recv() {
                  // Cap received
                  debug!("Received the cap {:?}, generating proof.", cap);

                  // Check for primality
                  if self.validate_cap(&cap, iterations) {
                      // Generate the VDF proof
                      let vdf_result = VDFResult { result, iterations };
                      let proof = VDFProof::new(
                          &self.modulus,
                          &self.base,
                          &vdf_result,
                          &cap,
                      );
                      debug!("Proof generated! {:?}", proof);

                      // Send proof to caller
                      if worker_sender.send(Ok(proof)).is_err() {
                          error!("Failed to send the proof to caller!");
                      }
                  } else {
                      error!("Received cap was not a prime!");
                      // Received cap was not a prime, send error to caller
                      if worker_sender.send(Err(InvalidCapError)).is_err()
                      {
                          error!(
                              "Error sending InvalidCapError to caller!"
                          );
                      }
                  }
                  break;
              } else {
                  continue;
              }
          }
      }
  });

  (caller_sender, caller_receiver)
}
