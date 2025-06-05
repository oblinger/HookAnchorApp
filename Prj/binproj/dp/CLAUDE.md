# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Dan Project (DP) documentation repository that defines a personal project management methodology. The project consists primarily of documentation files describing how "Dan Projects" are structured and organized.

## Project Structure

- `dp` - Python executable script that launches Claude Code and optionally PyCharm
- `dp.md` - Main documentation file describing the Dan Project methodology
- Projects following this methodology typically have:
  - A project folder with unique filename serving as project name
  - A markdown file with same name as folder (XXX/XXX.md)
  - Optional GitHub-linked git repo
  - Optional .claude folder for Claude configuration
  - Optional .idea folder for PyCharm configuration

## Script Functionality

The `dp` script is designed to:
1. Check if the current directory contains a `.idea` folder (PyCharm project)
2. If found, open the project in PyCharm using `open -a PyCharm .`
3. Launch Claude Code using `claude code`

This allows for seamless integration between PyCharm IDE and Claude Code for Dan Projects that use PyCharm.

## Development Context

This is part of a larger "BinProj" collection of script projects that are linked by a personal bin folder. The parent directory contains other related projects like "Budg" (budget management) and "km" (knowledge management).