<div align="center">
<br/>

# ai-sandbox

### Isolated execution environment for untrusted AI agents and user-submitted code on Sorabbyngo OS.

<br/>

[![Division](https://img.shields.io/badge/Division-Sorabbyngo_OS-6C3FC4?style=for-the-badge)](https://github.com/sorabbyngo-os)
[![Lang](https://img.shields.io/badge/Lang-Rust-AA66FF?style=for-the-badge)](https://github.com/sorabbyngo-os/ai-sandbox)
[![Status](https://img.shields.io/badge/Status-In_Development-FF6B35?style=for-the-badge)](https://github.com/sorabbyngo-os/ai-sandbox)

</div>

---

## What It Does

ai-sandbox provides a secure, resource-limited execution environment for AI agent code, user scripts, and untrusted workloads on Sorabbyngo OS. It uses Linux namespaces, seccomp-bpf, and cgroups to enforce strict isolation without full VM overhead.

---

## Features

| Feature | Description |
|---|---|
| **Namespace Isolation** | PID, net, mount, user, IPC namespaces per sandbox |
| **Seccomp-bpf Filtering** | Syscall allowlist enforced via BPF — deny by default |
| **Cgroup Resource Limits** | CPU, memory, and I/O caps per sandbox |
| **Filesystem Isolation** | Overlay FS — sandboxes cannot see host filesystem |
| **Network Policy** | Per-sandbox egress rules via CML data flow |
| **Execution Timeout** | Hard time limits with graceful and forced termination |
| **Audit Trail** | All sandbox executions logged to CML audit store |

---

## Architecture

```
ai-sandbox/
├── src/
│   ├── sandbox/      # Sandbox lifecycle — create, run, destroy
│   ├── namespace/    # Linux namespace setup
│   ├── seccomp/      # seccomp-bpf filter generation
│   ├── cgroups/      # Resource limit management
│   ├── fs/           # Overlay filesystem setup
│   ├── network/      # Sandbox network policy
│   └── audit/        # Execution audit logging
├── Cargo.toml
└── README.md
```

---

## Security Model

Every sandbox is a separate Linux namespace group. The host kernel is the only shared surface. Seccomp filters are generated at sandbox creation time based on the declared workload type (AI agent, script, WASM). CML enforces data classification on all sandbox I/O.

---

## Stack

- **Language:** Rust
- **Isolation:** Linux namespaces + seccomp-bpf + cgroups v2
- **OS target:** Sorabbyngo OS (all surfaces)
- **Dependencies:** `cml`, `sentinel-runtime`, `identity-core`

---

*Sorabbyngo OS · Kisii, Kenya · 2026*
