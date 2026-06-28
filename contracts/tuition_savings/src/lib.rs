#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Map, Symbol};

/// Data record stored for every tuition savings goal.
/// A single on-chain tuple captures who owns the goal, how much is
/// being saved, when it can be unlocked, and the current balance.
#[contracttype]
#[derive(Clone)]
pub struct Goal {
    pub saver: Address,
    pub target_amount: u64,
    pub target_date: u64,
    pub balance: u64,
    pub created_at: u64,
    pub deposits: u64,
}

#[contract]
pub struct TuitionSavings;

#[contractimpl]
impl TuitionSavings {
    /// Create a new tuition savings goal for `saver`.
    ///
    /// The `goal_id` must be a unique Symbol chosen by the saver
    /// (e.g. `"fall2026_cs_degree"`). `target_amount` is the total
    /// amount the saver wants to accumulate, expressed in the
    /// smallest unit of the chosen token. `target_date` is the
    /// unlock date encoded as a Stellar ledger timestamp (seconds
    /// since the Unix epoch) — once reached, the saver can
    /// withdraw the saved balance toward tuition payment.
    pub fn create_goal(
        env: Env,
        saver: Address,
        goal_id: Symbol,
        target_amount: u64,
        target_date: u64,
    ) {
        saver.require_auth();

        if target_amount == 0 {
            panic!("target_amount must be greater than zero");
        }
        if target_date <= env.ledger().timestamp() {
            panic!("target_date must be in the future");
        }

        let mut goals: Map<Symbol, Goal> = env
            .storage()
            .instance()
            .get(&"goals")
            .unwrap_or(Map::new(&env));

        if goals.contains_key(goal_id.clone()) {
            panic!("goal_id already exists");
        }

        let goal = Goal {
            saver: saver.clone(),
            target_amount,
            target_date,
            balance: 0,
            created_at: env.ledger().timestamp(),
            deposits: 0,
        };

        goals.set(goal_id.clone(), goal);
        env.storage().instance().set(&"goals", &goals);
    }

    /// Record a deposit into the goal identified by `goal_id`.
    ///
    /// Only the saver who created the goal is allowed to deposit.
    /// Returns the new balance after the deposit. The contract does
    /// not move any real XLM — it simply credits the goal's
    /// internal accounting so an off-chain payment rail (or a
    /// later token integration) can reconcile against it.
    pub fn deposit(
        env: Env,
        saver: Address,
        goal_id: Symbol,
        amount: u64,
    ) -> u64 {
        saver.require_auth();

        if amount == 0 {
            panic!("deposit amount must be greater than zero");
        }

        let mut goals: Map<Symbol, Goal> = env
            .storage()
            .instance()
            .get(&"goals")
            .unwrap_or(Map::new(&env));

        let mut goal = goals
            .get(goal_id.clone())
            .unwrap_or_else(|| panic!("goal not found"));

        if goal.saver != saver {
            panic!("only the goal owner can deposit");
        }

        goal.balance += amount;
        goal.deposits += 1;

        let new_balance = goal.balance;
        goals.set(goal_id.clone(), goal);
        env.storage().instance().set(&"goals", &goals);

        new_balance
    }

    /// Withdraw `amount` from the goal after the unlock date.
    ///
    /// Withdrawals are only permitted once `target_date` has been
    /// reached, and only the original saver may call this. Returns
    /// the new remaining balance. Funds cannot be withdrawn before
    /// the unlock date — this keeps the savings locked until the
    /// tuition payment window opens.
    pub fn withdraw(
        env: Env,
        saver: Address,
        goal_id: Symbol,
        amount: u64,
    ) -> u64 {
        saver.require_auth();

        if amount == 0 {
            panic!("withdraw amount must be greater than zero");
        }

        let mut goals: Map<Symbol, Goal> = env
            .storage()
            .instance()
            .get(&"goals")
            .unwrap_or(Map::new(&env));

        let mut goal = goals
            .get(goal_id.clone())
            .unwrap_or_else(|| panic!("goal not found"));

        if goal.saver != saver {
            panic!("only the goal owner can withdraw");
        }

        if env.ledger().timestamp() < goal.target_date {
            panic!("goal is still locked until target_date");
        }

        if amount > goal.balance {
            panic!("insufficient balance");
        }

        goal.balance -= amount;

        let new_balance = goal.balance;
        goals.set(goal_id.clone(), goal);
        env.storage().instance().set(&"goals", &goals);

        new_balance
    }

    /// Return the current saved balance for `goal_id`.
    pub fn get_balance(env: Env, goal_id: Symbol) -> u64 {
        let goals: Map<Symbol, Goal> = env
            .storage()
            .instance()
            .get(&"goals")
            .unwrap_or(Map::new(&env));

        goals
            .get(goal_id)
            .unwrap_or_else(|| panic!("goal not found"))
            .balance
    }

    /// Return whether the goal's unlock date has been reached.
    pub fn is_unlocked(env: Env, goal_id: Symbol) -> bool {
        let goals: Map<Symbol, Goal> = env
            .storage()
            .instance()
            .get(&"goals")
            .unwrap_or(Map::new(&env));

        let goal = goals
            .get(goal_id)
            .unwrap_or_else(|| panic!("goal not found"));

        env.ledger().timestamp() >= goal.target_date
    }

    /// Return progress as a basis-points value (0–10_000) toward
    /// the target. Useful for displaying a progress bar in the
    /// front-end. Saturates at 10_000 once the target is met.
    pub fn get_progress(env: Env, goal_id: Symbol) -> u32 {
        let goals: Map<Symbol, Goal> = env
            .storage()
            .instance()
            .get(&"goals")
            .unwrap_or(Map::new(&env));

        let goal = goals
            .get(goal_id)
            .unwrap_or_else(|| panic!("goal not found"));

        if goal.target_amount == 0 {
            return 10_000;
        }

        let basis = (goal.balance as u128)
            .saturating_mul(10_000)
            / (goal.target_amount as u128);

        if basis > 10_000 {
            10_000
        } else {
            basis as u32
        }
    }
}
