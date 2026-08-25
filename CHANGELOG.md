# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `when(predicate, handler)`: runs a handler's `run` only for conns matched by
  a predicate; all other lifecycle hooks delegate unconditionally.
- `unhalt(handler)`: runs a handler but prevents it from halting the conn;
  response mutations flow onward, and a conn that was already halted stays
  halted.
