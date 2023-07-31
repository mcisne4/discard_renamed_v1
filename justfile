# Watch: 'rs_dev'
dev: build
  @cd rs_dev && cargo watch -x run

# Run Tauri in dev mode
tauri: build-errors && build-fs build-rename
  @yarn tauri dev

# ===================== #
# === BUILD SCRIPTS === #
# ===================== #

# Build: 'rs_dev' and all dependencies
build: build-errors && build-fs build-rename build-dev
  @cargo build

# Build: All
build-all:
  @cargo build

# Build: 'rs_dev'
build-dev:
  @cargo build -p rs_dev

# Build: 'rs_fs'
build-fs:
  @cargo build -p rs_fs

# Build: 'rs_rename'
build-rename:
  @cargo build -p rs_rename

# Build: 'rs_errors'
build-errors:
  @cargo build -p rs_errors

# =========================== #
# === MODULE TREE SCRIPTS === #
# =========================== #

# Module Tree: All
mods: mods-dev && mods-fs mods-rename mods-errors

# Module Tree: 'rs_dev'
mods-dev:
  @cargo-modules generate tree -p rs_dev --types --traits --fns

# Module Tree: 'rs_fs'
mods-fs:
  @cargo-modules generate tree -p rs_fs --types --traits --fns

# Moduel Tree: 'rs_rename'
mods-rename:
  @cargo-modules generate tree -p rs_rename --types --traits --fns

# Module Tree: 'rs_errors'
mods-errors:
  @cargo-modules generate tree -p rs_errors --types --traits --fns
