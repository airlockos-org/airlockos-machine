# Editing this machine

- Edit `vms/VM.nix` for a NixOS VM's packages, services and settings. Each existing
  file is imported automatically for its named VM; local imports and assets can
  live alongside it. Commit all referenced files so review and builds capture them.
  The Hermit clipboard VM does not run NixOS and has no customization module.
- Edit `src/machine.rs` for VM declarations, connections and enabled features.
  `src/main.rs` is the preparation adapter; shared implementations and validation
  machines belong to the pinned `airlock-config` library.
- Read `/etc/airlock/usr-assistant/AGENTS.md` before inspecting or changing an
  installed system. Follow its baseline, approval, testing and deployment rules;
  a clean checkout or successful build does not establish deployment.
- Preserve Qubes/Xen architecture. Keep machine structure in Rust/Rusix and
  user-owned VM settings in NixOS modules. Keep generated Nix outside Git.
  Commit each coherent configuration change.
- From an Airlock development checkout, follow the parent
  [AGENTS.md](../AGENTS.md), including its Rust standards, and
  [development operations](../docs/development-operations.md). Shared installed
  guidance is authored in `crates/airlock-config/assets/usr-assistant/` there.
- Keep this checkout focused on this computer. Do not add generic application
  code, operating guides or alternative machine examples here.
