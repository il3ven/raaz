use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};
use near_sdk::{AccountId, PanicOnDefault, Promise};

const MIN_STAKE: u128 = 5_000_000_000_000_000_000_000_000;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Puzzle {
    question: String,
    solution: String, // Solution is stored as SHA256 hash
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    puzzle: Puzzle,
    total_prize_amount: u128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, question: String, solution: String) -> Self {
        Self {
            owner_id,
            puzzle: Puzzle { question, solution },
            total_prize_amount: 0,
        }
    }

    #[payable]
    pub fn guess_solution(&mut self, guess: String) {
        if env::attached_deposit() < MIN_STAKE {
            env::panic_str("Inssuficient deposit");
        }

        let hashed_guess = env::sha256(guess.as_bytes());
        let hashed_guess_hex = hex::encode(&hashed_guess);
        
        if self.puzzle.solution != hashed_guess_hex {
            self.total_prize_amount += env::attached_deposit();
            env::panic_str("Wrong solution");
        }

        // Correct solution has been gueeseed, transfer the prize
        Promise::new(env::predecessor_account_id()).transfer(self.total_prize_amount);
    }

    pub fn change_puzzle(&mut self, question: String, solution_hash: String) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Only the owner may call this method"
        );

        self.puzzle.question = question;
        self.puzzle.solution = solution_hash;
    }

    pub fn get_ammount_prize(&self) -> u128 {
        self.total_prize_amount
    }

    pub fn get_puzzle(&self) -> String {
        self.puzzle.question.clone()
    }
}

/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    // TESTS HERE
    #[test]
    fn test_new_contract() {
        let alice = AccountId::new_unchecked("alice".to_string());
        let context = get_context(alice.clone());
        testing_env!(context.build());
        let contract = Contract::new(
            alice,
            "What is the capital of France?".to_string(),
            hex::encode(env::sha256("Paris".as_bytes())),
        );
        assert_eq!(contract.get_puzzle(), "What is the capital of France?");
        assert_eq!(contract.total_prize_amount, 0);
    }

    #[test]
    #[should_panic(expected = "Inssuficient deposit")]
    fn test_guess_solution_failure_due_to_insufficient_deposit() {
        let alice = AccountId::new_unchecked("alice".to_string());
        let context = get_context(alice.clone());
        
        testing_env!(context.build());
        let mut contract = Contract::new(
            alice,
            "What is the capital of France?".to_string(),
            hex::encode(env::sha256("Paris".as_bytes())),
        );

        contract.guess_solution("Paris".to_string());
    }

    #[test]
    #[should_panic(expected = "Wrong solution")]
    fn test_guess_solution_failure_due_to_wrong_solution() {
        let alice = AccountId::new_unchecked("alice".to_string());
        let context = get_context(alice.clone());
        
        testing_env!(context.build());
        let mut contract = Contract::new(
            alice,
            "What is the capital of France?".to_string(),
            hex::encode(env::sha256("Paris".as_bytes())),
        );

        contract.guess_solution("London".to_string());
    }

    #[test]
    #[should_panic]
    fn test_guess_solution_success() {
        let alice = AccountId::new_unchecked("alice".to_string());
        let mut context = get_context(alice.clone());
        context.attached_deposit(6_000_000_000_000_000_000_000_000);
        
        testing_env!(context.build());
        let mut contract = Contract::new(
            alice,
            "What is the capital of France?".to_string(),
            hex::encode(env::sha256("Paris".as_bytes())),
        );

        contract.guess_solution("London".to_string());
        contract.guess_solution("New Delhi".to_string());
        // contract.guess_solution("Paris".to_string());
        context.account_locked_balance(12_000_000_000_000_000_000_000_000);
    }
}
