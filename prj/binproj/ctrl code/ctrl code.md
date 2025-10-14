.[[ctrl code]].  >[[binproj]]

[[ctrl medium-article]] 

# INFO
## 2025-10-13  Test & Deploy PyPI version

  Test:
  cd ctrl-package
  pipx install . --suffix=-test --force
  ctrl-test jgetlist  # Test packaged version
  ctrl jgetlist       # Your dev version still works
  pipx uninstall ctrl

  Deploy:
  cd ctrl-package
  python -m build
  twine upload dist/*
  

# LOG

## 2025-10-13  To Publish


  1. Test Local Installation

  cd "/Users/oblinger/ob/kmr/prj/binproj/ctrl code/ctrl-package"
  pipx install .

  This will install ctrl in an isolated environment and make it available globally.

  2. Test the Command

  ctrl --help
  ctrl jgetlist

  1. Publish to PyPI (When Ready)

  # Install build tools
  pip install build twine

  # Build the package
  python -m build

  # Upload to PyPI (you'll need a PyPI account)
  twine upload dist/*

  After publishing to PyPI, anyone can install with:
  pipx install ctrl

  The package name on PyPI will be ctrl (as configured), and it provides the ctrl CLI command!


