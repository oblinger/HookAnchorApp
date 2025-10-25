# ⚠️ CRITICAL: URL HANDLING DOCUMENTATION ⚠️

## DO NOT MODIFY URL HANDLING WITHOUT READING THIS DOCUMENT

### What Went Wrong (Multiple Times)

1. **System-wide lockups**: Incorrect URL handling caused Obsidian to lock up at 100% CPU for hours
2. **Lost productivity**: User lost 4+ hours of work time on multiple occasions
3. **The specific mistake**: Direct app execution from URL schemes shows the GUI popup, which blocks the calling application

### How macOS URL Handling Works

1. **URLs are NOT passed via command line arguments**
   - When `open hook://test` is called, macOS launches the app with NO arguments
   - The URL is sent via Apple Events, not command line

2. **Info.plist Registration**
   - Apps register URL schemes via `CFBundleURLTypes` in Info.plist
   - Once registered, macOS will launch that app for those URLs
   - The app launches in GUI mode by default

### The Current Architecture Problem

```
User clicks hook://cmd → macOS → Launches HookAnchor.app → Shows GUI popup → Obsidian hangs
```

This happens because:
- The Info.plist registers `hook` as a URL scheme
- macOS launches the app directly
- The app has no way to know it was launched for a URL (no command line args)
- It shows the GUI popup
- The calling app (Obsidian) waits forever for the URL handler to complete

### The Correct Solution: Bootstrap Wrapper

The app bundle should NOT directly handle URLs. Instead:

1. **Remove `CFBundleURLTypes` from Info.plist** - The app should NOT register URL schemes
2. **Create a separate URL handler** that:
   - Registers the URL scheme
   - Writes the URL to a marker file
   - Does NOT launch the GUI
   - Exits immediately

3. **Alternative: Wrapper Script in App Bundle**
   ```bash
   #!/bin/bash
   # hookanchor_wrapper - This runs instead of the main app
   for arg in "$@"; do
       if [[ "$arg" == hook://* ]]; then
           echo "$arg" > /tmp/hookanchor_url_launch
           # Now launch the real app to process it
           /Applications/HookAnchor.app/Contents/MacOS/hookanchor_real
           exit 0
       fi
   done
   # Not a URL, launch normally
   exec /Applications/HookAnchor.app/Contents/MacOS/hookanchor_real "$@"
   ```

### Current Problematic Files

1. `/Users/oblinger/ob/proj/HookAnchor/scripts/create_app_bundle.sh`
   - Lines 66-78: Creates Info.plist with `CFBundleURLTypes`
   - This causes direct GUI launch on URL handling

2. `/Users/oblinger/ob/proj/HookAnchor/src/ha.rs`
   - Lines 87-99: Tries to detect URL handling via marker file
   - But this only works if a wrapper writes the file first

### Testing URL Handling Safely

1. **NEVER register URL handlers in /Applications or ~/Applications during testing**
2. **Test with a separate test app bundle in /tmp**
3. **Always unregister test handlers immediately after testing**
4. **Check Launch Services database before and after**:
   ```bash
   lsregister -dump | grep -i "hook:"
   ```

### Emergency Recovery

If Obsidian locks up again:
1. Kill Obsidian: `killall Obsidian`
2. Find registered handlers: `lsregister -dump | grep -B10 "hook:" | grep path:`
3. Unregister them: `lsregister -u "/path/to/app"`
4. Remove the app if needed

### Implementation Checklist

- [ ] Remove `CFBundleURLTypes` from all Info.plist files
- [ ] Create a separate minimal URL handler app/script
- [ ] Test URL handling without affecting system stability
- [ ] Verify Obsidian and other apps work normally
- [ ] Document the final working solution

### Remember

**Every time you see `CFBundleURLTypes` in an Info.plist, that app will intercept URLs and potentially lock up the system. Remove it!**