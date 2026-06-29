---
name: Bug Report
about: Report a bug or unexpected behavior
title: "[BUG] "
labels: bug
body:
  - type: textarea
    id: description
    attributes:
      label: Description
      description: A clear description of the bug
    validations:
      required: true
  - type: textarea
    id: steps
    attributes:
      label: Steps to reproduce
      description: Steps to reproduce the behavior
    validations:
      required: true
  - type: textarea
    id: expected
    attributes:
      label: Expected behavior
      description: What you expected to happen
    validations:
      required: true
  - type: textarea
    id: actual
    attributes:
      label: Actual behavior
      description: What actually happened
    validations:
      required: true
  - type: input
    id: os
    attributes:
      label: OS
      placeholder: e.g. Ubuntu 24.04, macOS 14, Windows 11
    validations:
      required: true
  - type: input
    id: version
    attributes:
      label: Passman version
      placeholder: e.g. 0.1.0
    validations:
      required: true
  - type: dropdown
    id: component
    attributes:
      label: Component
      options:
        - Desktop app
        - CLI
        - Core library
        - Other
    validations:
      required: true
---

