## Benchmarks

#### Compile time

| Command              |  Mean [ms] | Min [ms] | Max [ms] |    Relative |
| :------------------- | ---------: | -------: | -------: | ----------: |
| `thiserror`          | 34.8 ± 0.5 |     34.1 |     37.6 | 1.00 ± 0.02 |
| `custom_error`       | 34.8 ± 0.6 |     34.1 |     37.8 |        1.00 |
| `build_domain_error` | 35.4 ± 2.1 |     34.2 |     50.8 | 1.02 ± 0.06 |

#### Runtime

| Command              | Mean [ms] | Min [ms] | Max [ms] |    Relative |
| :------------------- | --------: | -------: | -------: | ----------: |
| `thiserror`          | 0.9 ± 0.2 |      0.9 |      3.6 | 1.02 ± 0.19 |
| `custom_error`       | 0.9 ± 0.0 |      0.9 |      1.0 | 1.00 ± 0.05 |
| `build_domain_error` | 0.9 ± 0.0 |      0.9 |      1.1 |        1.00 |

#### Size

| Command              | Lines | Bytes |
| -------------------- | ----: | ----: |
| `custom_error`       |   378 |  4443 |
| `thiserror`          |   352 |  4644 |
| `build_domain_error` |   697 |  9715 |

#### Setup

- Hardware: `MacBookPro17,1`
- OS: `macOS 13.6`
- CPU arch: `arm64`
- Rust: `1.74.0-nightly`
