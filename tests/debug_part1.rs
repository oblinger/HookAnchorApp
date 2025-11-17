mod test_helpers;

use test_helpers::*;
use hookanchor::core::get_new_display_commands;

#[test]
fn debug_abcd_should_match_abc() {
    let scaffold = scaffold(r#"
        A:anchor; F:=A A:=/path/a
        AB:anchor; F:=A A:=/path/ab
        ABC:anchor; F:=A A:=/path/abc

        A! Item1:folder; A:=/item1
        AB! Item2:folder; A:=/item2
        ABC! Item3:folder; A:=/item3

        ABCD:folder; A:=/abcd
    "#);

    println!("\n=== COMMANDS ===");
    for cmd in &scaffold.commands {
        println!("  '{}' | action={} | flags='{}' | is_anchor={} | patch='{}'",
            cmd.command, cmd.action, cmd.flags, cmd.is_anchor(), cmd.patch);
    }

    println!("\n=== PATCHES ===");
    for (key, patch) in &scaffold.patches {
        println!("  key='{}' name='{}' anchors={}", key, patch.name, patch.anchor_commands.len());
    }

    println!("\n=== TEST: Input 'ABCD' ===");
    let (result, is_prefix, breadcrumb, count) = get_new_display_commands(
        "ABCD",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    println!("\nRESULTS:");
    println!("  is_prefix_menu: {} (expected: true)", is_prefix);
    println!("  prefix_menu_count: {}", count);
    println!("  breadcrumb: {:?}", breadcrumb.as_ref().map(|(o, r)| (&o.command, &r.command)));
    println!("  commands returned: {}", result.len());

    for (i, cmd) in result.iter().enumerate() {
        println!("    [{}] '{}' | action={}", i, cmd.command, cmd.action);
    }

    println!("\nEXPECTED:");
    println!("  - Progressive scan: ABCD -> ABC -> AB -> A");
    println!("  - Should match ABC (longest anchor)");
    println!("  - is_prefix_menu should be TRUE");
    println!("  - Should show ABC and ABC! Item3");

    println!("\nACTUAL FAILURE:");
    if !is_prefix {
        println!("  ❌ is_prefix_menu is FALSE (should be TRUE)");
        println!("  This means build_prefix_menu returned None");
        println!("  OR get_new_display_commands didn't find a prefix menu");
    }
}

#[test]
fn debug_alias_t_should_resolve_to_target() {
    let scaffold = scaffold(r#"
        Target:anchor; F:=A A:=/target
        T:alias; A:=Target

        Target! File1:folder; A:=/file1
        Target! File2:folder; A:=/file2
    "#);

    println!("\n=== COMMANDS ===");
    for cmd in &scaffold.commands {
        println!("  '{}' | action={} | arg='{}' | is_anchor={}",
            cmd.command, cmd.action, cmd.arg, cmd.is_anchor());
    }

    println!("\n=== TEST: Input 'T' ===");
    let (result, is_prefix, breadcrumb, count) = get_new_display_commands(
        "T",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    println!("\nRESULTS:");
    println!("  is_prefix_menu: {} (expected: true)", is_prefix);
    println!("  breadcrumb: {:?}", breadcrumb.as_ref().map(|(o, r)| (&o.command, &r.command)));

    for (i, cmd) in result.iter().enumerate() {
        println!("    [{}] '{}' | action={}", i, cmd.command, cmd.action);
    }

    println!("\nEXPECTED:");
    println!("  - T is alias pointing to Target");
    println!("  - Should resolve T -> Target");
    println!("  - Target is anchor, so should create prefix menu");
    println!("  - is_prefix_menu should be TRUE");
}

#[test]
fn debug_pjpp_bug() {
    let scaffold = scaffold(r#"
        PJ:anchor; F:=UA A:=/Users/oblinger/ob/kmr/prj
        PJ! PP:markdown; F:=UA A:=/Users/oblinger/ob/kmr/prj/pp/PP.md
        PJ! Code:folder; F:=UA A:=/Users/oblinger/ob/proj
    "#);

    println!("\n=== COMMANDS ===");
    for cmd in &scaffold.commands {
        println!("  '{}' | action={} | patch='{}' | flags='{}'",
            cmd.command, cmd.action, cmd.patch, cmd.flags);
    }

    println!("\n=== PATCHES ===");
    for (key, patch) in &scaffold.patches {
        println!("  key='{}' name='{}'", key, patch.name);
    }

    println!("\n=== TEST: Input 'PJPP' ===");
    let (result, is_prefix, breadcrumb, count) = get_new_display_commands(
        "PJPP",
        &scaffold.commands,
        &scaffold.patches,
        &scaffold.config
    );

    println!("\nRESULTS:");
    println!("  is_prefix_menu: {} (expected: true)", is_prefix);
    println!("  prefix_menu_count: {}", count);
    println!("  breadcrumb: {:?}", breadcrumb.as_ref().map(|(o, r)| (&o.command, &r.command)));
    println!("  commands returned: {}", result.len());

    for (i, cmd) in result.iter().enumerate() {
        println!("    [{}] '{}' | patch='{}'", i, cmd.command, cmd.patch);
    }

    println!("\nEXPECTED:");
    println!("  - Progressive scan: PJPP -> PJP -> PJ");
    println!("  - Should match PJ anchor");
    println!("  - Filter text should be 'PP'");
    println!("  - Should show 'PJ! PP' (matches filter)");
    println!("  - is_prefix_menu should be TRUE");
}
