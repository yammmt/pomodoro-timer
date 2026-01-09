# Dependabot Configuration Contract

**Date**: 2026-01-09  
**Feature**: [spec.md](../spec.md) | [plan.md](../plan.md)

## Overview

This document defines the structure and requirements for the Dependabot configuration file (`.github/dependabot.yml`) that will be created for this project.

## Configuration File Location

**Path**: `.github/dependabot.yml`  
**Format**: YAML  
**Schema Version**: 2

## Required Configuration Structure

```yaml
version: 2
updates:
  # npm ecosystem configuration
  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "weekly"
      day: "monday"
    versioning-strategy: increase
    groups:
      development-dependencies:
        dependency-type: "development"
      production-dependencies:
        dependency-type: "production"
    open-pull-requests-limit: 10
  
  # Cargo ecosystem configuration  
  - package-ecosystem: "cargo"
    directory: "/src-tauri"
    schedule:
      interval: "weekly"
      day: "monday"
    versioning-strategy: increase
    open-pull-requests-limit: 10
```

## Field Specifications

### Global Configuration

| Field | Type | Required | Description | Value |
|-------|------|----------|-------------|-------|
| `version` | number | Yes | Dependabot config schema version | `2` |
| `updates` | array | Yes | List of ecosystem configurations | See below |

### Update Configuration (per ecosystem)

| Field | Type | Required | Description | Values |
|-------|------|----------|-------------|--------|
| `package-ecosystem` | string | Yes | Package manager to monitor | `"npm"`, `"cargo"` |
| `directory` | string | Yes | Directory with package manifest | `"/"` (npm), `"/src-tauri"` (cargo) |
| `schedule` | object | Yes | Update check schedule | See schedule spec |
| `schedule.interval` | string | Yes | Check frequency | `"weekly"` |
| `schedule.day` | string | No | Day for weekly checks | `"monday"` |
| `versioning-strategy` | string | No | How versions are proposed | `"increase"` |
| `groups` | object | No | Dependency grouping rules | See grouping spec |
| `open-pull-requests-limit` | number | No | Max concurrent PRs | `10` (default: 5) |

### Grouping Configuration (optional)

| Field | Type | Required | Description | Values |
|-------|------|----------|-------------|--------|
| `{group-name}` | object | Yes | Named group configuration | Custom name |
| `dependency-type` | string | No | Filter by dependency type | `"development"`, `"production"` |
| `patterns` | array | No | Package name patterns | Array of glob patterns |

## Behavioral Contracts

### Update Detection

- **Trigger**: Weekly on Monday (configurable via `schedule.day`)
- **Scope**: All dependencies in specified `directory`
- **Types**: Both manifest and lockfile updates
- **Constraints**: Must respect semantic version ranges in manifests

### Pull Request Creation

- **Frequency**: As updates are detected, up to `open-pull-requests-limit`
- **Grouping**: By `dependency-type` (development vs production) for npm
- **Content**: Must include version changes, changelogs, and compatibility notes
- **Auto-close**: Superseded PRs closed automatically when newer versions available

### Security Updates

- **Priority**: Security updates processed immediately (bypass schedule)
- **Detection**: GitHub Security Advisory database

- **Labeling**: Automatically labeled with `security` tag

### Version Handling

- **Major versions**: Update manifest file (e.g., `"1.0"` → `"2.0"`)
- **Minor/Patch**: May be lockfile-only if within manifest range
- **Preference**: Manifest updates preferred when possible (per user requirement)

## Integration Points

### GitHub Actions CI

- **Trigger**: All Dependabot PRs trigger existing workflows
- **Required**: CI must pass for PR to be merge-ready
- **Workflow**: Same workflows as manual PRs (defined in spec 002-github-actions-ci)

### Pull Request Metadata

- **Title Format**: "Bump {package} from {old-version} to {new-version}"
- **Labels**: `dependencies`, ecosystem-specific labels, `security` if applicable
- **Body Content**:
  - Release notes
  - Changelog

  - Commits between versions
  - Compatibility score

## Validation Rules

### Configuration Validation

1. File must be valid YAML syntax
2. `version` field must equal `2`
3. `updates` array must have at least one element
4. Each update must specify valid `package-ecosystem` and `directory`
5. `schedule.interval` must be one of: `"daily"`, `"weekly"`, `"monthly"`

### Runtime Validation (by GitHub)

1. Directory paths must exist in repository

2. Package manifests must exist at specified paths:
   - npm: `package.json` at `/`
   - cargo: `Cargo.toml` at `/src-tauri`
3. Update schedule must be valid cron-compatible expression

## Error Handling

### Configuration Errors

- **Invalid YAML**: Dependabot fails to start, error shown in Security tab
- **Invalid path**: Warning in Dependabot logs, ecosystem skipped
- **Invalid ecosystem**: Configuration ignored, other ecosystems continue

### Runtime Errors

- **Network failures**: Retry with exponential backoff
- **Dependency resolution conflicts**: Create PR with failure details
- **API rate limits**: Defer checks until limit resets

## Example Pull Request

A Dependabot PR will include:

```markdown
Bump serde from 1.0.195 to 1.0.196 in /src-tauri

Bumps [serde](https://github.com/serde-rs/serde) from 1.0.195 to 1.0.196.
- [Release notes](https://github.com/serde-rs/serde/releases)
- [Commits](https://github.com/serde-rs/serde/compare/v1.0.195...v1.0.196)

---
updated-dependencies:
- dependency-name: serde
  dependency-type: direct:production
  update-type: version-update:semver-patch
...
```

## Success Criteria

1. Configuration file passes GitHub's validation
2. Both npm and cargo ecosystems are monitored
3. First update PRs created within 24 hours of enabling (if updates available)
4. PRs include complete metadata (versions, changelogs, compatibility)
5. Existing CI/CD validates all Dependabot PRs automatically
