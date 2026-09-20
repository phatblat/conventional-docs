# Pick the site style

## Problem

`site/assets/css/` carries three preview themes (`theme-*.css`) and a
switcher (`params.stylePreview: true`) for evaluating them, but no winner has
been chosen, so the site still ships the evaluation UI instead of a decided
look.

## Outcome

One of the three themes is kept, the other two `theme-*.css` files are
deleted, `params.style` names the winner, and `params.stylePreview` is `false`
— removing the switcher partial and its bootstrap script from the shipped
site.
