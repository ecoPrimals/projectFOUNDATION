# sporeprint/ — Hand-Off Status

**Wave 157a**: Gallery content is being handed off to lithoSpore
(pseudoSpore packaging) per `specs/HANDOFF_MAP.md`.

## Contents

- `validation-summary.md` — Generated validation summary
- `spores/` — Generated pseudoSpore gallery pages

## Migration Target

**lithoSpore** repo (`gardens/lithoSpore`) — publication artifacts belong
with the publication pipeline.

The `foundation-publish` crate continues to generate gallery pages but
will write to lithoSpore's tree rather than this local directory.

Files remain here until the lithoSpore team confirms absorption.
