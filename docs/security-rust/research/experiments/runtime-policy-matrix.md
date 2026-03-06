# Runtime Policy Matrix

This note records the public-safe runtime scenarios exercised in v1.

| Scenario | Input | Expected Result | Backing Test |
| --- | --- | --- | --- |
| Oversized guest state | Long synthetic strategy identifier | Runtime policy violation during load | `shared/runtime-security/tests/runtime_security.rs` |
| Malformed guest snapshot | Invalid JSON WAT fixture | Parse failure during load | `shared/runtime-security/tests/runtime_security.rs` |
| Over-budget execution | Busy-loop WAT fixture | Runtime policy violation during execution | `testing/security-labs/runtime-security/tests/runtime_security.rs` |
