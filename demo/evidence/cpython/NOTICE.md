# CPython demonstration material

Source: Python/CPython v3.14.7, commit `823f0323ee6ec1402088b73bce1a38473cac36dc`.
Repository: https://github.com/python/cpython

Copyright (c) 2001 Python Software Foundation; All Rights Reserved.

The accompanying `LICENSE` is copied unchanged from that revision and contains the applicable PSF and historical license texts. These notices apply to the upstream material represented in the patches, not as a newly chosen license for Shear.

`argparse.py` identifies Steven J. Bethard as its author and Raymond Hettinger as maintainer as of August 29, 2019. The original source and author headers remain in the disposable demonstration checkout.

## Changes represented here

- `argparse.patch`: Shear `8cf6242` removes four redundant alternatives and performs two short nested-condition merges in `Lib/argparse.py`. The previous guard rewrites and overlong merges are no longer included. Existing interfaces, helper bodies, comments, and tests are preserved. No new helper is introduced. Native whole-file reformatting was not applied.
- `locale-after-ruff.patch`: Shear moves the duplicated padding cleanup in `Lib/locale.py` into one shared continuation after Ruff 0.16.6 autofixes. This patch is relative to Ruff output, not pristine CPython; the exact configuration and source hashes are in `docs/NORMALIZATION_EXAMPLE.md`. Original copyright and author headers are retained.
- `urlparse.patch`: historical fallback demonstration (before the follow-up safety fixes) of the deterministic rules in `Lib/urllib/parse.py`, including port validation and defragmented-URL return paths.

These are demonstration patches, not contributions accepted by CPython or evidence of PSF endorsement. The claim is a checked alternative structural representation, not that Python was incorrect, slow, or optimally formatted before the change.
