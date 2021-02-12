## Domain


```yaml
# crate metadata
name: rand                    # crate name
version: 0.8.3                # crate version
dependencies:                 # crate dependency
  - name: libc                # crate name
    requirement: ^0.2.22      # crate version requirement
    type: build|dev|normal    # crate dependency type
```

## Postgres

```yaml
# crate_metadata
- id: 1
  name: rand
  version: 0.8.3
  dependencies: 1 # checksum to avoid using transactions

# crate_dependency
- id: 1
  crate_id: 1
  name: libc
  requirement: ^0.2.22
  type: dev|build|normal
```
