# Agent Guidelines

## Commands

- **Test (All):** `cd program && anchor test`
- **Test (Single):** `cd program && yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/**/*.ts" -g "test name"`
- **Lint:** `cd program && yarn lint` (TS) && `cargo fmt` (Rust)
- **Build:** `cd program && anchor build`

## Code Style (Rust/Anchor)

- **Formatting:** 4 spaces. Run formatters before committing.
- **Naming:** `snake_case` for functions/variables, `PascalCase` for types.
- **Structure:** `lib.rs` exposes instructions; logic lives in `src/instructions/`.
- **Pattern:** Instructions implement a public `process` function called by `lib.rs`.
- **State:** Account structs in `src/state.rs` or defined with instruction if specific.
- **Errors:** Use `Result<()>` and `#[error_code]` in `src/errors.rs`.

## Development Rules

- Verify all tests pass with `anchor test` before finishing.
- Use `glob` and `read` to explore before `edit`.
- Maintain existing import grouping: std/core -> external -> crate.
- Follow the `process` function pattern for new instructions.
