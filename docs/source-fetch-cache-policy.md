# Source Fetch Cache Policy

ROUTE source fetches must preserve existing cache evidence unless a replacement
has been successfully fetched, parsed, and written.

## Rules

1. Scoped fetches merge the refreshed scope into the existing cache. A command
   such as `route fetch-hpms --states TX` may replace TX rows, but it must
   preserve non-TX rows.
2. Full fetches may replace the full artifact, but only after the new artifact
   is complete enough to pass the command's validation floor.
3. Fetch writers use a temporary file and then replace the target after the
   payload is validated and fully flushed. A failed HTTP request, parse error,
   empty response, or partial write must leave the previous cache intact.
4. Live snapshot fetches may overwrite the current snapshot path because the
   path represents the latest observed feed, but they still use temp-then-replace
   so failed fetches do not destroy the last usable snapshot.
5. Commands that intentionally rebuild derived artifacts from source caches are
   not source fetches. Their gates should still make the dependency explicit,
   as with pavement acquisition using `route build --all-roads`.

This policy applies before optimizer, bundle, Beck, game, or release artifacts
consume the source. Cache mutation should never silently reduce evidence scope.
