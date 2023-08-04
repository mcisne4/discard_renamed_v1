# Watch: 'rs_dev'
dev: build
  @cd rs_dev && cargo watch -x run

# Run Tauri in dev mode
tauri: build-response && build-db build-fs build-rename
  @yarn tauri dev

# ===================== #
# === BUILD SCRIPTS === #
# ===================== #

# Build: 'rs_dev' and all dependencies
build: build-response && build-db build-fs build-rename build-dev

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

# Build: 'rs_response'
build-response:
  @cargo build -p rs_response

# Build: 'rs_db'
build-db:
  @cargo build -p rs_db

# =========================== #
# === MODULE TREE SCRIPTS === #
# =========================== #

# Module Tree: All
mods: mods-db && mods-dev mods-fs mods-rename mods-response

# Module Tree: 'rs_db'
mods-db:
  @cargo-modules generate tree -p rs_db --types --traits --fns

# Module Tree: 'rs_dev'
mods-dev:
  @cargo-modules generate tree -p rs_dev --types --traits --fns

# Module Tree: 'rs_fs'
mods-fs:
  @cargo-modules generate tree -p rs_fs --types --traits --fns

# Moduel Tree: 'rs_rename'
mods-rename:
  @cargo-modules generate tree -p rs_rename --types --traits --fns

# Module Tree: 'rs_response'
mods-response:
  @cargo-modules generate tree -p rs_response --types --traits --fns

