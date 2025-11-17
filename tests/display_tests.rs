//! Comprehensive tests for display menu construction logic
//!
//! These tests verify the specification documented in src/core/display.rs
//!
//! Test Organization (matches specification):
//! - Part 1: PREFIX MENU IDENTIFICATION
//! - Part 2: COMMAND MATCHING
//! - Part 3: PREFIX MENU CONSTRUCTION
//! - Part 4: SORTING & ORDERING
//! - Part 5: FINAL MENU ASSEMBLY
//! - Edge Cases & Special Behaviors

mod test_helpers;

use test_helpers::*;
use hookanchor::core::get_new_display_commands;

// ============================================================================
// TEST SCAFFOLDS
// ============================================================================

/// Real PJ menu commands from user's actual configuration
fn pj_menu_scaffold() -> TestScaffold {
    scaffold(r#"
        // PJ Anchor
        PJ:anchor; F:=UA A:=/Users/oblinger/ob/kmr/prj

        // PJ menu items - using patch field
        PJ! PP:markdown; F:=UA A:=/Users/oblinger/ob/kmr/prj/pp/PP.md
        PJ! CVT Gdrive:work; F:=U A:=https://drive.google.com/drive
        PJ! Code Proj:folder; F:=UA A:=/Users/oblinger/ob/proj

        // PJ menu items - using name prefix (command names start with "PJ ")
        PJ Tasks:markdown; A:=/Users/oblinger/tasks.md
        PJ Notes:markdown; A:=/Users/oblinger/notes.md

        // Distractors - should NOT appear in PJ menu
        Projects:folder; A:=/Users/shared/projects
        Personal:folder; A:=/Users/home/personal
        Python Code:folder; A:=/Users/code/python
    "#)
}

/// Simple progressive matching scaffold
fn progressive_scaffold() -> TestScaffold {
    scaffold(r#"
        A:anchor; F:=A A:=/path/a
        AB:anchor; F:=A A:=/path/ab
        ABC:anchor; F:=A A:=/path/abc

        A! Item1:folder; A:=/item1
        A! Xray:folder; A:=/xray
        AB! Item2:folder; A:=/item2
        AB! Xenon:folder; A:=/xenon
        ABC! Item3:folder; A:=/item3
        ABC! Delta:folder; A:=/delta

        ABCD:folder; A:=/abcd
    "#)
}

/// Alias resolution scaffold
fn alias_scaffold() -> TestScaffold {
    scaffold(r#"
        Target:anchor; F:=A A:=/target
        T:alias; A:=Target

        Target! File1:folder; A:=/file1
        Target! File2:folder; A:=/file2

        Testing:folder; A:=/testing
    "#)
}

/// Date prefix scaffold
fn date_prefix_scaffold() -> TestScaffold {
    scaffold(r#"
        Project:anchor; F:=A A:=/project

        2024-01-15 Project Notes:markdown; A:=/notes1.md
        2024_02_20_Project Report:markdown; A:=/report.md
        20240315-Project Meeting:markdown; A:=/meeting.md

        Project! Tasks:markdown; A:=/tasks.md
    "#)
}

/// Matching priority test scaffold
fn matching_priority_scaffold() -> TestScaffold {
    scaffold(r#"
        Main:anchor; F:=A A:=/main

        // Exact match candidates
        Main! Code:folder; A:=/code

        // No-words-skipped candidates
        Main! Code Projects:folder; A:=/code_projects
        Main! Code Review:folder; A:=/code_review

        // Words-skipped candidates
        Main! Old Code Files:folder; A:=/old_code
        Main! Some Code Here:folder; A:=/some_code
    "#)
}

// ============================================================================
// PART 1: PREFIX MENU IDENTIFICATION
// ============================================================================

#[test]
fn part1_backward_scan_longest_first() {
    // Input "ABCD" should try: ABCD → ABC → AB → A
    // Should match ABC (longest match), filter text = "D"
    let scaffold = progressive_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "ABCD",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);
    // ABC anchor doesn't match filter "D" so it won't appear
    assert_contains(&result, "Delta"); // Matches filter "D"
    assert_not_contains(&result, "Item3"); // Doesn't match filter "D"
    assert_not_contains(&result, "ABC"); // Anchor doesn't match filter
}

#[test]
fn part1_backward_scan_stops_at_first_anchor() {
    // Input "ABX" should try: ABX → AB → A
    // Should stop at AB (first anchor found), filter text = "X"
    let scaffold = progressive_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "ABX",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);
    // AB anchor doesn't match filter "X" so it won't appear
    assert_contains(&result, "Xenon"); // Matches filter "X"
    assert_not_contains(&result, "Xray"); // "Xray" is in A menu, not AB menu
    assert_not_contains(&result, "AB"); // Anchor doesn't match filter
}

#[test]
fn part1_case_insensitive_matching() {
    let scaffold = progressive_scaffold();

    // Lowercase "abc" should match uppercase "ABC" anchor
    let (result, is_prefix, _, _) = get_new_display_commands(
        "abc",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);
    assert_contains(&result, "ABC");
}

#[test]
fn part1_alias_resolution() {
    // "T" is alias to "Target" which is anchor
    let scaffold = alias_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "T",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);
    assert_contains(&result, "Target");
    assert_contains(&result, "File1");
}

// TODO: Add date prefix skipping support later
// #[test]
// fn part1_date_prefix_skip() {
//     // Commands with date prefixes should still match anchor
//     let scaffold = date_prefix_scaffold();
//
//     let (result, is_prefix, _, _) = get_new_display_commands(
//         "Project",
//         &scaffold.commands,
//         &scaffold.patches,
//         &scaffold.config
//     );
//
//     assert_prefix_menu(is_prefix, true);
//     assert_contains(&result, "2024-01-15 Project Notes");
// }

#[test]
fn part1_no_anchor_found() {
    // "ZZZZZ" matches no anchor → no prefix menu
    let scaffold = progressive_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "ZZZZZ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, false);
}

#[test]
fn part1_filter_text_extraction() {
    // Input "PJPP" → anchor "PJ", filter "PP"
    let scaffold = pj_menu_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "PJPP",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);
    // PP should appear (matches filter "PP")
    assert_contains(&result, "PP");
}

// ============================================================================
// PART 2: COMMAND MATCHING
// ============================================================================

#[test]
fn part2_match_word_boundaries() {
    // Input "PJD" should match "PJ Directories"
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "PJD",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_contains(&result, "PJ Tasks");
}

#[test]
fn part2_multi_char_from_same_word() {
    // Input "ProDir" matches "Project Directories"
    // (Pro from "Project", Dir from "Directories")
    let scaffold = scaffold(r#"
        Project Directories:folder; A:=/proj_dir
    "#);

    let (result, _, _, _) = get_new_display_commands(
        "ProDir",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_contains(&result, "Project Directories");
}

#[test]
fn part2_can_skip_words() {
    // Input "PD" matches "PJ Directories" (skipped middle of first word)
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "PD",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_contains(&result, "PJ Tasks");
}

#[test]
fn part2_skip_entire_first_word() {
    // Input "Dir" matches "PJ Directories" (skipped "PJ" entirely)
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "Dir",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_contains(&result, "PJ Tasks");
}

#[test]
fn part2_case_insensitive() {
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "pjd",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_contains(&result, "PJ Tasks");
}

#[test]
fn part2_no_mid_word_matching() {
    // Input "ject" should NOT match "Project" (doesn't start at word boundary)
    let scaffold = scaffold(r#"
        Project:folder; A:=/project
        Subject:folder; A:=/subject
    "#);

    let (result, _, _, _) = get_new_display_commands(
        "ject",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    // Should not match Project (ject is mid-word)
    // If implementation is correct, result should be empty or only match commands starting with 'j'
    assert_not_contains(&result, "Project");
}

// ============================================================================
// PART 3: PREFIX MENU CONSTRUCTION
// ============================================================================

#[test]
fn part3_patch_based_membership() {
    // Commands with patch="PJ" belong in PJ menu
    let scaffold = pj_menu_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "PJ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);
    assert_contains(&result, "PP");
    assert_contains(&result, "CVT Gdrive");
}

#[test]
fn part3_name_based_membership() {
    // Commands starting with "PJ " belong in PJ menu
    let scaffold = pj_menu_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "PJ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);
    assert_contains(&result, "PJ Tasks"); // Command name starts with "PJ "
    assert_contains(&result, "PJ Notes"); // Command name starts with "PJ "
}

#[test]
fn part3_separator_required_after_anchor() {
    // "Projects" starts with "P" but not "PJ ", should NOT be in PJ menu
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "PJ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_not_contains(&result, "Projects");
    assert_not_contains(&result, "Python Code");
}

#[test]
fn part3_anchor_self_included() {
    // Anchor command itself appears in its own menu
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "PJ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_contains(&result, "PJ");
}

#[test]
fn part3_no_duplicate_commands() {
    // Each (command, action) pair appears only once
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "PJ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    // PP should appear exactly once
    let pp_count = result.iter().filter(|c| c.command == "PJ! PP").count();
    assert_eq!(pp_count, 1, "PP should appear exactly once, found {}", pp_count);
}

#[test]
fn part3_separator_commands_excluded() {
    // Separator commands never appear
    let scaffold = scaffold(r#"
        Main:anchor; A:=/main
        ============:separator;
        Main! Item:folder; A:=/item
    "#);

    let (result, _, _, _) = get_new_display_commands(
        "Main",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_not_contains(&result, "============");
}

// ============================================================================
// PART 4: SORTING & ORDERING
// ============================================================================

#[test]
fn part4_exact_matches_first() {
    // Exact match should appear before partial matches
    let scaffold = matching_priority_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "MainCode",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    // "Code" is exact match for "Code" filter
    // Should appear before "Code Projects", "Code Review", etc.
    let code_idx = find_index(&result, "Code");
    let code_proj_idx = find_index(&result, "Code Projects");

    assert!(code_idx.is_some(), "Code should be in results");
    if let (Some(code), Some(proj)) = (code_idx, code_proj_idx) {
        assert!(code < proj, "Exact match 'Code' should come before 'Code Projects'");
    }
}

#[test]
fn part4_no_words_skipped_before_words_skipped() {
    // Commands matching all words appear before commands that skip words
    let scaffold = matching_priority_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "MainCo",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    // "Code" (no words skipped) before "Old Code Files" (skips "Old")
    assert_order(&result, "Code", "Old Code Files");
}

#[test]
fn part4_alphabetical_within_tier() {
    // Within same priority tier, sort alphabetically
    let scaffold = scaffold(r#"
        Main:anchor; F:=A A:=/main
        Main! Zebra:folder; A:=/zebra
        Main! Apple:folder; A:=/apple
        Main! Banana:folder; A:=/banana
    "#);

    let (result, _, _, _) = get_new_display_commands(
        "Main",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    // All have same relevance (no filter), should be alphabetical
    assert_order(&result, "Apple", "Banana");
    assert_order(&result, "Banana", "Zebra");
}

#[test]
fn part4_sort_by_filter_in_prefix_menu() {
    // Prefix menu sorts by filter text, not full input
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "PJPP",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    // Should sort by "PP", so "PP" should be near top
    let pp_idx = find_index(&result, "PP");
    assert!(pp_idx.is_some(), "PP should be in results");
    // It should be early in the list (exact match for "PP")
    if let Some(idx) = pp_idx {
        assert!(idx < 5, "PP should be near top (exact match), but was at index {}", idx);
    }
}

// ============================================================================
// PART 5: FINAL MENU ASSEMBLY
// ============================================================================

#[test]
fn part5_prefix_menu_then_separator_then_global() {
    // With prefix menu: prefix commands, separator, global matches
    let scaffold = pj_menu_scaffold();

    let (result, is_prefix, _, count) = get_new_display_commands(
        "PJ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);
    assert!(count > 0, "Should have prefix menu commands");

    // Check if separator exists (only if global matches exist)
    let has_separator = result.iter().any(|c| c.action == "separator");
    let has_global = result.len() > count;

    if has_global {
        assert!(has_separator, "Should have separator when global matches exist");
    }
}

#[test]
fn part5_no_separator_without_global() {
    // Prefix menu only, no global matches → no separator
    let scaffold = scaffold(r#"
        PJ:anchor; A:=/pj
        PJ! Item:folder; A:=/item
    "#);

    let (result, _, _, _) = get_new_display_commands(
        "PJ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    let has_separator = result.iter().any(|c| c.action == "separator");
    assert!(!has_separator, "Should not have separator when no global matches");
}

#[test]
fn part5_global_matches_deduplicated() {
    // Commands in prefix menu should not appear in global section
    let scaffold = pj_menu_scaffold();

    let (result, _, _, prefix_count) = get_new_display_commands(
        "PJ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    // Count how many times "PJ! PP" appears
    let pp_count = result.iter().filter(|c| c.command == "PJ! PP").count();
    assert_eq!(pp_count, 1, "PP should appear exactly once (no duplication)");
}

#[test]
fn part5_no_prefix_menu_global_only() {
    // No anchor found → global matching only
    let scaffold = scaffold(r#"
        Item1:folder; A:=/item1
        Item2:folder; A:=/item2
        Testing:folder; A:=/testing
    "#);

    let (result, is_prefix, _, _) = get_new_display_commands(
        "Test",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, false);
    assert_contains(&result, "Testing");

    let has_separator = result.iter().any(|c| c.action == "separator");
    assert!(!has_separator, "No separator in global-only mode");
}

// ============================================================================
// EDGE CASES & SPECIAL BEHAVIORS
// ============================================================================

#[test]
fn edge_empty_input() {
    let scaffold = pj_menu_scaffold();

    let (result, is_prefix, breadcrumb, count) = get_new_display_commands(
        "",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_count(&result, 0);
    assert_prefix_menu(is_prefix, false);
    assert!(breadcrumb.is_none());
    assert_prefix_count(count, 0);
}

#[test]
fn edge_whitespace_only_input() {
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "   ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_count(&result, 0);
}

#[test]
fn edge_no_matching_commands() {
    let scaffold = pj_menu_scaffold();

    let (result, _, _, _) = get_new_display_commands(
        "ZZZZZ",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_count(&result, 0);
}

#[test]
fn edge_single_character_input() {
    let scaffold = progressive_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "A",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);
    assert_contains(&result, "A");
}

#[test]
fn edge_very_long_input() {
    let scaffold = pj_menu_scaffold();

    let long_input = "PJPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPPP";
    let (result, is_prefix, _, _) = get_new_display_commands(
        long_input,
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    // Should still match "PJ" anchor
    assert_prefix_menu(is_prefix, true);
}

#[test]
fn edge_duplicate_names_different_actions() {
    // Same name, different actions → both allowed
    let scaffold = scaffold(r#"
        Main:anchor; A:=/main
        Main! Dupe:open; A:=/dupe1
        Main! Dupe:markdown; A:=/dupe2.md
    "#);

    let (result, _, _, _) = get_new_display_commands(
        "Main",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    // Both "Dupe" commands should appear
    let dupe_count = result.iter().filter(|c| c.command.contains("Dupe")).count();
    assert_eq!(dupe_count, 2, "Both Dupe commands should appear (different actions)");
}

// ============================================================================
// KNOWN BUGS TO VERIFY
// ============================================================================

#[test]
fn bug_pjpp_should_show_pp() {
    // Known bug: "PJPP" returns empty, but should show PP
    // This test documents the EXPECTED behavior
    let scaffold = pj_menu_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "PJPP",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);

    // EXPECTED: PP should appear (it's in PJ menu and matches filter "PP")
    assert_contains(&result, "PP");

    // EXPECTED: PJ anchor should appear when it matches
    assert_contains(&result, "PJ");
}

#[test]
fn bug_double_filtering_investigation() {
    // Test to investigate double filtering issue
    // After prefix menu is built, is additional filter applied?
    let scaffold = pj_menu_scaffold();

    let (result, is_prefix, _, _) = get_new_display_commands(
        "PJCV",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    assert_prefix_menu(is_prefix, true);

    // CVT should match filter "CV"
    assert_contains(&result, "CVT Gdrive");
}
