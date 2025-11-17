# Test Coverage Analysis: Prefix Menu vs Global Matching

## Current Test Coverage by Context

### Part 2: COMMAND MATCHING (7 tests)

All Part 2 tests currently test **INSIDE PREFIX MENUS**:

| Test | Context | Query | Expected Match |
|------|---------|-------|----------------|
| `part2_match_word_boundaries` | PREFIX (PJ) | "PJT" | "PJ Tasks" |
| `part2_multi_char_from_same_word` | GLOBAL | "ProDir" | "Project Directories" |
| `part2_can_skip_words` | PREFIX (PJ) | "PT" | "PJ Tasks" |
| `part2_skip_entire_first_word` | PREFIX (PJ) | "Tasks" | "PJ Tasks" |
| `part2_case_insensitive` | PREFIX (PJ) | "pjt" | "PJ Tasks" |
| `part2_no_mid_word_matching` | GLOBAL | "ject" | Should NOT match "Project" |
| `part2_multi_word_matching` | ? | ? | ? |

**Analysis:**
- 4 tests use PJ anchor (prefix menu context)
- 2 tests have no anchor (global context)
- Mixed coverage - need systematic pairs

### Part 4: SORTING & ORDERING (5 tests)

All Part 4 tests currently test **INSIDE PREFIX MENUS**:

| Test | Context | Query | What it tests |
|------|---------|-------|---------------|
| `part4_exact_matches_first` | PREFIX (Main) | "MainCode" | Exact before partial |
| `part4_no_words_skipped_before_words_skipped` | PREFIX (Main) | "MainCo" | No-skip before skip |
| `part4_alphabetical_within_tier` | PREFIX (Main) | "Main" | Alpha within tier |
| `part4_sort_by_filter_in_prefix_menu` | PREFIX (PJ) | "PJPP" | Sort by filter |
| `part4_multi_char_matching_no_skip_priority` | PREFIX (T) | "TWOT" | Multi-char no-skip priority |

**Analysis:**
- ALL tests are in prefix menu context
- NO global matching tests for sorting rules
- **MISSING:** Global equivalents for all 5 sorting tests

## Required Test Pairs

### Matching Rules (Part 2) - Need Global Equivalents

1. **Word Boundary Matching**
   - ✅ HAVE: `part2_match_word_boundaries` (PREFIX: "PJT" → "PJ Tasks")
   - ❌ NEED: `part2_match_word_boundaries_global` (GLOBAL: "PJT" → "PJ Tasks")

2. **Word Skipping**
   - ✅ HAVE: `part2_can_skip_words` (PREFIX: "PT" → "PJ Tasks")
   - ❌ NEED: `part2_can_skip_words_global` (GLOBAL: "PT" → "PJ Tasks")

3. **Skip Entire Word**
   - ✅ HAVE: `part2_skip_entire_first_word` (PREFIX: "Tasks" → "PJ Tasks")
   - ❌ NEED: `part2_skip_entire_first_word_global` (GLOBAL: "Tasks" → "PJ Tasks")

4. **Case Insensitive**
   - ✅ HAVE: `part2_case_insensitive` (PREFIX: "pjt" → "PJ Tasks")
   - ❌ NEED: `part2_case_insensitive_global` (GLOBAL: "pjt" → "PJ Tasks")

### Sorting Rules (Part 4) - Need Global Equivalents

1. **Exact Matches First**
   - ✅ HAVE: `part4_exact_matches_first` (PREFIX)
   - ❌ NEED: `part4_exact_matches_first_global` (GLOBAL)

2. **No Words Skipped Before Words Skipped**
   - ✅ HAVE: `part4_no_words_skipped_before_words_skipped` (PREFIX)
   - ❌ NEED: `part4_no_words_skipped_before_words_skipped_global` (GLOBAL)

3. **Alphabetical Within Tier**
   - ✅ HAVE: `part4_alphabetical_within_tier` (PREFIX)
   - ❌ NEED: `part4_alphabetical_within_tier_global` (GLOBAL)

4. **Multi-Char No-Skip Priority**
   - ✅ HAVE: `part4_multi_char_matching_no_skip_priority` (PREFIX)
   - ❌ NEED: `part4_multi_char_matching_no_skip_priority_global` (GLOBAL)

## Summary

**Current:** 35 tests total
- Part 2: Mixed context (4 prefix, 2 global, 1 unknown)
- Part 4: All prefix menu only (5 tests)

**Needed:** 8 additional tests minimum
- 4 global matching tests for Part 2 rules
- 4 global sorting tests for Part 4 rules

**Target:** 43 tests total (35 current + 8 new)

## Test Naming Convention

Proposed naming:
- Prefix menu tests: `part{N}_{description}` (current)
- Global tests: `part{N}_{description}_global` (new)

Example:
- `part2_match_word_boundaries` - tests in prefix menu
- `part2_match_word_boundaries_global` - tests in global menu
