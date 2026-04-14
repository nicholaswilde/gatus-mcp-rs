# Implementation Plan: Refactor Functions for Token Efficiency

## Phase 1: Research & Tool Mapping [checkpoint: e392461]
- [x] Task: Map existing tools to new consolidated definitions. (94cc2ab)
    - [x] Analyze `src/mcp.rs` tool definitions.
    - [x] Identify common patterns and potential for parameterization.
    - [x] Design the new "Management" and "Metrics" tools.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Research & Tool Mapping' (Protocol in workflow.md) (e392461)

## Phase 2: Consolidation Implementation [ ]
- [x] Task: Implement the consolidated "Management" tool. (fc3205a)
    - [x] Write failing tests for the new tool (Red Phase).
    - [x] Implement tool logic in `src/mcp.rs` (Green Phase).
    - [x] Refactor and verify (Refactor Phase).
- [x] Task: Implement the consolidated "Metrics" tool. (ac2dedc)
    - [x] Write failing tests for the new tool (Red Phase).
    - [x] Implement tool logic in `src/mcp.rs` (Green Phase).
    - [x] Refactor and verify (Refactor Phase).
- [x] Task: Conductor - User Manual Verification 'Phase 2: Consolidation Implementation' (Protocol in workflow.md) (6057b2c)

## Phase 3: Payload Optimization & Cleanup [checkpoint: ea23b64]
- [x] Task: Optimize tool response payloads (thinning).
    - [x] Write failing tests for optimized output (Red Phase).
    - [x] Update `src/fmt.rs` and tool handlers to return minimal high-signal data (Green Phase).
    - [x] Refactor and verify (Refactor Phase).
- [x] Task: Remove deprecated granular tools.
    - [x] Remove old tool definitions from `src/mcp.rs`.
    - [x] Cleanup obsolete tests.
    - [x] Verify full project stability with `task test`.
- [x] Task: Conductor - User Manual Verification 'Phase 3: Payload Optimization & Cleanup' (Protocol in workflow.md) (ea23b64)

## Phase 4: Finalization [ ]
- [x] Task: Update documentation.
    - [x] Update `README.md` with the new tool reference.
    - [x] Update `Product Definition` if necessary.
- [x] Task: Conductor - User Manual Verification 'Phase 4: Finalization' (Protocol in workflow.md) (0494aa9)
