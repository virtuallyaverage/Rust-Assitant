fn main() {
    println!("Hello, world!");
}


use picovoice::{rhino::RhinoInference, PicovoiceBuilder};

let wake_word_callback = || {
    // wake word detected
};

let inference_callback = |inference: RhinoInference| {
    if inference.is_understood {
        let intent = inference.intent.unwrap();
        let slots = inference.slots;
        // take action based on inferred intent and slot values
    } else {
        // handle unsupported commands
    }
};

let mut picovoice = PicovoiceBuilder::new(
    "\${ACCESS_KEY}",
    "\${KEYWORD_FILE_PATH}",
    wake_word_callback,
    "\${CONTEXT_FILE_PATH}",
    inference_callback,
)
.init()
.expect("Failed to create picovoice");