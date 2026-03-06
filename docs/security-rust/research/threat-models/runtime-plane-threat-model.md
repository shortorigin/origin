# Runtime Plane Threat Model

## Scope

The runtime plane covers Wasmtime-executed guest modules, wasmCloud component bindings, and workflow-driven service mutations that emit evidence.

## Boundaries

- Public edge to control plane
- Control plane to runtime plane
- Runtime plane to data plane

## Threats

- malformed guest outputs that break parser assumptions
- oversized guest state that exceeds memory policy
- capability or interface drift between component descriptors and WIT expectations
- workflow mutations that lose evidence or trace correlation

## Primary Controls

- explicit runtime policy limits
- service and workflow interface bindings
- evidence manifests
- policy and approval enforcement
