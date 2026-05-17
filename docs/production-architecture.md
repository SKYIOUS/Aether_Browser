# Production Architecture

This document outlines the high-level architecture for Aether Browser's production-ready design, including process isolation and GPU acceleration.

## Process Isolation
- Basic IPC mechanisms are defined in `src/engine/process/`. This module will be extended to facilitate secure communication between the browser kernel and renderer processes.

## GPU Acceleration
- Preliminary hooks are implemented in `src/engine/renderer/renderer.rs`. Future implementation will leverage `wgpu` to offload painting tasks to the GPU.

## Standards Compliance
- Tokenizer and CSS3 support are continuously evolving towards full specification coverage.
