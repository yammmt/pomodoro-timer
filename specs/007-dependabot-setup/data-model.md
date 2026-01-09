# Data Model: Enable GitHub Dependabot

**Date**: 2026-01-09  
**Feature**: [spec.md](spec.md) | [plan.md](plan.md)

## Overview

This feature involves configuration data only. No application data models are required. Dependabot's operational data (update checks, PR state) is managed entirely by GitHub's infrastructure.

## Configuration Entities

### Dependabot Configuration

**Purpose**: Defines how Dependabot monitors and updates dependencies

**Structure** (YAML format):

- **version**: Schema version (always "2")
- **updates**: Array of ecosystem configurations

### Update Configuration (per ecosystem)

**Purpose**: Defines update behavior for a specific package ecosystem

**Attributes**:

- **package-ecosystem**: Ecosystem type ("npm" or "cargo")
- **directory**: Path to package manifest relative to repository root
- **schedule**: Update check schedule configuration
  - **interval**: Frequency of checks ("daily", "weekly", "monthly")
  - **day** (optional): Day of week for weekly schedule
  - **time** (optional): Time of day for checks
- **groups** (optional): Dependency grouping rules
  - **group-name**: Custom group identifier
    - **dependency-type**: Type filter ("development" or "production")
    - **patterns** (optional): Package name patterns to group

### Dependency Update (Runtime Entity - managed by GitHub)

**Purpose**: Represents a proposed dependency update in a pull request

**Attributes** (informational only, not stored in repository):

- **dependency-name**: Name of the dependency
- **current-version**: Currently installed version
- **target-version**: Proposed update version
- **update-type**: Type of update ("version-update:semver-major", "version-update:semver-minor", "version-update:semver-patch")
- **compatibility-score**: GitHub's assessment of breaking change risk
- **changelog**: Release notes and change information
- **cve-identifiers** (if security update): List of CVE IDs addressed

## Relationships

```text
Repository (1) ──┬──> (1) Dependabot Configuration
                 │
                 ├──> (1+) npm Update Configuration
                 │
                 └──> (1+) Cargo Update Configuration

Dependabot Service ──> (0+) Dependency Update PRs
```

## Validation Rules

### Configuration Validation

- **version** must be "2"
- **package-ecosystem** must be valid ecosystem identifier
- **directory** must exist in repository
- **schedule.interval** must be "daily", "weekly", or "monthly"
- Group names must be unique within an ecosystem configuration

### Semantic Versioning Constraints

- Updates must respect version constraints in manifest files
- Major version updates may require manifest changes
- Minor/patch updates within range can be lockfile-only

## State Transitions

Dependabot operational states (managed by GitHub):

1. **Idle** → Check scheduled → **Checking**
2. **Checking** → Updates found → **Creating PR**
3. **Checking** → No updates → **Idle**
4. **Creating PR** → PR created → **Awaiting Review**
5. **Awaiting Review** → Tests fail → **Failed** (marked in PR)
6. **Awaiting Review** → Approved & merged → **Applied**
7. **Awaiting Review** → Newer version available → **Superseded** (PR closed)

## Notes

- All operational data is managed by GitHub - no application database required
- Configuration is declarative and stateless
- Dependabot reads configuration on each check cycle
- Changes to configuration take effect on next scheduled check
