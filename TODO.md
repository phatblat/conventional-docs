# Todo

- Session: 01a078c4-aafc-76b6-891b-d36e846641be
- Synced: 2026-09-06T23:36:19Z
- Plan: `PLAN.md`

## Steps

- [x] Accept the decision and append the reciprocal erratum
- [x] Write `PLAN.md`
- [ ] Move the crate to `crates/conventional-docs` and fix the Cargo metadata
- [ ] Reshape the CLI into noun-first artifact groups
- [ ] Rename the `dec` module, doc comments, and error prefix
- [ ] Rewrite the tests for the new argv shape, plus the alias and verbless
      cases
- [ ] Run `cargo fmt`, `clippy`, and `test`
- [ ] Rename the tool in `README.md`, `ROADMAP.md`, and `CHANGELOG.md`
- [ ] Move the site page to `/doc/` and rewrite its command table
- [ ] Run `just check` and smoke-test `target/debug/doc`
