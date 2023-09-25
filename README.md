## Cleaner Tukosmo Error Handling

This proposal repository contains a Proof of Concept (PoC) that leverages crates to streamline error handling in [`Tukosmo`][1]. While the code is ready for implementation, it's advisable to first extend `DomainError` accordingly.

### Overview

Currently, the process of generating error messages involves a significant amount of repetitive code, much like what can be seen at [tls.rs (L31-L229)](https://github.com/lajtomekadimon/tukosmo/blob/56465a8e6f8cb482b3ffc3b2cabe70e3c25ec3b9/server/src/core/shared/infrastructure/leptos_actix/service/tls.rs#L31-L229). Here we present idealized versions of how the code is being handled and how it could be refactored using crates. These implementations are located at [src/bin](src/bin).

### Crate Selection

Since right now there's not a strict policy, the following criteria was used when choosing crates:

- **Compiles without errors or warnings.**
- **Compatible with Rust 2021 Edition.**
- **Supports WASM.**

###### These can be used as a starting point for future project reference guidelines, with any modifications necessary.

Based on the above, the following crates were investigated:
| Crate          | Size (lines) | Maintained | Builds | WASM | 2021 Edition |
| -------------- | -----------: | ---------- | -----: | ---: | -----------: |
| `custom_error` |       `1044` | Not active |      ✅ |    ✅ |            ✅ |
| `thiserror`    |       `3500` | Regularly  |      ✅ |    ✅ |            ✅ |

Running `cargo tree` on [Tukosmo][1] shows that `thiserror` is already in use. No new dependencies need to be added to the project if we choose this crate:

```text
tukosmo_hydration
└── leptos
    └── leptos_config
        └── config
            └── json5
                └── pest
                    └── thiserror
```

Ultimately, choosing any of the crates listed will reduce space usage by a factor of `2X`. At the current scale there are no significant compile or runtime differences. See [Benchmarks](bench.md).

For comparison, a small sample on how these would reduce boilerplate follows:

https://github.com/Neved4/tukosmo-errhandle/blob/ad1d498ab048214328a617f78cfa7b2b18c5e8e7/src/bin/build_domain_error.rs#L68-L77

https://github.com/Neved4/tukosmo-errhandle/blob/ad1d498ab048214328a617f78cfa7b2b18c5e8e7/src/bin/thiserror.rs#L5-L9

https://github.com/Neved4/tukosmo-errhandle/blob/ad1d498ab048214328a617f78cfa7b2b18c5e8e7/src/bin/custom_error.rs#L5-L8

### Summary
Overall, using a crate or macro will greatly reduce the need for boilerplate code. While `custom_error` codebase is simpler, `thiserror` seems more actively kept and is already integrated by the project. Both codebases are small enough, making it feasible to investigate potential issues. Additionally, reimplementing the equivalent functionality within the project remains a viable option.

[1]: https://github.com/lajtomekadimon/tukosmo
