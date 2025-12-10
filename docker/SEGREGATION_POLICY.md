# Docker Test Environment Segregation Policy

## Purpose
This document defines the segregation between the Frame Transpiler and Frame Debugger teams' use of Docker containers in the shared test environment (`framepiler_test_env`).

## Namespace Segregation

### Transpiler Team Resources
All transpiler resources MUST use the `transpiler` namespace:

- **Container Names**: `frame-transpiler-*` prefix
  - Example: `frame-transpiler-python-20241208-143022`
  - Pattern: `frame-transpiler-<language>-<timestamp>`

- **Image Names**: `frame-transpiler/*` repository
  - Example: `frame-transpiler/test-prt:latest`
  - Registry: `ghcr.io/frame-lang/transpiler/*` (when published)

- **Networks**: `frame-transpiler-test-net-*`
  - Subnet: `172.28.0.0/16` (transpiler reserved)
  - Example: `frame-transpiler-test-net-20241208-143022`

- **Volumes**: `frame-transpiler-*` prefix
  - Example: `frame-transpiler-results-20241208-143022`

- **Labels**: All resources tagged with `frame.component=transpiler`

### Debugger Team Resources
All debugger resources MUST use the `debugger` namespace:

- **Container Names**: `frame-debugger-*` prefix
- **Image Names**: `frame-debugger/*` repository
- **Networks**: `frame-debugger-*`
  - Subnet: `172.29.0.0/16` (debugger reserved)
- **Volumes**: `frame-debugger-*` prefix
- **Labels**: All resources tagged with `frame.component=debugger`

## File System Segregation

### Shared Test Environment Structure
```
framepiler_test_env/
├── transpiler/                    # Transpiler-only area
│   ├── fixtures/
│   │   ├── python/
│   │   ├── typescript/
│   │   └── rust/
│   ├── results/
│   │   └── <test-run-id>/
│   └── framec/                    # Transpiler binary
│       └── framec
├── debugger/                      # Debugger-only area  
│   ├── fixtures/
│   ├── results/
│   └── debug-adapter/
└── shared/                        # Shared utilities (read-only)
    └── tools/
```

### Container Mount Points

**Transpiler Containers**:
- `/transpiler/framec` - Transpiler binary mount
- `/transpiler/fixtures` - Test fixtures mount
- `/transpiler/results` - Results output

**Debugger Containers**:
- `/debugger/adapter` - Debug adapter mount
- `/debugger/fixtures` - Test fixtures mount
- `/debugger/results` - Results output

## Environment Variables

### Transpiler-Specific
- `FRAME_TEST_NAMESPACE=transpiler`
- `FRAME_TEST_COMPONENT=<component>`
- `FRAME_TRANSPILER_VERSION=<version>`

### Debugger-Specific
- `FRAME_TEST_NAMESPACE=debugger`
- `FRAME_TEST_COMPONENT=<component>`
- `FRAME_DEBUGGER_VERSION=<version>`

## Network Isolation

### IP Ranges
- Transpiler: `172.28.0.0/16` (172.28.0.1 - 172.28.255.254)
- Debugger: `172.29.0.0/16` (172.29.0.1 - 172.29.255.254)
- Shared services: `172.30.0.0/16` (if needed)

### Port Ranges
- Transpiler test services: 9000-9499
- Debugger test services: 9500-9999
- Shared monitoring: 8000-8099

## Resource Limits

To prevent one team from starving the other:

### Per-Team Limits
```yaml
transpiler_limits:
  cpus: '8.0'          # Max 8 CPUs
  memory: 16GB         # Max 16GB RAM
  containers: 20       # Max 20 concurrent containers
  
debugger_limits:
  cpus: '8.0'          # Max 8 CPUs  
  memory: 16GB         # Max 16GB RAM
  containers: 20       # Max 20 concurrent containers
```

## Conflict Detection

The run scripts MUST check for conflicts before starting:

1. Check if other team's containers are running
2. Verify network namespaces don't overlap
3. Ensure mount points are not cross-mounted
4. Validate resource availability

## Cleanup Policy

### Automatic Cleanup
- Containers older than 24 hours: auto-removed
- Networks without containers: auto-removed after 1 hour
- Volumes without references: preserved for 7 days

### Manual Cleanup Commands
```bash
# Transpiler team cleanup (preserves debugger resources)
docker ps -a | grep "frame-transpiler-" | awk '{print $1}' | xargs docker rm -f
docker network ls | grep "frame-transpiler-" | awk '{print $1}' | xargs docker network rm
docker volume ls | grep "frame-transpiler-" | awk '{print $2}' | xargs docker volume rm

# Debugger team cleanup (preserves transpiler resources)
docker ps -a | grep "frame-debugger-" | awk '{print $1}' | xargs docker rm -f
docker network ls | grep "frame-debugger-" | awk '{print $1}' | xargs docker network rm
docker volume ls | grep "frame-debugger-" | awk '{print $2}' | xargs docker volume rm
```

## Monitoring

Each team should monitor their resource usage:

```bash
# Transpiler team monitoring
docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}" \
  $(docker ps -q --filter "label=frame.component=transpiler")

# Debugger team monitoring  
docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}" \
  $(docker ps -q --filter "label=frame.component=debugger")
```

## Enforcement

1. CI/CD pipelines MUST use the appropriate namespace
2. Pull requests that violate segregation will be rejected
3. Regular audits to ensure compliance
4. Alerts when resource limits are approached

## Emergency Procedures

If one team's tests are blocking the other:

1. Identify blocking resources: `docker ps -a | grep frame-`
2. Contact team via Slack: #frame-testing-conflicts
3. If unresponsive (>30 min), force cleanup with team prefix
4. Document incident in shared log

## Version History

- v1.0.0 (2024-12-08): Initial segregation policy
- Authors: Frame Transpiler & Debugger Teams