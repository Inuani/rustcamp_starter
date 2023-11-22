import { createActor } from '../declarations/rust_canister';

async function callBackend() {
    const actor = createActor(); // Add your canister ID and other necessary parameters
    // Use actor to call methods on your canister
  }
  
  callBackend();