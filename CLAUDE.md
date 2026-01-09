# CLAUDE.md - cnctd_git

> Brief reference for the Git operations library.

## Purpose

Multi-provider Git operations supporting GitHub, GitLab, and Bitbucket for repository management and API interactions.

## Key Exports

```rust
pub enum GitProvider {
    GitHub,
    GitLab,
    Bitbucket,
}

pub mod account;  // Account operations
pub mod api;      // API interactions
pub mod repo;     // Repository operations
```

## Usage Example

```rust
use cnctd_git::{GitProvider, repo};

// Clone repository
repo::clone("https://github.com/user/repo.git", "./local")?;

// Commit and push
repo::add_all()?;
repo::commit("Update files")?;
repo::push()?;
```

## Ecosystem Role

- **Used by**: cnctd_cli (git workflows)
- **Depends on**: git2, cnctd_rest

## Version

**0.1.6**

---

*Part of the cnctd monorepo. See `../../../CLAUDE.md` for ecosystem context.*
