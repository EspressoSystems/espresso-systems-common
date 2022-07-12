Constants used by the Espresso Systems company. This is stored in a central area to make sure we
don't accidentally introduce duplicates.

These are grouped by project. In the descendent projects you can re-export the module and use all
the constants in there.

```ignore
// in CAP
use espresso_system_common::cap;

#[tagged_blob(cap::ASSET_SEED)]
pub struct AssetCodeSeed(...);
```

The initial motivation for this repository is to provide a central location for strings used for
domain separation and for tags identifying the types associated with base64-encoded data. However,
this is also a good place for text and artwork related to branding.
