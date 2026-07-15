# Documentation conventions

Per-language code-doc style. Anchored by ADR
[0018](../adr/0018-documentation-tooling.md).

## Python (worker, slice 7)

* **Toolchain:** Sphinx with the `myst-parser` extension.
  Docstrings are **Markdown**, matching the rest of the docs
  tree.
* **Style:** Google-style docstrings, rendered via napoleon.
  Type hints are picked up automatically by
  `sphinx-autodoc-typehints`.

Example:

```python
def embed_selfie(image: bytes) -> Embedding:
    """Compute a face embedding for the given selfie image.

    Args:
        image: JPEG- or PNG-encoded selfie bytes.

    Returns:
        A dense embedding vector suitable for pgvector search.

    Raises:
        NoFaceDetectedError: If no face was found in the image.
    """
```

## Rust (API, slice 6)

* **Toolchain:** `rustdoc` (default; no extra config needed).
* **Style:** `///` for item-level docs, `//!` for crate/module-
  level. Intra-doc links via `[SomeType]`.

Example:

```rust
/// Compute a face embedding for the given selfie image.
///
/// Returns [`Embedding`] suitable for pgvector search.
/// Errors with [`Error::NoFaceDetected`] if no face is present.
pub fn embed_selfie(image: &[u8]) -> Result<Embedding, Error> {
    // ...
}
```

## TypeScript (web, slice 8)

* **Toolchain:** TypeDoc.
* **Style:** TSDoc syntax. `@public` / `@internal` tags gate
  the generated API surface — internal helpers stay out of the
  published docs.

Example:

```ts
/**
 * Fetch photos matching the given share token.
 *
 * @param token - The opaque per-event share token.
 * @returns The list of media the token can access.
 * @public
 */
export async function fetchPhotos(token: string): Promise<Media[]> {
  // ...
}
```

## Publish target

Deferred to slice 9 (`arch-development-workflow`) — the three
tools each emit HTML and the `Makefile` can wire them into a
single publish step at that point.
