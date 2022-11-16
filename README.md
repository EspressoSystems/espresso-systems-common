# Features

By default this crate is `no_std` compatible.

|Feature  |Description|Default?|
|---------|-----------|--------|
|`std`    |Enables `std` in this crate. Required for `metrics`|no|
|`metrics`|Enables the [`Metrics`](#metrics) interface (note: enables `std`)|no|

# Constants

Constants used by the Espresso Systems company. This is stored in a central area to make sure we
don't accidentally introduce duplicates.

These are grouped by project. In the descendent projects you can re-export the module and use all
the constants in there.

```rs
// in CAP
use espresso_system_common::cap::tag;

#[tagged_blob(tag::ASSET_SEED)]
pub struct AssetCodeSeed(...);
```

The initial motivation for this repository is to provide a central location for strings used for
domain separation and for tags identifying the types associated with base64-encoded data. However,
this is also a good place for text and artwork related to branding.

# Metrics

With the `metrics` feature enabled, this crate exposes the `Metrics` trait.

```rs
pub trait Metrics: Send + Sync {
    fn create_counter(
        &self,
        label: String,
        unit_label: Option<String>
    ) -> Box<dyn Counter>;
    fn create_gauge(
        &self,
        label: String,
        unit_label: Option<String>
    ) -> Box<dyn Gauge>;
    fn create_histogram(
        &self,
        label: String,
        unit_label: Option<String>
    ) -> Box<dyn Histogram>;
    fn create_label(&self, label: String) -> Box<dyn Label>;
    fn subgroup(&self, subgroup_name: String) -> Box<dyn Metrics>;
}
```

This trait can be used to gather metrics throughout espresso applications.

**note**: the `metrics` feature enables `std`, making this crate unusable in `no_std` environments.