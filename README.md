# Machine configuration

Customize each NixOS VM through its named module in [vms/](vms/). These are ordinary
NixOS modules for packages, services and settings, composed with Airlock's defaults.
Files imported by a module belong in this repository and must be committed too.

Edit [src/machine.rs](src/machine.rs) for VM identities, connections and enabled
Airlock features. [GUI border settings](assets/sys-gui/gnome/gui-style.json)
are a separate machine setting. Shared implementation, validation
machines and operating guidance come from the pinned Airlock application source.

The GitHub repository is `airlockos-org/airlockos-machine`. The parent development
checkout currently retains the local submodule path `airlock-machine/`.

On an installed system this checkout lives at `~/airlockos-machine` in
`usr-assistant`. Read [AGENTS.md](AGENTS.md) before changing it. The installed
guide at `/etc/airlock/usr-assistant/AGENTS.md` explains inspection, source
approval, deployment and verification; local commits do not require a GitHub push.

`make prepare` emits an inspectable Nix build tree outside Git. It does not
build or deploy the system. Application dependencies and their offline Cargo
sources are supplied by the installed authoring environment.
Nix-only VM edits reuse the Rust authoring program; they still require NixOS
evaluation and the normal reviewed build and deployment.

Original configuration is [MIT licensed](LICENSE). See [SECURITY.md](SECURITY.md)
and [third-party notices](THIRD_PARTY.md) for security and dependency terms.
