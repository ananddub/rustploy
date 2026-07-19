# Rustploy Shell scripting macro (`sh!`)

`rustploy_sh_macros` is a powerful procedural macro DSL (`sh!`) that compiles a clean, type-safe, Rust-like syntax into a robust shell script representation (`ShellIR`). The macro guarantees type-safety and syntax validation at compile-time, shielding against shell-injection vulnerabilities and syntax errors.

---

## Table of Contents
1. [Core Design Concepts](#core-design-concepts)
2. [Syntax Reference](#syntax-reference)
   - [Command Invocation](#command-invocation)
   - [Variables & Scope](#variables--scope)
   - [Dynamic Rust Interpolation (`rust!`)](#dynamic-rust-interpolation-rust)
   - [Conditionals (`if` / `else if` / `else`)](#conditionals-if--else-if--else)
   - [Output Redirection & Chaining](#output-redirection--chaining)
3. [Supported Package Managers](#supported-package-managers)
4. [Comprehensive Examples](#comprehensive-examples)

---

## Core Design Concepts

When you write scripts inside `sh!`, the macro parses your code and outputs a `Vec<ShellIR>` (where `ShellIR` represents commands, assignments, loops, and conditions).
* **Compile-time Scope Validation**: All variable usage is checked at compile-time to prevent undefined variable references.
* **Auto-Quoting and Escaping**: String arguments are safely single-quoted to eliminate command-injection risks.
* **Platform Agnostic Abstractions**: Provides native DSL helpers for files, ports, networks, locks, and package managers.

---

## Syntax Reference

### Command Invocation

You can run commands using the `cmd(...)` helper, or built-in utility wrappers like `echo`, `sleep`, or `exit`.

```rust
let script = sh!(
    cmd("git", "clone", "https://github.com/example/repo.git");
    echo("Repository cloned successfully!");
    sleep(5);
    exit(0);
);
```

### Variables & Scope

You can declare bash-level variables using standard Rust `let` bindings. The macro ensures at compile-time that any referenced variable has been declared first.

```rust
let script = sh!(
    let port_num = "8080";
    cmd("nc", "-l", port_num); // Translates to: nc -l $port_num
);
```

### Dynamic Rust Interpolation (`rust!`)

Since the macro parses tokens at compile-time, it cannot directly read the runtime values of local Rust variables (like `self.name` or dynamic options).
To inject dynamic Rust expressions into the generated shell script, use the **`rust!(...)`** pseudo-macro. The inner expression will be evaluated in Rust at runtime:

```rust
let package_name = String::from("nginx");

let script = sh!(
    // Injects the runtime value of package_name into the script
    cmd("apt-get", "install", "-y", rust!(package_name));
);
```

> [!NOTE]
> Any expression inside `rust!(...)` must implement the `IntoCommand` trait (which includes standard `String` and `&str`).

---

### Conditionals (`if` / `else if` / `else`)

You can write conditional checks using Rust-like `if` statements. The condition can be a command execution:

```rust
let script = sh!(
    if cmd("command", "-v", "systemctl").stdout("/dev/null") {
        cmd("systemctl", "restart", "nginx");
    } else {
        cmd("service", "nginx", "restart");
    }
);
```

---

### Output Redirection & Chaining

Every command supports builder-like methods for redirecting stdout and stderr:

* `.stdout(destination)`: Redirects standard output.
* `.stderr(destination)`: Redirects standard error.

```rust
let script = sh!(
    // Discards standard output and redirects errors to a log file
    cmd("apt-get", "update").stdout("/dev/null").stderr("/var/log/update_errors.log");
);
```

---

## Supported Package Managers

The DSL natively intercepts package manager calls to translate them into type-safe representations. It fully supports **10 distinct package managers**:

| Package Manager | DSL Helper | Example Method Call |
|-----------------|------------|---------------------|
| **APT** (Debian/Ubuntu) | `apt()` | `apt().install("curl")` |
| **APK** (Alpine) | `apk()` | `apk().install("curl")` |
| **DNF** (Fedora) | `dnf()` | `dnf().install("curl")` |
| **YUM** (RHEL/CentOS) | `yum()` | `yum().install("curl")` |
| **PACMAN** (Arch) | `pacman()` | `pacman().install("curl")` |
| **ZYPPER** (openSUSE) | `zypper()` | `zypper().install("curl")` |
| **XBPS** (Void Linux) | `xbps()` | `xbps().install("curl")` |
| **EMERGE** (Gentoo) | `emerge()` | `emerge().install("curl")` |
| **NIX** (NixOS/Generic) | `nix()` | `nix().install("curl")` |
| **BREW** (macOS) | `brew()` | `brew().install("curl")` |
| **Generic/Auto-detect** | `package()` | `package().install("curl")` |

### Package Manager Operations

The package manager DSL helpers support the following methods:
* `.install(package_name)`: Installs a package.
* `.remove(package_name)`: Uninstalls/removes a package.
* `.update_index()`: Synchronizes/updates the package database.
* `.upgrade_all()`: Upgrades all installed packages.
* `.clean()`: Cleans cached package files.

```rust
let script = sh!(
    // 1. Specific package manager commands:
    apt().update_index();
    apt().install("nginx");
    
    // 2. Cross-platform auto-detecting installation command:
    // This dynamically checks for the present manager at runtime!
    package().install("curl");
);
```

---

## Comprehensive Examples

### 1. Complex Cross-Platform Package Setup

This script dynamically checks for a package manager, updates indices, installs a package with custom configurations, and falls back gracefully with helpful logging.

```rust
let pkg = "git";
let custom_install_options = "--no-cache";

let script = sh!(
    if cmd("command", "-v", "apk").stdout("/dev/null") {
        cmd("apk", "add", rust!(custom_install_options), rust!(pkg));
    } else if cmd("command", "-v", "apt-get").stdout("/dev/null") {
        apt().update_index();
        apt().install(rust!(pkg));
    } else if cmd("command", "-v", "brew").stdout("/dev/null") {
        brew().install(rust!(pkg));
    } else {
        echo("No supported package manager found!").stderr("/dev/stderr");
        cmd("exit", "1");
    }
);
```

### 2. Multi-Operation Service Init

```rust
let service_name = "docker";

let script = sh!(
    let svc = rust!(service_name);
    if cmd("systemctl", "is-active", svc).stdout("/dev/null") {
        echo("Service is already running!");
    } else {
        echo("Starting service...");
        cmd("systemctl", "start", svc);
    }
);
```
