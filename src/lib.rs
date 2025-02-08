use near_sdk::near_bindgen;
use serde::{Deserialize, Serialize};

#[near_bindgen]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]

pub struct Contract {
    greeting: String,
}

#[near_bindgen]
impl Contract {
    // (Constructor)
    // the contract is first deployed.
    #[init]
    pub fn new(greeting: String) -> Self {
        Self { greeting }
    }

    /// **Getter function to return the greeting**
    pub fn greet(&self) -> String {
        return self.greeting.clone();
    }

    /// **Change the greeting message (payable)**
    #[payable]
    pub fn change_greeting(&mut self, new_greeting: String) -> String {
        self.greeting = new_greeting;
        self.greeting.make_ascii_uppercase();
        return self.greeting.clone();
    }
}
