# SchemaOrg Rust Workspace Architecture

## Overview
This repository implements a high-performance, strongly-typed Schema.org engine in Rust with WebAssembly bindings and a Genkit integration adapter.

## Workspace Layout
- `crates/schemaorg-vocabulary`: Static metadata lookup tables for terms, domains, ranges, inheritance trees, and codegen-produced typed structs.
- `crates/schemaorg-codegen`: Code generation utility to parse the official Schema.org JSON-LD definitions snapshot and output Rust types and metadata tables.
- `crates/schemaorg-core`: Core semantic engine, dynamic document model (`DynamicEntity`), parser, serializer, validation engine, and conservative hydration engine.
- `crates/schemaorg-html`: HTML Microdata extractor and converter to JSON-LD objects.
- `crates/schemaorg-wasm`: `wasm-bindgen` WebAssembly interface exposing hydration, validation, parsing, and HTML extraction to JS/TS.
- `integrations/genkit`: Node.js / TypeScript adapter for Google Genkit with Zod schema utilities and WASM-backed validation and hydration.
