# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.7.2 - 31-Dec-2023

### Changes

* Update `aws-sdk-*` crates to version `1.*.0`

## 0.7.1 - 20-Nov-2023

* Add ability to create empty archive (#124)

## 0.7.0 - 18-Aug-2023

### Changes

* Update to Rust version 1.71.0
* Update `aws-sdk-*` crates to version `0.27.0`
* Update `aws-smithy-*` crates to version `0.56`

## 0.6.0 - 07-Jul-2023

### Changes

* Add `CredentialsProvider` trait to support custom way to provide credentials

  The code that uses this crate as a lib can implement this trait and pass it into config

## 0.5.1 - 23-Jun-2023

### Changes

* Add `aws-sdk-sts v0.27.0` as a dependency for `ssstar` crate
* Add new `role_credentials_provider` module that implements auto refreshing credentials

  This provider assumes the role and gets the temporary credentials and refreshes the credentials when they are close
  to expiration.

## 0.5.0 - 22-May-2023

### Breaking Changes

* Rename several `CreateProgressCallback` methods to remove the `tar_` prefix to reflect the fact that the archive might
not be tar format in the future.
* Change `CreateProgressCallback` trait to add data `byte_offset` and object `timestamp` parameters to `archive_object_written` method

  This is a bit of an abuse of the progress callback since this information is not needed for progress reporting and
  instead will be used in Elastio to populate the index of the S3 backup, however it's an easy change to make that will
  facilitate our initial indexing impl without having to wait for the new storage API to stabilize.

### Changes

* Update AWS SDK crates from 0.54 to 0.55 and 0.24 to 0.27
* Update to Rust version 1.69.0
* Update to `rust-cache` action v2
* Remove `coverage` GHA job since it's consistently broken and the `tarpaulin` GHA seems abandoned
* Replace deprecated `tempfile` dependency with `tempdir`.  This is a dev-only dependency so this shouldn't have any
runtime impact.
* Add new `storage` module with new public API for writing to custom archive formats instead of tar

  This isn't actually implemented yet, but we're releasing the API in an experimental state to collect feedback.  This
  new storage API is subject to change and should not be relied upon until we finalize it.  For now it's hidden behind
  the `storage` feature and is not enabled by default.
* Move all dependencies to the workspace level to make it easier to update dependencies in the future.

## 0.4.3 - 3-Mar-2023

### Changes

* Add new field `user_agent` to `Config` to allow callers to customize the user agent when talking to S3 APIs.

  This is not a breaking change since `Config` is marked with the `non_exhaustive` attribute.

## 0.4.2 - 25-Feb-2023

### Breaking Changes

### Dependencies

* Update AWS SDK crates from 0.52 to 0.54
* Update Tokio to 1.25 and tokio-util to 0.7.7
* Update misc dependencies:
    * async-trait v0.1.57 -> v0.1.64
    * aws-smith-http to 0.54.4
    * base64 v0.20.0 -> v0.21.0
    * bytes v1.2.1 -> v1.4.0
    * clap v4.0.32 -> v4.1.6
    * futures v0.3.24 -> v0.3.26
    * http to 0.2.9
    * indicatif v0.17.1 -> v0.17.3
    * once_cell v1.15.0 -> v1.17.1
    * regex v1.6.0 -> v1.7.1
    * tokio-stream to 0.1.12
    * tracing-test v0.2.3 -> v0.2.4
    * vergen v7.4.2 -> v7.5.1
    * which v4.3.0 -> v4.4.0

## 0.4.1 - 16-Feb-2023

* Add `aws_session_token` option for `ssstar::Config` to allow to use AWS temporal credentials

## 0.4.0 - 27-Jan-2023

### Breaking Changes

* Add `Sync` trait requirement to the inner reader of `ssstar::SourceArchive::Reader(..)`.  Now, the inner reader
  requires implementation of traits: `Read + Send + Sync`

### Dependencies

* Update `base64` to 0.20

## 0.3.0 - 6-Jan-2023

### Fixes

* The performance of `extract` writing many files to object storage should be improved now.  In general, the `extract`
  speed should more or less match the performance of the `create` operation which produced the archive to be extracted.
* Integration tests are updated to work correctly with the multipart checksum logic in recent MinIO releases.

### Breaking Changes

* Increment MSRV to 1.64

### Dependencies

* Update AWS SDK for S3 from 0.17 to 0.22
* Update AWS SDK common crates like `aws-config`, `aws-http`, etc from 0.47 to 0.52
* Update `clap` from 3.x to 4.x

## 0.2.0 - 2-Sept-2022

### Breaking Changes

* Report average bytes per second rates at the end of each stage of operation.  This modifies the signature of the
  progress callback traits.

## 0.1.3 - 31-Aug-2022

First release published via GitHub Actions.  Functionally identical to 0.1.0.

## 0.1.2 - 31-Aug-2022

Incomplete release missing `ssstar-cli`

## 0.1.1 - 31-Aug-2022

Incomplete release missing `ssstar` and `ssstar-cli`

## 0.1.0 - 31-Aug-2022

Initial release
