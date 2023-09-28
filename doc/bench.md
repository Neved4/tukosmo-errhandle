## Benchmarks

### Compile Time

| Command              |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
| :------------------- | ---------: | -------: | -------: | ----------: |
| `thiserror`          | 37.1 ± 0.6 |     36.2 |     38.5 | 1.00 ± 0.03 |
| `custom_error`       | 37.1 ± 0.6 |     36.1 |     38.2 | 1.00 ± 0.03 |
| `build_domain_error` | 37.0 ± 0.8 |     35.9 |     39.4 |        1.00 |

### Runtime

| Command              |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
| :------------------- | ---------: | -------: | -------: | ----------: |
| `thiserror`          |  1.8 ± 0.2 |      1.7 |      3.1 | 1.02 ± 0.09 |
| `custom_error`       |  1.7 ± 0.0 |      1.7 |      2.0 |        1.00 |
| `build_domain_error` | 14.8 ± 0.3 |     14.4 |     15.9 | 8.56 ± 0.27 |

### Code Size

| Command              | Lines | Bytes |
| :------------------- | ----: | ----: |
| `custom_error`       |   400 |  4675 |
| `thiserror`          |   374 |  4876 |
| `build_domain_error` |   719 |  9949 |

### Setup Info

- Hardware: `MacBookPro17,1`
- OS: `macOS 14.0`
- CPU arch: `arm64`
- Rust: `1.74.0-nightly`
