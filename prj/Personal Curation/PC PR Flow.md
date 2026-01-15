# PC PR Flow {SIZE}

**Objectives:**  
- Provide an efficient flow for allowing the user to review every line of Claude code exactly once as it is added to main.
- Minimize touch points where user needs to swap back to interact w/ Claude during development.



**Key Constraint**:  The user is doing other things, so Claude always needs to alert the user when it has stopped either by:
1. **Surf a PR** — Notified the user a new PR is ready via browser tab (see below) OR
2. **Call `alert`** — Notify the user that User gets notified via popup (see below)


## The PR FLOW Procedure

**PRECONDITIONS**
- The repository is clean (no uncommitted changes). Otherwise, commit them.
- No open PRs on this repo
- The roadmap is ready (see [[PC Roadmap Ready]])
**PR FLOW PROCEDURE**
1. **PR flow** — User says "PR flow", Claude finds next incomplete roadmap item, user goes to do other work
2. **Claude Work** — Claude creates feature branches (if needed), does work on `work` branch
3. **PR & Surf** — Claude PRs `work` → `feature`, merges, surfs the **Files tab**
4. **Review** — User reviews files, provides feedback if needed
5. **Iterate** — If fixes needed, go to step 2
6. **Complete** — When feature done, Claude PRs `feature` → `main`, squash merges, surfs Files tab (no wait), deletes branches, continues to next feature


## Claude Work Details

During step 2 (Claude Work), Claude should batch small milestones together:

**Batching Rule**: If a milestone is completed and the PR would have fewer than **{SIZE}** total edited lines, continue to the next milestone on the same branch. Keep working until the PR has enough substance.

**Default SIZE**: 300 lines

**Custom SIZE**: If user says "PR flow 500", use 500 as the target instead of 300.

- The branch name doesn't need to match all milestones — it's just a workspace
- The merge commit message (feature → main) will have the correct final name
- Git history accuracy comes from the squash merge, not the branch name
- Total edited lines includes code, tests, and documentation — all changes are reviewed

**Example**: If M3.9 only needed 12 lines added, continue to M3.10 on the same branch. The feature → main PR can be titled "M3.9-M3.10: Trace Recording and Scoring" or just use the final milestone name.

---

# Notes

## Roadmap Ordering

If the roadmap ordering no longer seems correct (e.g., a later item should be done first, or dependencies have changed), alert the user before proceeding. (See [[PC Roadmap Ready]] for details)

The user will be saying "PR flow" repeatedly to drive progress — so don't blindly follow a stale ordering.

## Alert on Wait

If Claude needs to stop work and wait for user feedback, but has NOT surfed a PR (i.e., the user won't be automatically notified), Claude MUST call the alert command:

```bash
alert "Waiting for: <reason>"
```

This pops up on the user's screen so they know to check Claude's interface.

Examples:
- `alert "Roadmap reordering needed"`
- `alert "Need clarification on feature scope"`
- `alert "Encountered blocking issue"`

---

## Why This Works

- PRing `work` → `feature` repeatedly shows clean incremental diffs
- User reviews familiar PR Files tab interface
- Squash merge keeps main history clean (one commit per feature)
- User only needs to: say "PR flow", review files, give feedback


### Alert command

If Claude needs to ask a question, get clarification, or wait for any reason and there's no PR to surf, it MUST call:
```bash
alert "Waiting for: <reason>"
```

This ensures the user is always notified when Claude needs attention.

---

### Branch Structure

```
main
 └── feature/{name}
      └── feature/{name}/work   ← all work happens here
```

## Implementation Details

### Starting a Feature

```bash
git checkout main
git pull
git checkout -b feature/{name}
git checkout -b feature/{name}/work
```

### Each Review Cycle

1. Commit work to `work` branch
2. Create PR from `work` → `feature`:
   ```bash
   gh pr create --base feature/{name} --head feature/{name}/work --title "Work on {name}: description" --body "..."
   ```
3. Merge the PR:
   ```bash
   gh pr merge --merge
   ```
4. Surf the Files tab:
   ```bash
   ctrl surf "https://github.com/{owner}/{repo}/pull/{number}/files"
   ```

Each PR shows only the delta since last merge (clean incremental diff).

### PR Naming Convention

- **work → feature PRs**: Title starts with "Work on" (e.g., "Work on M3.1: implement Bio.run()")
- **feature → main PRs**: Clean title without "Work on" (e.g., "M3.1: Scenario Execution")

This makes it easy to distinguish at a glance:
- "Work on..." = incremental work PR (for user review)
- Clean title = final merge to main (already reviewed, just completing the feature)

The "Work on" commits disappear when we squash merge to main — only the clean feature title appears in main's history.

### Completing a Feature

1. Create PR from `feature` → `main`:
   ```bash
   gh pr create --base main --head feature/{name} --title "{name}: {description}" --body "..."
   ```
2. Squash merge with descriptive commit message:
   ```bash
   gh pr merge --squash
   ```
3. Surf the Files tab (user can see what went into main, but no wait — already reviewed)
4. Delete branches:
   ```bash
   git checkout main
   git pull
   git branch -d feature/{name}/work feature/{name}
   git push origin --delete feature/{name}/work feature/{name}
   ```
5. Continue immediately to next feature or task

