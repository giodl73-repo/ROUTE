# I-80 camera-tour proof

This owner-authored REEL package demonstrates a bounded camera tour over
ROUTE's generated `maps/i80.png` structural map.

It is a research-lab explainer, not a project announcement. ROUTE retains
authority over the map, waypoint meaning, narration, claim posture, and release
review. REEL owns only timing, interpolation, bounded crop execution, and
artifact evidence.

`reel-dependency.yaml` pins the exact REEL v0.2.45 implementation used by this
proof.

Render from the ROUTE repository root after installing the pinned REEL release:

```powershell
reel animatic-render media/reel/i80-camera-tour/manifest.yaml `
  --asset-root . `
  --silent `
  --captions media/reel/i80-camera-tour/captions.srt `
  --transition-seconds 0 `
  --output renders/i80-camera-tour.mp4
```

The controlling claim boundaries remain:

- `docs/map-publication-scope.md`
- `gaps/i80-flagship.md`
- `docs/media/media-claim-guide.md`
