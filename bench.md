## Benchmarks

### Compile Time

| Command              |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
| :------------------- | ---------: | -------: | -------: | ----------: |
| `thiserror`          | 34.8 ± 1.6 |     33.9 |     44.8 | 1.01 ± 0.05 |
| `custom_error`       | 34.6 ± 0.6 |     33.9 |     37.7 | 1.00 ± 0.02 |
| `build_domain_error` | 34.5 ± 0.4 |     34.1 |     36.0 |        1.00 |

### Runtime

| Command              |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
| :------------------- | ---------: | -------: | -------: | ----------: |
| `thiserror`          |  1.7 ± 0.1 |      1.6 |      2.2 |        1.00 |
| `custom_error`       |  1.8 ± 0.4 |      1.7 |      7.9 | 1.03 ± 0.24 |
| `build_domain_error` | 15.1 ± 0.3 |     14.4 |     16.2 | 8.81 ± 0.37 |

### Code Size

| Command              | Lines | Bytes |
| -------------------- | ----: | ----: |
| `custom_error`       |   400 |  4675 |
| `thiserror`          |   374 |  4876 |
| `build_domain_error` |   719 |  9949 |

### Setup Info

- Hardware: `MacBookPro17,1`
- OS: `macOS 13.6`
- CPU arch: `arm64`
- Rust: `1.74.0-nightly`
