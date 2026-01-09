# Research: Enable GitHub Dependabot

**Date**: 2026-01-09  
**Feature**: [spec.md](spec.md) | [plan.md](plan.md)

## Research Tasks

### 1. Update Schedule Preferences

**Question**: What update schedule should be configured - daily, weekly, or monthly?

**Decision**: Weekly updates

**Rationale**:

- **Daily**: Too frequent for a small project, could create PR fatigue and interrupt development flow
- **Weekly**: Balanced approach - catches security updates within reasonable timeframe while not overwhelming with PRs
- **Monthly**: Too infrequent, security vulnerabilities could remain unpatched for extended periods

**Alternatives considered**:

- Daily updates: Rejected due to potential PR overload for small team/solo developer
- Monthly updates: Rejected due to slower security patch adoption

### 2. GitHub Dependabot Configuration Best Practices

**Question**: What are the recommended configuration options for a Tauri desktop application?

**Decision**: Configure separate package ecosystems with appropriate settings

**Rationale**:

- **Separate npm and cargo configurations**: Each ecosystem has different update patterns and requirements
- **Enable version updates**: Keeps dependencies current with bug fixes and features
- **Enable security updates**: Automatically enabled by GitHub, provides faster response to CVEs
- **Group updates by dependency type**: Can reduce PR volume by grouping related updates

**Configuration Structure**:

```yaml
version: 2
updates:
  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "weekly"
    groups:
      development-dependencies:
        dependency-type: "development"
      production-dependencies:
        dependency-type: "production"
  
  - package-ecosystem: "cargo"
    directory: "/src-tauri"
    schedule:
      interval: "weekly"
```

**Best Practices Identified**:

1. Set appropriate directory paths for each ecosystem
2. Use grouping to reduce PR volume
3. Weekly schedule balances security and workload
4. Allow both lockfile-only and manifest updates (default behavior)

### 3. Integration with Existing CI/CD

**Question**: How will Dependabot PRs interact with existing GitHub Actions CI/CD?

**Decision**: Rely on existing CI/CD pipelines to validate Dependabot PRs

**Rationale**:

- Dependabot PRs automatically trigger existing workflows (identified in spec 002-github-actions-ci)
- CI must pass before PRs can be merged
- No additional configuration needed
- Broken updates will be identified by CI failures

**Alternatives considered**:

- Creating separate CI workflow for Dependabot: Unnecessary complexity, existing CI is sufficient
- Auto-merge on passing tests: Risky without human review, rejected for safety

### 4. Handling Lockfile vs Manifest Updates

**Question**: Should Dependabot be configured to update only lockfiles or both lockfiles and manifests?

**Decision**: Allow both types of updates (default behavior)

**Rationale**:

- User preference stated: manifest updates preferred when possible, lockfile-only updates acceptable
- Dependabot's default behavior: attempts manifest updates for major versions, lockfile-only for minor/patch within range
- This matches user's stated preference
- No custom configuration needed to achieve desired behavior

**Configuration**: No special flags needed - Dependabot's default behavior matches requirements

## Summary

All NEEDS CLARIFICATION items resolved:

- ✅ **Constraints**: Weekly update schedule chosen for balance between security responsiveness and PR management
- ✅ **Configuration approach**: Separate ecosystem configs for npm and cargo with grouping enabled
- ✅ **CI integration**: Existing workflows will validate all Dependabot PRs
- ✅ **Update types**: Default behavior (both lockfile and manifest updates) matches requirements

Ready to proceed to Phase 1.
