# boson-spectra-telemetry verification

Re-run after code or doc changes. Covered by unit + integration tests below.

## Environment

```bash
export CARGO_BUILD_JOBS=1
export CARGO_TARGET_DIR=target-boson-spectra-telemetry
```

## Unit + integration (CI)

```bash
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test
```

### TEST_MAP

| Behavior | Level | Happy | Sad | Notes |
|----------|-------|-------|-----|-------|
| `sanitize_error_message` | unit | short message preserved | oversize clipped; credential-like substrings redacted | `sanitize::tests` |
| `ops_log` label/payload filters | unit | allowlisted metric/event fields kept | unknown names dropped; high-cardinality ids hashed | `ops_log::tests` |
| `sink_forward::field_str` / `field_i64` | unit | string/bool/number coercions | missing / null / array / bad parse → `""` / `0` | private helpers |
| `install_ops_log_from_env` | integ | `off`/`0`/`false`/`none`, `console`, default Spectra | — | process-wide; under `ENV_LOCK` |
| `SpectraOpsLog` OpsLog methods | integ | counter (incl. fractional), gauge, event | unknown names / empty payload accepted | forwards via `try_*` gate |
| `sink_forward` | integ | known counters + event tables | unknown name ignored; missing fields default | consumer / sink_forward |
| Topic constants | integ | all `*_TOPIC` non-empty `boson_*` | — | Photon wire names |

## Notes

- Integration tests serialize env mutations with `ENV_LOCK`.
- Hosts only call `install_ops_log_from_env`; Boson runtime emits lifecycle signals via
  `OpsLog`. Under Spectra `try_*` gates, adapter calls may no-op when Spectra is not
  configured; assertions focus on non-panic forward paths.
- Sad-path tests are named with `_sad` so audits detect them; they assert concrete
  drop/default behavior, not smoke-only `is_err()`.
