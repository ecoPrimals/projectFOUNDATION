# benchmarks/ — Hand-Off Status

**Wave 157a**: CPU parity benchmarks are being handed off to barraCuda
(the GPU math primal) or benchScale (infra benchmarking framework in
projectNUCLEUS) per `specs/HANDOFF_MAP.md`.

## Contents

`barracuda_cpu_parity/` — 6 Python scripts, 32 test cases validating
that barraCuda GPU math produces identical results to CPU reference
implementations.

## Migration Target

**barraCuda** repo (`primals/barraCuda`) — benchmarks belong with the
primal they validate.

Files remain here until the barraCuda team confirms absorption.
