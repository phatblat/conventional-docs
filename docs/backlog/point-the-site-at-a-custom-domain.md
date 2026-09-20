# Point the site at a custom domain

## Problem

If a custom domain is registered for this project, the site still serves
from its default GitHub Pages URL, because `baseURL` in `site/hugo.yaml` and
`site/static/CNAME` are not set up for one.

## Outcome

Once a custom domain is registered, `baseURL` in `site/hugo.yaml` is updated
and `site/static/CNAME` is added, and the site serves from that domain.
