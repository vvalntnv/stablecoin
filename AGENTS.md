# Agent Guidelines

## Commands
- **Test (All):** `cd program && yarn test`
- **Test (Single):** `cd program && yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/**/*.ts" -g "test name"`
- **Lint:** `cd program && yarn lint`
- **Build:** `cd program && anchor build`

## Code Style (Rust/Anchor)
- **Formatting:** 4 spaces indent. Run `cargo fmt` before committing.
- **Naming:** `snake_case` for functions/variables, `PascalCase` for structs/enums.
- **Imports:** Group imports: `std`/`core` first, then external crates, then `crate::`.
- **Structure:** Keep instructions in `src/instructions/`, state in `src/state.rs`.
- **Error Handling:** Use `Result<()>` with custom errors in `src/errors.rs`.
- **Anchor:** Use `#[derive(Accounts)]` for account validation structs.

## Rules
- Always run tests before finishing a task.
- Check `program/Anchor.toml` for script definitions.
- Ensure `programs/stablecoin/src/lib.rs` exposes instructions publicly.
