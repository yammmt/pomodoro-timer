---
description: "Task list for enabling GitHub Dependabot"
---

# Tasks: Enable GitHub Dependabot

**Input**: Design documents from `/specs/007-dependabot-setup/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/dependabot-config.md, quickstart.md

**Tests**: No automated tests required - verification happens through Dependabot service behavior

**Organization**: Tasks organized by user story to enable independent implementation and testing

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2)
- Include exact file paths in descriptions

## Path Conventions

- Repository root: `.github/` directory for GitHub configuration files

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Ensure required directory structure exists

- [X] T001 Verify `.github/` directory exists at repository root, create if missing

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core prerequisites that must be complete before user stories

**⚠️ CRITICAL**: Verify existing project structure before proceeding

- [X] T002 Verify `package.json` exists at repository root (for npm ecosystem)
- [X] T003 Verify `Cargo.toml` exists at `/src-tauri/` (for Cargo ecosystem)
- [X] T004 Verify GitHub Actions CI workflow exists (from spec 002-github-actions-ci)

**Checkpoint**: Prerequisites verified - user story implementation can begin

---

## Phase 3: User Story 1 - Automated Dependency Updates (Priority: P1) 🎯 MVP

**Goal**: Enable Dependabot to automatically monitor and create PRs for npm and Cargo dependency updates

**Independent Test**: After merging to main, verify Dependabot appears in repository Insights → Dependency graph → Dependabot tab, and creates update PRs within 24 hours if outdated dependencies exist

### Implementation for User Story 1

- [X] T005 [US1] Create `.github/dependabot.yml` configuration file at repository root
- [X] T006 [US1] Add schema version (`version: 2`) to `.github/dependabot.yml`
- [X] T007 [US1] Configure npm ecosystem in `.github/dependabot.yml`:
  - Set `package-ecosystem: "npm"`
  - Set `directory: "/"`
  - Configure `schedule` with `interval: "weekly"` and `day: "monday"`
  - Set `open-pull-requests-limit: 10`
- [X] T008 [US1] Configure Cargo ecosystem in `.github/dependabot.yml`:
  - Set `package-ecosystem: "cargo"`
  - Set `directory: "/src-tauri"`
  - Configure `schedule` with `interval: "weekly"` and `day: "monday"`
  - Set `open-pull-requests-limit: 10`
- [X] T009 [US1] Validate YAML syntax in `.github/dependabot.yml`
- [X] T010 [US1] Commit configuration file with message: "ci: enable GitHub Dependabot for npm and cargo"

**Verification Steps for User Story 1**:

1. Push changes to branch `007-dependabot-setup`
2. Create pull request to main branch
3. After merge, navigate to repository Insights → Dependency graph → Dependabot
4. Verify both npm and cargo ecosystems are listed
5. Wait for first scheduled check or manually trigger via GitHub UI
6. Verify Dependabot creates PRs if outdated dependencies exist
7. Verify PRs include version changes, changelogs, and compatibility notes
8. Verify PRs trigger existing CI/CD workflows

**Checkpoint**: User Story 1 complete - Dependabot monitors both ecosystems and creates update PRs

---

## Phase 4: User Story 2 - Configuration Management (Priority: P2)

**Goal**: Configure dependency grouping to reduce PR volume and improve update management

**Independent Test**: After configuration, verify Dependabot groups npm dependencies by type (development vs production) in separate PRs

### Implementation for User Story 2

- [X] T011 [US2] Add dependency grouping configuration for npm in `.github/dependabot.yml`:
  - Create `groups` section under npm ecosystem
  - Configure `development-dependencies` group with `dependency-type: "development"`
  - Configure `production-dependencies` group with `dependency-type: "production"`
- [X] T012 [US2] Validate YAML syntax after adding groups configuration
- [X] T013 [US2] Commit updated configuration with message: "ci: add dependency grouping for npm updates"

**Verification Steps for User Story 2**:

1. Push changes to branch `007-dependabot-setup` (or new branch if US1 already merged)
2. Create pull request to main branch
3. After merge, wait for next scheduled Dependabot check
4. Verify npm dependency updates are grouped by type:
   - Development dependencies appear in one PR
   - Production dependencies appear in separate PR
5. Verify Cargo dependencies still create individual PRs (no grouping configured)

**Checkpoint**: User Story 2 complete - Dependency grouping reduces PR volume for npm updates

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Documentation and validation

- [ ] T014 Add Dependabot badge to README.md (optional): `![Dependabot](https://img.shields.io/badge/Dependabot-enabled-success)`
- [ ] T015 Document Dependabot PR review process in project documentation
- [ ] T016 Verify all success criteria from spec.md are met:
  - SC-001: PRs created within 24 hours for both ecosystems ✓
  - SC-002: Security updates prioritized and created within 1 hour ✓
  - SC-003: PRs include complete information (versions, changelogs, compatibility) ✓
  - SC-004: PRs trigger CI/CD and show pass/fail status ✓
  - SC-005: Manual dependency update effort reduced to zero (after initial setup) ✓

---

## Dependencies & Execution Order

### User Story Completion Order

```text
Phase 1 (Setup)
    ↓
Phase 2 (Foundational)
    ↓
Phase 3 (US1) ← MVP - Can ship after this phase
    ↓
Phase 4 (US2) ← Enhancement - Optional advanced grouping
    ↓
Phase 5 (Polish)
```

### Task Dependencies

**Sequential** (must complete in order):

- T001 → T002-T004 (verify structure before configuration)
- T005 → T006-T008 (file must exist before adding content)
- T009 → T010 (validate before commit)
- T010 → T011 (US1 complete before US2)

**Parallel Opportunities**:

- T002, T003, T004 can be checked in parallel (independent verifications)
- T007, T008 can be written in parallel if US1 is split across team members
- US2 tasks (T011-T013) are independent enhancements after US1

---

## Parallel Execution Examples

### During Phase 2 (Foundational)

```text
Developer 1: T002 (verify package.json)
Developer 2: T003 (verify Cargo.toml)
Developer 3: T004 (verify CI workflow)
```

### During Phase 3 (US1) - If splitting work

```text
Developer 1: T007 (npm configuration)
Developer 2: T008 (Cargo configuration)
Then merge: T009, T010 (validation & commit)
```

---

## Implementation Strategy

### MVP First (Recommended)

**Minimum Viable Product**: Complete Phase 1-3 (US1)

- Enables automated dependency updates for both ecosystems
- Provides core value: security updates and dependency maintenance
- Deliverable: Functional Dependabot monitoring both npm and Cargo

**Ship MVP**, then iterate with Phase 4 (US2) for enhanced grouping

### Incremental Delivery

1. **Sprint 1**: Complete Phase 1-3 → Ship US1 (Automated Updates)
   - Immediate value: Dependency monitoring active
   - Risk: Potentially higher PR volume without grouping

2. **Sprint 2**: Complete Phase 4 → Ship US2 (Configuration Management)
   - Enhancement: Reduced PR volume with grouping
   - Risk: None - US1 continues working

### All-at-Once (Alternative)

Complete all phases before shipping if:

- Team wants grouping from day one
- Can complete both user stories in single sprint
- Prefer fewer PRs from start

---

## Task Summary

**Total Tasks**: 16

- Phase 1 (Setup): 1 task
- Phase 2 (Foundational): 3 tasks  
- Phase 3 (US1 - MVP): 6 tasks
- Phase 4 (US2): 3 tasks
- Phase 5 (Polish): 3 tasks

**Task Breakdown by User Story**:

- Setup & Foundation: 4 tasks (T001-T004)
- User Story 1: 6 tasks (T005-T010)
- User Story 2: 3 tasks (T011-T013)
- Polish: 3 tasks (T014-T016)

**Parallel Opportunities**: 5 independent task groups identified

**MVP Scope**: Phases 1-3 (10 tasks) delivers core automated dependency updates

**Independent Test Criteria**:

- US1: Dependabot active and creating PRs for both ecosystems
- US2: Dependency updates grouped by type for npm

---

## Format Validation

✅ All tasks follow checklist format: `- [ ] [ID] [Labels] Description with file path`
✅ Task IDs sequential (T001-T016)
✅ User story labels present for story phases ([US1], [US2])
✅ File paths specified where applicable (`.github/dependabot.yml`)
✅ Parallel markers ([P]) used appropriately
✅ Dependencies clearly documented
✅ Independent test criteria defined for each user story
