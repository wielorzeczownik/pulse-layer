# Security Policy

## Supported versions

Only the latest release receives security fixes.

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Report vulnerabilities privately via [GitHub Security Advisories](https://github.com/wielorzeczownik/pulse-layer/security/advisories/new).

Include as much detail as possible:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

You will receive a response within **7 days**. If the issue is confirmed, a fix will be released as soon as possible and you will be credited in the release notes (unless you prefer to remain anonymous).

## Scope

This project is a local desktop application. The attack surface includes:

- The local HTTP server serving the overlay widget (bound to `127.0.0.1` by default)
- BLE communication with the smart ring
- Settings file handling

Issues related to the upstream ring firmware or the COLMI protocol are out of scope.

## Security notes for operators

- The overlay server binds to `127.0.0.1` only, do not expose it to external networks.
