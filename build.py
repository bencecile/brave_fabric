import argparse
import subprocess

class Device:
    def __init__(self, name, target, is_desktop):
        self.name = name
        self.target = target
        self.is_desktop = is_desktop
    def __repr__(self):
        return self.name
    def __hash__(self):
        return hash(self.name)
    def __eq__(self, rhs):
        return self.name == rhs.name
class Devices:
    Android = Device("android", "aarch64-linux-android", False)
    # Only a subset of the projects will be available on desktops
    Windows = Device("windows", "x86_64-pc-windows-msvc", True)
    def all():
        return [
            Devices.Android,
            Devices.Windows,
        ]
    def from_str(device_name):
        try:
            return {
                Devices.Android.name: Devices.Android,
                Devices.Windows.name: Devices.Windows,
            }[device_name]
        except KeyError:
            return Device("Match Nothing")

def main():
    parser = argparse.ArgumentParser("Brave Fabric Builder")
    subparsers = parser.add_subparsers()

    build_parser = subparsers.add_parser("build", help="Builds all or some projects")
    build_parser.add_argument("device", choices=Devices.all(), type=Devices.from_str)
    build_parser.set_defaults(fn=build_for_device)

    run_parser = subparsers.add_parser("run", help="Runs the main thing for that device")
    run_parser.add_argument("device", choices=Devices.all(), type=Devices.from_str)
    run_parser.set_defaults(fn=run_for_device)

    # TODO We will want to be able to install (release build and packaged)

    args = parser.parse_args()
    fn = args.fn
    fn_args = { key: value for (key, value) in vars(args).items() if key != "fn" }
    fn(**fn_args)

def build_for_device(device):
    if device.is_desktop:
        build_for_desktop(device)
    else:
        build_for_android(device)
def build_for_desktop(device):
    build_projects(device, [
        "brave_fabric_desktop",
        "brave_emulator",
    ])
def build_for_android(device):
    print("Building for Android is not supported right now")

def build_projects(device, projects):
    for project in projects: build(device, project)
def build(device, project_name):
    subprocess.run([
        "cargo", "+nightly",
        "build",
        "--target", device.target,
        "-p", project_name,
    ], check=True)

def run_for_device(device):
    if device.is_desktop:
        run_for_desktop(device)
    else:
        # Maybe we could start up the emulator for these devices?
        print("Running non-desktop is currently not supported")
        exit(1)
def run_for_desktop(device):
    build_for_desktop(device)

    build_path = f"target/{device.target}/debug/"
    desktop_program_name = {
        Devices.Windows: "brave_fabric_desktop.exe",
    }[device]
    subprocess.run([
        build_path + desktop_program_name,
    ], cwd=build_path, check=True)

if __name__ == "__main__":
    main()
