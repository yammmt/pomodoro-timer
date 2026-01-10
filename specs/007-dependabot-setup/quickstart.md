# Quickstart: Enable GitHub Dependabot

**Date**: 2026-01-09  
**Feature**: [spec.md](spec.md) | [plan.md](plan.md)

## Overview

Enable GitHub Dependabot to automatically monitor and update npm and Cargo dependencies in the pomodoro_timer project.

## Prerequisites

- GitHub repository with admin access
- Existing `package.json` in repository root
- Existing `Cargo.toml` in `/src-tauri` directory
- GitHub Actions CI configured (spec 002-github-actions-ci)

## Quick Setup (5 minutes)

### 1. Create Configuration File

Create `.github/dependabot.yml` with the following content:

```yaml
version: 2
updates:
  # npm dependencies
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
  
  # Cargo dependencies
  - package-ecosystem: "cargo"
    directory: "/src-tauri"
    schedule:
      interval: "weekly"
      day: "monday"
    versioning-strategy: increase
    open-pull-requests-limit: 10
```

### 2. Commit and Push

```bash
git add .github/dependabot.yml
git commit -m "ci: enable GitHub Dependabot for npm and cargo"
git push origin 007-dependabot-setup
```

### 3. Create Pull Request

Create a PR from branch `007-dependabot-setup` to `main`:

```bash
# Using GitHub CLI
gh pr create --title "Enable GitHub Dependabot" \
  --body "Enables automated dependency updates for npm and Cargo dependencies"

# Or manually via GitHub web interface
```

### 4. Verify Activation

After merging:

1. Go to repository → **Insights** → **Dependency graph** → **Dependabot**
2. Verify both ecosystems (npm, cargo) are listed
3. Check "Last checked" timestamp updates

## What Happens Next

### Immediate (typically within 1 hour)

- GitHub validates the configuration file
- Dependabot appears in the Dependency graph tab
- Initial dependency scan begins

### Within 24 hours (typically)

- Dependabot completes first update check
- Creates PRs for any outdated dependencies (if found)
- PRs trigger existing CI/CD workflows

### Scheduled check (typically every Monday)

- Dependabot runs scheduled update check
- Creates grouped PRs for development and production dependencies
- Closes outdated PRs if newer versions available

## Expected Pull Requests

Dependabot will create PRs with:

- **Title**: "Bump {package} from {old} to {new}"
- **Labels**: `dependencies`, ecosystem labels
- **Content**: Version changes, changelogs, compatibility notes
- **CI Status**: Automatic workflow execution

## Managing Dependabot PRs

### Reviewing a PR

1. Check CI status (must be green)
2. Review version changes and changelogs
3. For major updates: review breaking changes
4. Approve and merge if CI passes

### Handling Failed CI

1. Review CI logs in the PR
2. If breaking changes: update code to fix
3. If incompatible: close PR and investigate
4. If flaky test: re-run CI

### Ignoring Dependencies

To ignore a specific dependency, add to `.github/dependabot.yml`:

```yaml
updates:
  - package-ecosystem: "npm"
    directory: "/"
    # ... existing config ...
    ignore:
      - dependency-name: "package-name"
        versions: ["1.x", "2.x"]  # Ignore specific versions
```

## Configuration Options

### Change Update Frequency

```yaml
schedule:
  interval: "daily"    # Check every day
  # OR
  interval: "monthly"  # Check once per month
```

### Change Day for Weekly Updates

```yaml
schedule:
  interval: "weekly"
  day: "friday"  # Options: monday-sunday
```

### Adjust PR Limits

```yaml
open-pull-requests-limit: 5  # Max concurrent PRs per ecosystem
```

## Troubleshooting

### Dependabot Not Running

**Symptom**: No "Last checked" timestamp or status in Dependency graph

**Solutions**:

1. Verify file path is exactly `.github/dependabot.yml`
2. Check YAML syntax is valid (use YAML linter)
3. Ensure `version: 2` is set correctly
4. Verify directories exist (`/` and `/src-tauri`)

### No Pull Requests Created

**Symptom**: Dependabot runs but creates no PRs

**Possible reasons**:

1. All dependencies are up to date (expected behavior)
2. `open-pull-requests-limit` already reached
3. Updates would violate version constraints in manifest files

### Pull Request CI Failures

**Symptom**: Dependabot PRs fail CI checks

**Solutions**:

1. Review CI logs for specific errors
2. For breaking changes: update code in a separate commit on the PR branch
3. For incompatible updates: close PR and pin dependency version
4. For test flakes: re-run failed workflows

### Too Many Pull Requests

**Symptom**: Overwhelmed by number of Dependabot PRs

**Solutions**:

1. Reduce `open-pull-requests-limit` to lower number
2. Change schedule from weekly to monthly
3. Add more grouping rules to combine related updates
4. Ignore non-critical dependencies

## Security Updates

Dependabot automatically prioritizes security updates:

- **Detection**: Immediate (bypasses schedule)
- **Label**: Automatically tagged with `security`
- **Priority**: Should be reviewed and merged ASAP
- **Notification**: GitHub sends security alert emails

## Further Reading

- [GitHub Dependabot Documentation](https://docs.github.com/en/code-security/dependabot)
- [Configuration Options](https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuration-options-for-the-dependabot.yml-file)
- [Keeping Dependencies Updated](https://docs.github.com/en/code-security/dependabot/working-with-dependabot)

## Success Metrics

After enabling Dependabot:

- ✅ Configuration validates successfully
- ✅ Both ecosystems monitored (npm + cargo)
- ✅ First PRs created within 24 hours (if updates available)
- ✅ All PRs include complete metadata
- ✅ CI automatically validates all Dependabot PRs
- ✅ Weekly updates occur on schedule

## Next Steps

After enabling Dependabot:

1. Monitor initial PRs and merge safe updates
2. Review security updates promptly
3. Adjust configuration based on PR volume
4. Document any ignored dependencies and reasons
