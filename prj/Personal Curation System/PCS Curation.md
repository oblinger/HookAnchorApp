
## FILE STRUCTURE

Template variables:
- `{FULL_NAME}` = folder name (the full descriptive name)
- `{TLC}` = short code (if it exists, typically 2-5 uppercase letters)
- `{NAME}` = TLC if it exists, otherwise FULL_NAME
- `{repo}` = repository name

### Complete Folder Structure
```
{FULL_NAME}/                       # Anchor root folder (full descriptive name)
├── {FULL_NAME}.md                 # Redirect: contains only "See [[{TLC}]]" (if TLC exists)
│
├── {NAME} Docs/                   # Private docs (NOT published)
│   ├── {NAME}.md                  # PRIMARY ANCHOR PAGE (link table, overview)
│   ├── {NAME} PRD.md              # Product Requirements Document
│   ├── {NAME} Features.md         # Feature designs (dated sections)
│   ├── {NAME} Notes.md            # Discussion notes (dated sections)
│   ├── {NAME} Roadmap.md          # Milestone-based task tracking
│   └── {NAME} Todo.md             # Short-term tasks (dated sections)
│
├── {NAME} Research/               # Research materials (optional)
│   ├── {NAME} References.md       # Bibliography
│   └── {NAME} Related Work.md     # Analysis of related work
│
└── {repo}/                        # Repository clone (PUBLISHABLE)
    ├── .git/
    ├── README.md                  # Brief description, installation, quick start
    ├── CLAUDE.md                  # Claude Code project instructions
    ├── pyproject.toml             # Project metadata and dependencies
    ├── mkdocs.yml                 # MkDocs configuration
    ├── justfile                   # Task runner commands
    │
    ├── src/{package}/             # Source code
    ├── tests/                     # Test files
    │
    ├── docs/                      # Documentation SOURCE (hand-written + generated)
    │   ├── index.md               # Docs home page
    │   ├── user-guide/            # Task-oriented tutorials
    │   ├── architecture/          # System design docs
    │   └── api/                   # Generated API reference
    │
    └── site/                      # Generated docs site (gitignored)
```

### Concrete Example (with TLC)
Folder `Alien Biology/` contains:
- `Alien Biology.md` — says `See [[ABIO]]`
- `ABIO Docs/ABIO.md` — the main anchor page with link table
- `ABIO Docs/` — private planning/design docs
- `alienbio/` — the repository (publishable)

### Concrete Example (without TLC)
Folder `My Simple Project/` contains:
- `My Simple Project Docs/My Simple Project.md` — the main anchor page
- `My Simple Project Docs/` — private planning/design docs
- `my-simple-project/` — the repository (publishable)

### Anchor Folder Definition
- An **anchor** is a folder that contains a `{NAME} Docs/` subfolder with the primary anchor markdown
- The primary anchor markdown (`{NAME}.md`) lives inside the Docs folder
- If the anchor has a TLC, the root folder also has `{FULL_NAME}.md` containing only `See [[TLC]]`
- Example with TLC: `.../Alien Biology/ABIO Docs/ABIO.md` — primary anchor page
- Example without TLC: `.../My Project/My Project Docs/My Project.md` — primary anchor page

### Root Folder vs Repository
- The anchor root folder has a `{NAME} Docs/` subfolder for private documentation (NOT published)
- If the project has a code repository, it is a **subdirectory** of the anchor folder
- The subdirectory name matches the GitHub repository name (since it's a clone)
- This separation ensures planning docs don't accidentally get committed to the repo

---

# GENERAL CURATION RULES

## CURATION ACTIONS

- **CODE FLOW** --

## DEFINED TERMS

- **DATED FOLDER** — A folder where all files and subfolders within it are prefixed with the date format (`YYYY-MM-DD`). Items sort in chronological order when viewed alphabetically.

- **DATED SECTIONS** — A file composed of H2 sections whose titles begin with a date prefix (`## YYYY-MM-DD — Title`), listed in reverse chronological order (newest first). Used for notes, features, todo, and log files.

---

## MARKDOWN FORMATTING

Standard formatting conventions used throughout all documents.

### Vertical Spacing
Headings should visually associate with the content that follows, not the content before:
- **H1, H2** — Three blank lines before, one blank line after
- **H3 AND BELOW** — No blank line after the heading
- **LISTS** — No blank line between a heading/text and the list that follows it

Example of correct spacing:
```markdown
Some preceding content here.



## Section Title

First paragraph of this section.

### Subsection
- List item 1
- List item 2

More text here.
```

### Named List
A bullet list where each item has a bold ALL CAPS name followed by a double dash and description:
- **NAME** — Description of what this item is or does

Used for: defining terms, listing standard entries, describing fields.

### Dated Sections Example
Example of a file with **dated sections**:
```
## 2026-01-12 — Topic or Feature Name

Content for this entry...

## 2026-01-10 — Earlier Topic

Earlier content...
```

### Date Format
Standard date format is `YYYY-MM-DD` (ISO 8601):
- **IN HEADINGS** — Use as prefix: `## 2026-01-12 — Topic Name`
- **IN FILENAMES** — Use as prefix for archived items: `2026-01-12 Old Project Name`
- **IN TEXT** — Use consistently for all dates

This format ensures:
- Chronological sorting when viewing alphabetically
- Unambiguous interpretation (no month/day confusion)
- Compatibility with international standards

---



# ANCHOR RULES

[[Anchor Template v1]]

The most common stream types are streams of anchors. Each anchor is usually associated with a folder. Here are the aspects that may be associated with an anchor folder.

---

## FILE STRUCTURE

Template variables:
- `{FULL_NAME}` = folder name (the full descriptive name)
- `{TLC}` = short code (if it exists, typically 2-5 uppercase letters)
- `{NAME}` = TLC if it exists, otherwise FULL_NAME
- `{repo}` = repository name

### Complete Folder Structure
```
{FULL_NAME}/                       # Anchor root folder (full descriptive name)
├── {FULL_NAME}.md                 # Redirect: contains only "See [[{TLC}]]" (if TLC exists)
│
├── {NAME} Docs/                   # Private docs (NOT published)
│   ├── {NAME}.md                  # PRIMARY ANCHOR PAGE (link table, overview)
│   ├── {NAME} PRD.md              # Product Requirements Document
│   ├── {NAME} Features.md         # Feature designs (dated sections)
│   ├── {NAME} Notes.md            # Discussion notes (dated sections)
│   ├── {NAME} Roadmap.md          # Milestone-based task tracking
│   └── {NAME} Todo.md             # Short-term tasks (dated sections)
│
├── {NAME} Research/               # Research materials (optional)
│   ├── {NAME} References.md       # Bibliography
│   └── {NAME} Related Work.md     # Analysis of related work
│
└── {repo}/                        # Repository clone (PUBLISHABLE)
    ├── .git/
    ├── README.md                  # Brief description, installation, quick start
    ├── CLAUDE.md                  # Claude Code project instructions
    ├── pyproject.toml             # Project metadata and dependencies
    ├── mkdocs.yml                 # MkDocs configuration
    ├── justfile                   # Task runner commands
    │
    ├── src/{package}/             # Source code
    ├── tests/                     # Test files
    │
    ├── docs/                      # Documentation SOURCE (hand-written + generated)
    │   ├── index.md               # Docs home page
    │   ├── user-guide/            # Task-oriented tutorials
    │   ├── architecture/          # System design docs
    │   └── api/                   # Generated API reference
    │
    └── site/                      # Generated docs site (gitignored)
```

### Concrete Example (with TLC)
Folder `Alien Biology/` contains:
- `Alien Biology.md` — says `See [[ABIO]]`
- `ABIO Docs/ABIO.md` — the main anchor page with link table
- `ABIO Docs/` — private planning/design docs
- `alienbio/` — the repository (publishable)

### Concrete Example (without TLC)
Folder `My Simple Project/` contains:
- `My Simple Project Docs/My Simple Project.md` — the main anchor page
- `My Simple Project Docs/` — private planning/design docs
- `my-simple-project/` — the repository (publishable)

### Anchor Folder Definition
- An **anchor** is a folder that contains a `{NAME} Docs/` subfolder with the primary anchor markdown
- The primary anchor markdown (`{NAME}.md`) lives inside the Docs folder
- If the anchor has a TLC, the root folder also has `{FULL_NAME}.md` containing only `See [[TLC]]`
- Example with TLC: `.../Alien Biology/ABIO Docs/ABIO.md` — primary anchor page
- Example without TLC: `.../My Project/My Project Docs/My Project.md` — primary anchor page

### Root Folder vs Repository
- The anchor root folder has a `{NAME} Docs/` subfolder for private documentation (NOT published)
- If the project has a code repository, it is a **subdirectory** of the anchor folder
- The subdirectory name matches the GitHub repository name (since it's a clone)
- This separation ensures planning docs don't accidentally get committed to the repo

---

## NAMING

### TLC (Three Letter Codes)
- Commonly accessed anchors have a short acronym for quick access
- Ideally three letters, hence "TLC" — but can be 2-5 letters if needed
- **TLCs should always be ALL CAPS** (e.g., SYS, ABIO, PCS)
- If an anchor has a TLC, create `{TLC}.md` in the `{TLC} Docs/` folder
- The root folder has `{FULL_NAME}.md` containing only: `See [[TLC]]`
- The TLC file in Docs becomes the primary anchor markdown with all the content

### TLC Index
- The markdown [[SYS/Closet/Three Letter Codes/TLC]] contains a table of all TLCs
- Periodically scan `~/ob/kmr` and `~/ob/proj` to find anchor folders with TLCs
- Table fields:
  - **DATE** — Creation date of TLC.md file (table sorted reverse chronologically)
  - **TLC** — Wiki link to the acronym file
  - **FULL ANCHOR NAME** — Folder name containing the TLC
  - **DESC** — Description (stored in anchor markdown with prefix `desc::`)

### Finding Anchors
Use the `ha` (HookAnchor) command to find anchor paths by TLC or name:
```bash
ha -p ASP              # Returns path to the ASP anchor folder
ha -p "Alien Biology"  # Find by full name
```
This is useful for quickly navigating to any anchor from the command line.

### File Naming with NAME Prefix
- Within an anchor folder, prefix related files with {NAME} (TLC if exists, else FULL_NAME)
- Example: `ASP Notes.md`, `ASP Roadmap.md`, `ASP PRD.md`
- This keeps files organized when viewing alphabetically
- Exception: Generic subfolder names like `Docs/`, `Research/`, `docs/`

---

## ANCHOR PAGE

The anchor page (`{TLC}.md`) is the primary entry point for the project.

### Description Field
- Include a `desc::` field near the top for the TLC index
- Format: `desc:: Brief description of the project`
- This is extracted when building the TLC table

### Anchor Link Table
The anchor page begins with a link table. Example:

| [[TLC Planning]]           | -------------------------------- [[Related Project]]      |
| -------------------------- | --------------------------------------------------------- |
| External                   | [Repo](url), [Docs](url)                                  |
| [[TLC Research]]           | [[TLC References]]                                        |
| [[TLC Planning\|Planning]] | [[TLC PRD]], [[TLC Features]], [[TLC Notes]]              |
| - Execution                | [[TLC Todo]], [[TLC Roadmap]]                             |
| [[TLC Docs\|User Docs]]    | [[User Guide]], [[Architecture Docs]], [[api/index\|API]] |

#### External Row
Links to resources outside the Obsidian vault:
- **REPO** — GitHub repository URL
- **DOCS** — Published documentation site (GitHub Pages)
- **LEGACY** — Link to legacy/archived version if this replaces an older project

#### Research Row
Links to research materials (optional, for research-oriented projects):
- **TLC RESEARCH** — Anchor for the research subfolder
- **TLC REFERENCES** — Bibliography and reference links
- **TLC RELATED WORK** — Analysis of related work

#### Planning Row
Links to planning documents (private, not published):
- **TLC PLANNING** — Anchor for the planning subfolder
- **TLC PRD** — Product Requirements Document; feature-by-feature design specs
- **TLC FEATURES** — Feature design log; **dated sections**
- **TLC NOTES** — High-level discussion notes; **dated sections**

#### Execution Row
Links to task tracking documents (sub-row of Planning):
- **TLC TODO** — Short-term tasks and open questions; **dated sections**
- **TLC ROADMAP** — Milestone-based task organization

#### User Docs Row
Links to published end-user documentation:
- **USER GUIDE** — Task-oriented tutorials and how-tos
- **ARCHITECTURE DOCS** — System design, class documentation
- **API** — Generated API reference (from docstrings)

### Body Content
- Below the link table, add project-specific content as needed
- Overview, key concepts, quick reference, etc.
- This is project-specific and varies by anchor type

---

## DOCS FOLDER

The `{NAME} Docs/` folder contains private planning and design documentation (NOT published).

### Standard Docs Documents
- **{NAME}.MD** — Primary anchor page (link table, overview)
- **{NAME} PRD.MD** — Product Requirements Document; feature-by-feature design specs
- **{NAME} FEATURES.MD** — Feature design log; **dated sections**
- **{NAME} NOTES.MD** — High-level discussion notes; **dated sections**
- **{NAME} ROADMAP.MD** — Milestone-based task organization
- **{NAME} TODO.MD** — Short-term tasks and open questions; **dated sections**
- **{NAME} OPEN QUESTIONS.MD** — Design questions awaiting resolution

### Roadmap Format
Roadmaps use checkboxes in headings to track milestone completion:

```markdown
## Phase 1: Foundation

### [ ] M1.1 - Repository Setup

Create the repository with initial structure.

**Deliverables**:
- [ ] Git repository initialized
- [ ] Directory structure created
- [ ] pyproject.toml configured

### [x] M1.2 - Basic Configuration

Completed milestone example.
```

Key conventions:
- **PHASES** — H2 headings group related milestones
- **MILESTONES** — H3 headings with `[ ]` or `[x]` checkbox, numbered (M1.1, M1.2, etc.)
- **DELIVERABLES** — Bullet lists with checkboxes under each milestone
- Checkboxes in headings allow tracking at both milestone and task level

Placement:
- Roadmap lives in `{NAME} Docs/` folder as `{NAME} Roadmap.md`
- Linked from anchor page under "Execution Docs" row (sub-row of Planning)
- Grouped with Todo (both are execution/tracking docs, vs PRD/Features/Notes which are design docs)

---

## DOCUMENTATION

Documentation is split between private docs (anchor level) and published user docs (repository level).

### Private Docs vs User Docs

- **PRIVATE DOCS** — Located in `{NAME} Docs/` folder at anchor level. Contains planning, design decisions, internal discussions, rough ideas. NOT published.
- **USER DOCS** — Published, located in repository under `docs/`. Contains polished documentation for end users and developers.

### docs/ Folder Structure

All repositories with documentation should organize `docs/` as follows:

```
docs/
├── index.md              # Documentation home page (entry point)
├── user-guide/           # Task-oriented tutorials and how-tos
│   ├── getting-started.md
│   ├── installation.md
│   └── ...
├── architecture/         # System design and technical reference
│   ├── overview.md
│   ├── config-reference.md
│   └── ...
└── api/                  # Generated API reference (auto-generated)
    └── ...
```

### Documentation Types

- **INDEX.MD** — Entry point linking to all documentation sections
- **USER-GUIDE/** — Task-oriented tutorials, getting started guides, how-tos. Written for end users.
- **ARCHITECTURE/** — System design docs, configuration reference, technical specifications. Written for developers.
- **API/** — Auto-generated from source code. Do not edit manually.

### Documentation Generators

Choose the appropriate generator for your project type:

- **PYTHON** — MkDocs with mkdocstrings for API docs
- **SWIFT** — swift-docc or Jazzy for API docs
- **TYPESCRIPT/JS** — TypeDoc for API docs
- **GENERAL** — MkDocs, Docusaurus, or similar static site generator

### MkDocs Setup (Python Projects)

```
repo/
├── mkdocs.yml            # MkDocs configuration
├── docs/                 # Documentation source
└── site/                 # Generated site (gitignored)
```

Key files:
- **MKDOCS.YML** — Configuration file in repo root
- **SITE/** — Generated documentation folder (gitignored, deployed to GitHub Pages)

### Documentation Workflow

1. Write user guides and architecture docs by hand in `docs/`
2. API docs are auto-generated from source code docstrings
3. Build docs locally: `mkdocs build` (or equivalent)
4. Preview locally: `mkdocs serve` (or equivalent)
5. Deploy to GitHub Pages: `mkdocs gh-deploy` (or equivalent)

---

## CODE REPOSITORY STRUCTURE

See the Complete Folder Structure above for the full layout.

### Key Repository Files
- **README.MD** — Brief project description, installation, quick start
- **CLAUDE.MD** — Claude Code project instructions; coding conventions, key patterns
- **PYPROJECT.TOML** — Dependencies, build config, project metadata (Python)
- **JUSTFILE** — Task runner; common tasks like `just build`, `just test`, `just docs`
- **MKDOCS.YML** — MkDocs documentation site configuration (if using MkDocs)

### site/ Folder
Generated by documentation build. Should be gitignored. Deployed to GitHub Pages.

---

## OPTIONAL INTEGRATIONS

### Git
- Anchor may have its own git repository at the repo subdirectory
- Default remote: private repo on GitHub
- Repository name matches subdirectory name

### GitHub Pages
- Published documentation hosted via GitHub Pages
- URL pattern: `username.github.io/repo-name/`
- Built from `docs/` folder or `gh-pages` branch

### Claude (CLAUDE.md)
- `CLAUDE.md` at repo root provides project-specific instructions
- Include: coding conventions, key patterns, things to avoid

### tmux
- Project may have associated tmux session for development
- Session name typically matches TLC

---

## RESEARCH PROJECTS

### Research Folder
- `TLC Research/` — Contains research materials
- `TLC References.md` — Bibliography and reference links
- `TLC Related Work.md` — Analysis of related work

### Paper Structure (if writing papers)
- Section files: `TLC 1 Introduction.md`, `TLC 2 Methods.md`, etc.
- Log files for each section: `TLC 1 Introduction Log.md`
- Appendix files: `TLC A1 - Topic.md`
- Experiment series: `TLC B1 - Experiment Name.md`

---

## CHECKLIST: Creating a New Anchor

1. [ ] Create folder with descriptive name
2. [ ] Create anchor markdown (`Folder Name.md`)
3. [ ] Decide on TLC if commonly accessed
4. [ ] Create TLC.md with link table, update folder markdown to `See [[TLC]]`
5. [ ] Create Planning subfolder with: PRD, Notes, Roadmap
6. [ ] If code project: create repository subdirectory
7. [ ] If code project: set up docs/, mkdocs.yml, pyproject.toml
8. [ ] Add to TLC index if applicable
9. [ ] Create CLAUDE.md if using Claude for development

---

# ACTIONS

## Organizational Actions

### Move Anchor Folder
Moving an anchor folder requires updating several systems that index by path.

#### 1. Move the Folder
```bash
mv "/old/path/{NAME}" "/new/path/{NAME}"
```

#### 2. Migrate Claude Code Sessions
Claude Code stores sessions in `~/.claude/projects/` with paths encoded (slashes become dashes). When you move a folder, sessions become orphaned.

**Manual migration:**
```bash
cd ~/.claude/projects/
# Rename the session directory (use -- to handle leading dash)
mv -- -old-path-encoded -new-path-encoded
```

Example: Moving `~/ob/proj/My Project` to `~/ob/kmr/prj/My Project`:
```bash
mv -- -Users-oblinger-ob-proj-My-Project -Users-oblinger-ob-kmr-prj-My-Project
```

**Automated migration:** Use the `claude-mv` script from Chase Adams:
```bash
claude-mv ~/old/path ~/new/path
```
This handles renaming directories and updating path references in `.jsonl` files.

**Verify:** After migration, run `claude --continue` in the new location.

#### 3. Reindex HookAnchor
HookAnchor maintains an index of anchor folders. Trigger a rescan:
```bash
ha --rescan     # Rescan filesystem
```

#### 4. Rebuild Documentation Index
If the anchor has published docs, rebuild the MkDocs site:
```bash
cd /new/path/{repo}
mkdocs build    # Rebuild site/ folder
```

If using GitHub Pages, push changes to trigger rebuild.

#### 5. Update TLC Index
If the anchor has a TLC, update [[SYS/Closet/Three Letter Codes/TLC]] with the new location.

#### 6. Update Git Remotes (if applicable)
If the repository was moved, remote URLs should still work. But verify:
```bash
cd /new/path/{repo}
git remote -v
```

#### 7. Scan for Hardcoded Paths
Search the codebase for hardcoded references to the old path:
```bash
cd /new/path/{repo}
grep -r "/old/path" .
grep -r "old-path-segment" .
```

Common places where paths may be hardcoded:
- **CLAUDE.MD** — Project instructions may reference absolute paths
- **CONFIG FILES** — pyproject.toml, mkdocs.yml, justfile
- **SCRIPTS** — Shell scripts, Python scripts with path constants
- **DOCUMENTATION** — Examples or tutorials with absolute paths
- **TESTS** — Test fixtures or test data paths

#### Checklist: Move Anchor Folder
1. [ ] Move the folder itself
2. [ ] Migrate Claude Code sessions (`~/.claude/projects/`)
3. [ ] Reindex HookAnchor (`ha --rescan`)
4. [ ] Rebuild docs if applicable (`mkdocs build`)
5. [ ] Update TLC index
6. [ ] Verify git remotes
7. [ ] Scan for and update hardcoded paths
8. [ ] Test: `claude --continue` works in new location

---

## Coding Actions

### Justfile Standards
The `justfile` is the standard way to manage project tasks. Use [just](https://github.com/casey/just) as the task runner.

#### Standard Recipes
- **BUILD** — Build the project (compile, package)
- **TEST** — Run the test suite
- **LINT** — Run linters (ruff, mypy, etc.)
- **CHECK** — Run all checks (lint + test)
- **DOCS** — Build documentation (`mkdocs build`)
- **DOCS-SERVE** — Serve docs locally (`mkdocs serve`)
- **DOCS-DEPLOY** — Deploy docs to GitHub Pages (`mkdocs gh-deploy`)
- **INSTALL** — Install dependencies
- **DEV** — Install in development mode with dev dependencies
- **CLEAN** — Remove build artifacts, cache files
- **RELEASE** — Build and publish to PyPI

#### Example Justfile
```just
# Default recipe - show available commands
default:
    @just --list

# Build the project
build:
    python -m build

# Run tests
test:
    pytest tests/

# Run linter
lint:
    ruff check src/

# Format code
format:
    ruff format src/ tests/

# Run all checks
check: lint test

# Build documentation
docs:
    mkdocs build

# Serve docs locally
docs-serve:
    mkdocs serve

# Deploy docs to GitHub Pages
docs-deploy:
    mkdocs gh-deploy

# Install in development mode
dev:
    pip install -e ".[dev]"

# Clean build artifacts
clean:
    rm -rf dist/ build/ *.egg-info site/ .pytest_cache/ .ruff_cache/
```

#### Project-Specific Recipes
Add project-specific recipes as needed:
- **RUN** — Run the application
- **DB-MIGRATE** — Run database migrations
- **DOCKER-BUILD** — Build Docker image
- **DEPLOY** — Deploy to production

---

### Archive to Yore/
The `Yore/` subfolder stores archived code, documentation, or other materials that are no longer active but worth preserving. The name "Yore" sorts late alphabetically, keeping archives at the bottom of directory listings.

#### When to Archive
- Old versions of code being replaced
- Deprecated documentation
- Legacy implementations kept for reference
- Backup copies before major refactoring

#### Archive Naming
Prefix archived items with the date in `YYYY-MM-DD` format:
```
Yore/
├── 2026-01-12 OldProjectName/
├── 2025-11-15 DeprecatedModule/
└── 2025-08-20 LegacyDocs/
```

#### Archive Procedure
1. Create `Yore/` folder if it doesn't exist
2. Move the item with date prefix:
   ```bash
   mkdir -p Yore
   mv "OldProject" "Yore/$(date +%Y-%m-%d) OldProject"
   ```
3. Update any references or symlinks that pointed to the archived item
4. Add a note in the anchor page or README about what was archived and why

---

### Update Double Click Symlinks
*(To be documented)*

When moving anchor folders, Double Click symlinks under the vault's Users tree may need updating to point to new locations.
