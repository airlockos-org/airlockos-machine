//! Declare the physical USB machine's VMs and explicit network, display and capture relationships.

use crate::{
    components,
    model::{
        AuthoringError, GiB, HermitApplication, HostPlatform, Machine, MachineSnapshot, MiB, Vm,
    },
};

/// Connect the declared physical-machine VMs before freezing their validated identities and resources.
pub fn machine() -> Result<MachineSnapshot, AuthoringError> {
    let mut machine = Machine::default();
    machine.set_host_platform(HostPlatform::UsbLab);

    let network = Vm::network("sys-net", GiB(2))?;
    network.set_feature("ipv6", "1")?;
    machine.add(&network)?;

    let firewall = Vm::hermit_firewall("sys-firewall", &network)?;
    firewall.set_autostart(true);
    firewall.set_feature("ipv6", "1")?;
    machine.add(&firewall)?;

    // Keep management, builds and the optional UpdatesProxy on Linux. Both
    // paths still depend on sys-net; native failure never reassigns a client.
    let recovery = Vm::firewall("sys-firewall-old", &network)?;
    recovery.set_feature("ipv6", "1")?;
    machine.add(&recovery)?;

    let home = Vm::personal("usr-home", &firewall, GiB(16))?;
    home.set_feature("ipv6", "1")?;
    machine.add(&home)?;

    let gui = Vm::physical_gui("sys-gui", &home)?;
    gui.add_module(components::desktop::gnome());
    components::screen_capture::connect(&gui, &home);
    machine.add(&gui)?;

    let controller = Vm::controller("sys-ctrl", &recovery, GiB(32))?;
    machine.add(&controller)?;

    let assistant = Vm::assistant("usr-assistant", &recovery, GiB(16))?;
    machine.add(&assistant)?;

    let clipboard = Vm::hermit("sys-clipboard", MiB(24), HermitApplication::Clipboard)?;
    clipboard.set_autostart(true);
    machine.add(&clipboard)?;
    components::clipboard::connect(&gui, &clipboard);

    machine.set_host(&gui, &firewall, &controller);
    machine.set_updates_proxy(&recovery);
    components::diagnostics::enable(&mut machine, &assistant);
    machine.finish()
}
