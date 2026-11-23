# Security

Thanks for taking the time to look. Please report vulnerabilities privately through [GitHub Security Advisories](https://github.com/wielorzeczownik/pulse-layer/security/advisories/new) instead of a public issue, so there's a chance to fix them first. You'll be credited in the release notes unless you'd rather not be. Only the latest release gets fixes.

## In scope

- the overlay server on `127.0.0.1:9000` (the page and its WebSocket)
- the self-updater: fetching `latest.json` and checking release signatures
- BLE communication with the ring
- reading and writing the settings file

Problems in ring firmware or the ring's BLE protocol itself are out of scope.
