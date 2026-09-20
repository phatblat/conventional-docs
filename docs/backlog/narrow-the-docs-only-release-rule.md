# Narrow the docs-only patch release rule

## Problem

`.releaserc.json` releases every `docs:`-typed commit as a patch, which was
right while nothing else triggered a release, but means every documentation
tweak to a stable convention keeps cutting a release indefinitely — there is
no rule that narrows this once the convention stabilizes.

## Outcome

`.releaserc.json`'s release rules state, explicitly, which kinds of `docs:`
change still warrant a patch once the convention is no longer in its initial
draft phase, and which do not.
