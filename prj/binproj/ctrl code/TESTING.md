# ctrl Testing & Deployment Workflow

## Quick Reference

### Test Before Release

```bash
cd "/Users/oblinger/ob/kmr/prj/binproj/ctrl code/ctrl-package"
pipx install . --suffix=-test --force
ctrl-test --help
pipx uninstall ctrl
```

### Deploy to PyPI

```bash
cd "/Users/oblinger/ob/kmr/prj/binproj/ctrl code/ctrl-package"

# 1. Update version in pyproject.toml and src/ctrl/__init__.py
# 2. Build and upload
python -m build
twine upload dist/*
```

### Your Dev Version

Always works via symlink:
```bash
ctrl --help  # Uses ~/bin/ctrl -> ctrl code/ctrl
```
