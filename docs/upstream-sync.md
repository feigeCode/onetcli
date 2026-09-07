# Syncing Upstream

How the `navop-gpui-ce` fork absorbs new work from upstream
`feigeCode/gpui-component` `main`.

## Why syncing is not a plain merge

The fork keeps its own packaging while upstream has moved on:

| | this fork (`navop-gpui-ce`) | upstream `main` |
|---|---|---|
| Crate packages | `gpui_ce_components*`, version `0.2.0` | `gpui-component`/`gpui-kit*`, version `0.6.0` |
| GPUI dependency | `gpui-ce` (git, `feigeCode/gpui-ce`) | `gpui-pre*` (crates.io) |
| Component crate path | `crates/ui` | `crates/component` |
| Imports in code | `use gpui::…; use gpui_component::…` | `use gpui_kit::component::…` / `use gpui_kit::*` |
| Time source | `web_time` | `instant` (older commits) / `web_time` |

A direct `git merge origin/main` therefore produces repo-wide conflicts
(directory renames, repackaging, wholesale rebrands) that are mostly noise.
Instead, sync by cherry-picking the upstream commits you actually want.

## Procedure

### 1. Start clean

```bash
git -C gpui-component status --short      # expect clean, or…
git -C gpui-component stash push -m "wip"  # …stash local work first
git -C gpui-component fetch origin main
```

### 2. List what is new upstream

```bash
git -C gpui-component log --oneline --reverse HEAD..origin/main
```

The fork already contains most upstream history under different hashes
(previous syncs/merges), so many entries here are effectively already
applied. `git cherry-pick` reports those as empty; just `--skip` them.

### 3. Classify before picking

Categories to **skip** outright (they are upstream-lane, not fork value):

- Rebrand / packaging: `Rebrand to GPUI Kit`, `chore: Use gpui-kit`,
  `gpui-pre:*`, CI/publish changes, `move the native examples`,
  `isolate story examples from test-support`.
- Website / docs / skills series (Astro migration, App Stories, site copy).
- Land-and-revert pairs (e.g. a feature commit plus its immediate revert).

Everything else (shell runtime, base/editor/input fixes, markdown, dock,
component fixes, fps, icon) is candidate work for the fork. When unsure,
ask which groups to take rather than assuming.

### 4. Pick in topological order, resolving conflicts fork-style

```bash
git -G gpui-component cherry-pick <sha>…
```

On conflict, apply the fork conventions below, then:

```bash
git add <resolved files> && git cherry-pick --continue
# or, when the commit is already present:
git cherry-pick --skip
```

## Conflict-handling conventions

- **Import blocks.** Drop `use gpui_kit::…` / `use gpui_kit::*` and keep the
  fork's `use gpui::{…}` + `use gpui_component::{…}` (or `gpui_base::…`),
  adding whichever items the merged body hunks need (`Placement`,
  `Icon`, `red_500`, `StyleRefinement`, `radians`, …). Resolve the imports,
  then re-add any missing name the compiler reports.
- **Path mapping.** Upstream paths `crates/component/…` map onto the fork's
  `crates/ui/…`. Git usually follows the rename automatically; when a
  `crates/component/src/…` phantom appears (modify/delete), `git rm` it and
  apply the change to the fork path by hand.
- **Package names.** If a conflict resolution lands on upstream's
  `Cargo.toml` wholesale, restore the fork package identity:
  `name = "gpui_ce_components_*"`, `version = "0.2.0"` (check
  `[workspace.dependencies]` in the root `Cargo.toml` for the declared name).
- **`instant` vs `web_time`.** The fork migrated to `web_time`; upstream
  commits written against `instant` need `use instant::…` rewritten to
  `use web_time::…` (the fork has no `instant` dependency).
- **`gpui::ColorExt`.** Fork GPUI (`gpui-ce`) exposes `Hsla::opacity` through
  the `ColorExt` trait. If an upstream merge drops the `ColorExt as _`
  import from a file that still calls `.opacity(…)`, re-add it.
- **Heavily forked files** (expect real conflicts every time upstream
  touches them):
  - `crates/ui/src/icon.rs` — keep the fork structure (`IconSize`,
    `IconColorMode`, `file_path`, functional/object icon wrappers). Port new
    upstream capabilities (e.g. `Icon::data` / `IconSource`) in fork style
    rather than adopting upstream's refactor wholesale.
  - `crates/base/src/history.rs` + `undo_history.rs` — upstream split
    navigation (simple `History`) from undo (`UndoHistory`) in #2923; keep
    that split, it already landed.
  - `crates/shell/src/*` and `crates/fps/src/*` — shell carries fork runtime
    customizations; fps history says follow upstream wholesale when a commit
    reworks the sampler/monitor.
- **Local WIP.** `crates/shell/src/lib.rs` (`with_current` export) and
  `crates/shell/src/typings.rs` (element-method overloads) are local work in
  progress. Stash them before syncing and pop after, and keep them out of
  sync commits.

## Verify

```bash
cargo check -p gpui_ce_components_base \
            -p gpui_ce_components \
            -p gpui_ce_components_fps \
            -p gpui_ce_components_shell \
            -p gpui-component-story
```

Cargo will update `Cargo.lock` for any new dependencies; commit that
separately. Known caveat: `gpui-component-story` has a handful of
pre-existing errors (`.opacity` without a `Colorize`/`ColorExt` import and a
missing `InputEvent::GutterMarkerMouseDown` arm) that predate any sync; the
four production crates above are the green bar.

## Keeping this document honest

When a sync finishes, update the **baseline** below so the next run starts
from the right point, and record any commit you deliberately left out.

- Last synced upstream: `main` tip `cbdf5baa` (`input: Multi cursors`).
  Applied via cherry-pick (2026-09): #2904 #2908 #2909 #2910 #2920 #2922
  #2923 #2928 #2931 #2954 #2956 #2957 #2971 #2973 #2974 #2975 #2980 #2984,
  plus one reconciliation commit. Skipped: rebrand/packaging (#2927 #2929
  #2936 #2937 #2940 #2963 #2966 #2982 #2985 and the gpui-pre/CI series),
  website/docs/skills series, and the land/revert fps pair #2915/#2916.
  Commits already present in the fork from an earlier sync were skipped as
  empty (#2906 #2911 #2912 #2913 #2918 #2919 #2921 #2938 #2939 #2941 #2944
  #2945 #2946 #2947 #2952 #2953 #2955 #2969 #2837 and more).
- Deferred decisions: adopt upstream packaging wholesale (drops CE identity,
  makes future syncs nearly free) vs keep CE packaging (this document).
