# infra/models/

Placeholder for model-weights the worker loads at boot (the CPU/CUDA
selfie/face embedder, per [ADR 0010](../../docs/adr/0010-inference-runtime-worker-cuda-api-cpu.md)).
No weights are committed here — slice 7 picks the concrete model and
its fetch/mount strategy.
