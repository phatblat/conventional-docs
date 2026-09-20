# Add the site URL and badge at go-live

## Problem

Once the site is live, `package.json` has no `homepage`, `README.md` has no
badge or link to it, and the README still carries sections the site now owns
in a better-navigable form.

## Outcome

`package.json`'s `homepage` points at the live site, `README.md` carries the
site badge and link, and the README sections the site now owns are trimmed
to a pointer rather than duplicated in full.
