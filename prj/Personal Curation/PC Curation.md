# GENERAL CURATION RULES

## FILE STRUCTURE

Template variables:
- `{FULL_NAME}` = full descriptive name (Title Case with spaces)
- `{TLC}` = short code (if it exists, typically 2-5 uppercase letters)
- `{NAME}` = TLC if it exists, otherwise FULL_NAME
- `{kebab-name}` = FULL_NAME in kebab-case (lowercase, hyphens)
- `{snake_name}` = FULL_NAME in snake_case (lowercase, underscores)

### Naming Convention
The same name appears in different formats to indicate context:
- **kebab-case** = Git repository / anchor folder
- **Title Case with spaces** = Wrapper folder (public repos only)
- **snake_case** = Python package

Example: `system-setup` (repo) → `System Setup` (wrapper) → `system_setup` (package)

---

### Private Repo Anchor (Primary Case)

For private repos, the repository IS the anchor. No wrapper folder needed.

```
{kebab-name}/                      # Repository = Anchor (kebab-case)
├── .git/
├── README.md
├── CLAUDE.md
│
├── {NAME} Docs/                   # Private docs (version controlled)
│   ├── {NAME}.md                  # PRIMARY ANCHOR PAGE
│   ├── {NAME} PRD.md
│   ├── {NAME} Roadmap.md
│   └── ...
│
│   ─── If Python project ───
│
├── pyproject.toml
├── justfile
├── src/{snake_name}/
├── tests/
├── docs/                          # Published user docs
└── site/                          # Generated (gitignored)
```

**Example**: `system-setup/` is the anchor
- `system-setup/SYS Docs/SYS.md` — primary anchor page
- `system-setup/SYS Docs/` — all private planning docs
- Everything is version controlled in one repo

---

### Public Repo Anchor

For public repos, private docs can't be in the repo. Add a wrapper folder.

```
{FULL_NAME}/                       # Wrapper folder (Title Case)
├── {FULL_NAME}.md                 # Pointer: "See [[{NAME}]]"
│
├── {NAME} Docs/                   # Private docs (OUTSIDE repo)
│   ├── {NAME}.md                  # PRIMARY ANCHOR PAGE
│   └── ...
│
└── {kebab-name}/                  # Public repository
    ├── .git/
    ├── README.md
    ├── src/
    └── ...
```

**Version control for private docs**: Create a parent `proj/` repo with gitignore whitelist:

```
proj/                              # Parent repo (private)
├── .gitignore                     # Ignore *, whitelist doc folders
├── Alien Biology/                 # Wrapper folder
│   ├── Alien Biology.md
│   ├── ABIO Docs/                 # Tracked by parent repo
│   └── alien-biology/             # Public repo (ignored)
```

Gitignore whitelist pattern:
```gitignore
*
!{FULL_NAME}/
!{FULL_NAME}/{NAME} Docs/
!{FULL_NAME}/{NAME} Docs/**
```

---

### Anchor Definition
- An **anchor** is a folder containing `{NAME} Docs/` with the primary anchor markdown
- For private repos: the repo root IS the anchor
- For public repos: the wrapper folder is the anchor

---

## DEFINED TERMS

- **DATED FOLDER** — A folder where all files and subfolders within it are prefixed with the date format (`YYYY-MM-DD`). Items sort in chronological order when viewed alphabetically.

- **DATED SECTIONS** — A file composed of H2 sections whose titles begin with a date prefix (`## YYYY-MM-DD — Title`), listed in reverse chronological order (newest first). Used for notes, features, todo, and log files.

---

## CURATION ACTIONS

Each action has a standalone runbook document optimized for execution.

| Action | Trigger | Description |
| ------ | ------- | ----------- |
| [[PC PR Flow]] | "PR flow" | Iterative development using pull requests |
| [[PC Tidy]] | "tidy" | Validate and correct anchor folder structure |
| [[PC Roadmap Ready]] | "roadmap ready" | Validate roadmap structure and surface open questions |
| [[PC Move Anchor]] | Move anchor | Update all systems when moving an anchor folder |
| [[PC Archive]] | Archive | Move old content to Yore/ with date prefix |

### Finding Anchors

Use the `ha` (HookAnchor) command to find anchor paths:
```bash
ha -p ASP              # Returns path to the ASP anchor folder
ha -p "Alien Biology"  # Find by full name
```

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

## NAMING

### TLC (Three Letter Codes)
- Commonly accessed anchors have a short acronym for quick access
- Ideally three letters, hence "TLC" — but can be 2-5 letters if needed
- **TLCs should always be ALL CAPS** (e.g., SYS, ABIO, PC)
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

### Anchor Page Rules
- All documentation must be linked from the anchor page
- If content would generate a lot of text directly in the anchor page, move it to a sub-document and link to it
- Goal is a dense, scannable page with links to details

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
Roadmaps contain only high-level descriptions of what needs to be done. Detailed content belongs elsewhere:
- **Detailed discussion** — Put in Notes document, reference from roadmap via H2 section
- **Detailed task lists** — Put in Todo document, reference from roadmap

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

### Handling Milestone Deferrals
When a milestone needs to be deferred to a later phase:

1. **Mark the deferred item** with `[~]` and add "(Deferred - see Mx.y)" to the title:
   ```markdown
   ### [~] M1.11 - Documentation Sync (Deferred - see M3.14)
   ```

2. **Add a revisit milestone** at the end of the target milestone/phase:
   ```markdown
   ### [ ] M3.14 - Revisit: M1.11 Documentation Sync
   ```

3. **Cross-reference both directions**:
   - The deferred item points to where it will be revisited
   - The revisit item links back to the original deferred milestone

This ensures:
- Deferred work is not forgotten
- There's a defined point where it will be addressed
- Traceability in both directions

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

### Documenting Code Interfaces

When documenting APIs or code interfaces, follow these conventions:

**Show return types** — Always annotate the return type so readers know what they're getting:
```python
run: dict = bio.fetch("data/experiments/run_001")
```

**Inline comments on same line** — Put explanatory comments on the same line as the code:
```python
run: dict = bio.fetch("data/experiments/run_001")              # data directory — run results
scenario: Scenario = bio.fetch("data/experiments/run_001.scenario")  # dig into structure
scenario: Scenario = bio.fetch("catalog.scenarios.mutualism")  # source tree — template
```

**Align comment markers** — Line up the `#` symbols for readability when showing multiple related calls.

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

## MAINTENANCE RULES

Checklist for validating an anchor is properly set up:

- **ANCHOR PAGE** — Ensure all documentation is correctly linked from the anchor page
- **ROADMAP** — Contains only high-level descriptions, not detailed content
- **LINK TABLE** — All rows populated with valid links
- **TLC INDEX** — If anchor has TLC, verify it's in the TLC index table

---

# STANDARDS

## Justfile Standards

The `justfile` is the standard way to manage project tasks. Use [just](https://github.com/casey/just) as the task runner.

### Standard Recipes
- **BUILD** — Build the project (compile, package)
- **TEST** — Run the test suite
- **LINT** — Run linters (ruff, mypy, etc.)
- **CHECK** — Run all checks (lint + test)
- **DOCS** — Build documentation
- **INSTALL** — Install dependencies
- **DEV** — Install in development mode
- **CLEAN** — Remove build artifacts

### Project-Specific Recipes
Add project-specific recipes as needed:
- **RUN** — Run the application
- **DB-MIGRATE** — Run database migrations
- **DOCKER-BUILD** — Build Docker image
- **DEPLOY** — Deploy to production

---

## Update Double Click Symlinks
*(To be documented)*

When moving anchor folders, Double Click symlinks under the vault's Users tree may need updating to point to new locations.
