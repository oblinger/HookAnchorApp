# ANCHOR RULES

[[Anchor Template v1]]

The most common stream types are streams of anchors. Each anchor is usually associated with a folder. Here are the aspects that may be associated with an anchor folder.

---

## FILE STRUCTURE

### Anchor Folder Definition
- An **anchor** is a folder that contains a Markdown file with the same name as the folder
- Example: `.../My Project/My Project.md` — the `.md` file is the "anchor markdown"
- The anchor markdown provides links to key parts of the anchor folder
- An anchor folder without the same-named markdown is just a regular folder

### Root Folder vs Repository
- The anchor root folder contains planning/private documentation that should NOT be published
- If the project has a code repository, it is a **subdirectory** of the anchor folder
- The subdirectory name matches the GitHub repository name (since it's a clone)
- Template variables: `{NAME}` = full name, `{TLC}` = short code, `{repo}` = repository name

### Complete Folder Structure
```
{NAME}/                            # Anchor root folder (full descriptive name)
├── {NAME}.md                      # Redirect: contains only "See [[{TLC}]]"
├── {TLC}.md                       # PRIMARY ANCHOR PAGE (link table, overview)
│
├── {TLC} Planning/                # Planning docs (PRIVATE - not published)
│   ├── {TLC} Planning.md          # Anchor for planning subfolder
│   ├── {TLC} PRD.md               # Product Requirements Document
│   ├── {TLC} Features.md          # Feature designs (streaming)
│   ├── {TLC} Notes.md             # Discussion notes (streaming)
│   ├── {TLC} Roadmap.md           # Milestone-based task tracking
│   └── {TLC} Todo.md              # Short-term tasks (streaming)
│
├── {TLC} Research/                # Research materials (optional)
│   ├── {TLC} References.md        # Bibliography
│   └── {TLC} Related Work.md      # Analysis of related work
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

### Concrete Example
Folder `Alien Biology/` contains:
- `Alien Biology.md` — says `See [[ABIO]]`
- `ABIO.md` — the main anchor page with link table
- `ABIO Planning/` — private planning docs
- `alienbio/` — the repository (publishable)

This separation ensures planning docs don't accidentally get committed to the repo.

---

## NAMING

### TLC (Three Letter Codes)
- Commonly accessed anchors have a short acronym for quick access
- Ideally three letters, hence "TLC" — but can be 2-4 letters if needed
- If an anchor has a TLC, create a `TLC.md` file (e.g., `ASP.md`)
- The full-named anchor markdown should contain only: `See [[TLC]]`
- The TLC file becomes the primary anchor markdown with all the content

### TLC Index
- The markdown [[SYS/Closet/Three Letter Codes/TLC]] contains a table of all TLCs
- Periodically scan `~/ob/kmr` and `~/ob/proj` to find anchor folders with TLCs
- Table fields:
  - **DATE** — Creation date of TLC.md file (table sorted reverse chronologically)
  - **TLC** — Wiki link to the acronym file
  - **FULL ANCHOR NAME** — Folder name containing the TLC
  - **DESC** — Description (stored in anchor markdown with prefix `desc::`)

### File Naming with TLC Prefix
- Within an anchor folder, prefix related files with the TLC
- Example: `ASP Notes.md`, `ASP Roadmap.md`, `ASP PRD.md`
- This keeps files organized when viewing alphabetically
- Exception: Generic subfolder names like `Planning/`, `Research/`, `docs/`

---

## ANCHOR PAGE

The anchor page (`{TLC}.md`) is the primary entry point for the project. It has specific structural rules.

### Link Table at Top
- The anchor page begins with a table linking to key resources
- Table format with columns for category, links, and optional notes
- Standard rows (include only those that apply):

| Row | Purpose | Example Links |
|-----|---------|---------------|
| **External** | Links outside the vault | GitHub repo, published docs, legacy |
| **Research** | Research materials | References, related work |
| **Planning** | Planning documents | PRD, Features, Notes, Roadmap, Todo |
| **User Docs** | End-user documentation | User Guide, Architecture, API Reference |

### Example Link Table
```markdown
| [[TLC Planning]]           | -------------------------------- [[Related Project]]        |
| -------------------------- | ------------------------------------------------------------ |
| External                   | [Repo](url), [Docs](url), [[legacy/TLC Legacy\|Legacy]]      |
| [[TLC Research]]           | [[TLC References]]                                           |
| [[TLC Planning\|Planning]] | [[TLC PRD]], [[TLC Features]], [[TLC Notes]]                 |
| - Execution                | [[TLC Todo]], [[TLC Roadmap]]                                |
| [[TLC Docs\|User Docs]]    | [[User Guide]], [[Architecture Docs]], [[api/index\|API]]    |
```

### Description Field
- Include a `desc::` field near the top for the TLC index
- Format: `desc:: Brief description of the project`
- This is extracted when building the TLC table

### Body Content
- Below the link table, add project-specific content as needed
- Overview, key concepts, quick reference, etc.
- This is project-specific and varies by anchor type

---

## PLANNING FOLDER

### Standard Planning Documents
- `TLC Planning.md` — Anchor for the planning subfolder
- `TLC PRD.md` — Product Requirements Document (feature-by-feature design)
- `TLC Features.md` — Streaming file of feature designs (dated H2 entries)
- `TLC Notes.md` — Streaming file of high-level discussion notes (dated H2 entries)
- `TLC Roadmap.md` — Milestone-based task organization
- `TLC Todo.md` — Short-term task tracking, open questions (dated H2 entries)
- `TLC Open Questions.md` — Design questions awaiting resolution

### Streaming Files
- Some files are "streaming" — they accumulate entries over time
- Format: H2 headers with dated prefixes, newest first (reverse chronological)
- Example:
  ```markdown
  ## 2026-01-12 — Feature: Export to CSV

  Design notes for CSV export...

  ## 2026-01-10 — Feature: Dark Mode

  Design notes for dark mode...
  ```
- Streaming files: Features, Notes, Todo, Log files

---

## DOCUMENTATION STRUCTURE

### Planning Docs (Private)
- Located in anchor root, NOT in repository
- Include: PRD, Features, Notes, Roadmap, Todo
- These contain work-in-progress, internal discussions, rough ideas

### User Docs (Published)
- Located in repository under `docs/`
- Types:
  - **User Guide** — Task-oriented tutorials and how-tos
  - **Architecture Docs** — System design, class documentation
  - **API Reference** — Generated from source code (MkDocs, Sphinx, etc.)

### Documentation Generation
- Use MkDocs (Python) or equivalent for API reference generation
- `mkdocs.yml` configures documentation build
- Generated site goes to `site/` folder or GitHub Pages
- API docs are auto-generated from docstrings; other docs are hand-written

---

## CODE REPOSITORY STRUCTURE

See the Complete Folder Structure above for the full layout. Key points:

### Key Repository Files
- `README.md` — Brief project description, installation, quick start
- `CLAUDE.md` — Claude Code project instructions (coding conventions, key patterns)
- `pyproject.toml` — Dependencies, build config, project metadata
- `justfile` — Common tasks: `just build`, `just test`, `just docs`
- `mkdocs.yml` — MkDocs documentation site configuration

### docs/ Folder Organization
- `docs/index.md` — Documentation home page
- `docs/user-guide/` — Task-oriented tutorials and how-tos
- `docs/architecture/` — System design docs, class documentation
- `docs/api/` — Generated API reference (auto-generated from docstrings)

### site/ Folder
- Generated by `mkdocs build`
- Should be gitignored
- Deployed to GitHub Pages

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
