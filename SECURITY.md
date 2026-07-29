# Security Policy

## Supported versions

Security fixes are accepted against the latest `main` branch and tagged releases (`0.1.x`) of this repository's crates (`boson-spectra-telemetry`).

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security-sensitive reports.

Prefer one of the following:

1. **GitHub Security Advisories** — use [Report a vulnerability](https://github.com/unified-field-dev/boson-spectra-telemetry/security/advisories/new) on this repository when available.
2. Contact the maintainers privately via the repository owner listed at https://github.com/unified-field-dev/boson-spectra-telemetry.

Include:

- a description of the issue and its impact
- steps to reproduce or a proof of concept when possible
- affected crate names and versions

We will acknowledge receipt as soon as practical and coordinate a fix and disclosure timeline with you.

## Scope

In scope: vulnerabilities in this repository's published crates and documentation that could cause unsafe production defaults, plus CI/supply-chain issues in this repository.

Out of scope: vulnerabilities solely in third-party dependencies unless this project mishandles them in a security-relevant way.

## Telemetry hardening

- Operator-visible error strings are sanitized (credential-like substrings redacted, length
  bounded) before they are persisted in events.
- The ops-log adapter forwards only schema-defined metric/event names with allowlisted labels
  and filtered JSON payloads; high-cardinality identifiers are hashed or dropped.

## Authentication and authorization

Forwards Boson's operational signals (task/job ids, durations, error strings) into Spectra. Caller authentication and authorization for Boson are host concerns.

