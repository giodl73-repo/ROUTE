# Security Policy

## Reporting a vulnerability

Please report suspected vulnerabilities through GitHub's private vulnerability
reporting for this repository. Do not open a public issue with exploit details,
credentials, private data, or an unpatched proof of concept.

If private vulnerability reporting is unavailable, email
`giodl73@gmail.com` with the repository name and a concise impact summary.

## Supported versions

Security fixes target the default branch. No older release line is currently
supported.

## Scope

ROUTE fetches and parses public transportation data, generates artifacts, and
runs local analysis tooling. Reports should identify whether an issue affects
credential handling, path traversal, archive extraction, source integrity,
generated artifact trust, or unintended network/file access.
