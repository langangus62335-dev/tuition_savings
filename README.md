# tuition_savings

## Project Title
tuition_savings

## Project Description
Tuition is one of the largest lump-sum expenses a student or parent will face, and it is easy to fall short when the payment deadline arrives. **tuition_savings** is an on-chain goal-tracking contract that lets a student or parent create a named savings goal, record periodic deposits, and watch their progress toward a target amount with a hard unlock date. The contract locks the balance until the tuition payment window opens, helping savers resist the temptation to dip into the funds early.

## Project Vision
Our long-term vision is to make higher-education financing as transparent and disciplined as a regular savings account, but with the trust guarantees of a public blockchain. By encoding the goal, the target, and the unlock date directly on Stellar, families and students get a tamper-proof record of how much has been saved and when it can legitimately be spent — paving the way for a future where tuition escrow is the default for every academic term.

## Key Features
- **Goal creation with a target amount and unlock date** — set up `goal_id`, `target_amount`, and `target_date` in a single transaction.
- **Periodic deposit tracking** — every deposit is recorded against the goal, incrementing both the running balance and a deposit counter.
- **Locked withdrawals** — funds cannot be withdrawn before `target_date`, enforcing saving discipline until the tuition payment window opens.
- **Progress read-out** — `get_balance` and `get_progress` (in basis points) make it trivial to render a progress bar in any front-end.
- **Unlock check** — `is_unlocked` lets a UI or external contract query whether the saver is allowed to withdraw yet.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** education dApp — see `contracts/tuition_savings/src/lib.rs` for the full tuition_savings business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** `CCRGVCEALF2N4WQZ6NVH5G2OZAT7X2KBPEL4AEBY4KRU3HRB6H4K2HJY`
- **Explorer template:** `https://stellar.expert/explorer/testnet/tx/c3509447c110712e8457aff913927dec7f1f0545c34bccbd9eeb2a4657469f8c`

## Future Scope
- **Stablecoin integration** — accept USDC deposits natively via Soroban's token interface so the recorded balance is backed by real on-chain assets.
- **Recurring deposit automation** — integrate with scheduled-transaction tools to nudge the saver when a monthly contribution is due.
- **Multi-beneficiary goals** — allow a parent and a student to co-own a goal, with configurable deposit and withdrawal rules.
- **Penalty or reward logic** — incentivize hitting the target by enabling bonus contributions from sponsors, or refunding a small portion of unused balance to the saver after the unlock date.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `tuition_savings` (education)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
