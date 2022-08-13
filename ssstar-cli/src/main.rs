use clap::{ArgGroup, Parser, Subcommand};
use std::path::PathBuf;
use tracing::debug;
use url::Url;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Operation to perform
    #[clap(subcommand)]
    command: Command,

    #[clap(flatten)]
    global: Globals,
}

/// Arguments that apply regardless of command
#[derive(Parser, Debug)]
struct Globals {
    /// Enable verbose log output
    #[clap(short = 'v', long, conflicts_with = "quiet", global = true)]
    verbose: bool,

    /// Be quiet, suppress almost all output (except errors)
    #[clap(short = 'q', long, conflicts_with = "verbose", global = true)]
    quiet: bool,

    /// The number of async task worker threads in the thread pool.
    ///
    /// This value has a reasonable default selected based on the specifications of the system on
    /// which it runs and should never be overridden except in rare cases.  If you're not sure what
    /// to set this to, don't set it at all.
    #[clap(long, global = true)]
    worker_threads: Option<usize>,

    /// The maximum number of threads used to run blocking tasks.
    ///
    /// This value has a reasonable default selected based on the specifications of the system on
    /// which it runs and should never be overridden except in rare cases.  If you're not sure what
    /// to set this to, don't set it at all.
    #[clap(long, global = true)]
    max_blocking_threads: Option<usize>,

    #[clap(flatten)]
    config: ssstar::Config,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a new tar archive from S3 objects
    #[clap(group(ArgGroup::new("output").required(true)))]
    Create {
        /// Write the tar archive to a file.
        #[clap(short = 'f', long, value_parser, group = "output")]
        file: Option<PathBuf>,

        /// Write the tar archive to an S3 object.
        ///
        /// The URL should specify the S3 bucket as well as the object name.  This URL will be used
        /// verbatim without any processing by ssstar.
        #[clap(short = 's', long, value_parser, group = "output", value_name = "URL")]
        s3: Option<Url>,

        /// Write the tar archive to stdout
        #[clap(short = 't', long, group = "output")]
        stdout: bool,

        /// One or more S3 URLs pointing to a bucket, a prefix, a specific object, or a glob.
        ///
        /// Each URL must be a URL starting with `s3://`, and can specify just a bucket, a bucket
        /// and an object path, or globs.
        ///
        /// EXAMPLES:
        ///
        /// s3://foo/         - Read all objects from the bucket `foo`
        ///
        /// s3://foo/bar/     - Read all objects from the bucket `foo` with the prefix `bar/`
        ///
        /// s3://foo/bar      - Read the object `bar` in the bucket `foo`
        ///
        /// s3://foo/*.txt    - Read the all objects in the root of bucket `foo` with extension `.txt`
        ///
        /// s3://foo/**/*.txt - Read the all objects in any directory of bucket `foo` with
        ///                     extension `.txt``
        ///
        /// NOTE: When specifying globs, make sure to enclose the entire URL in "", otherwise your
        /// shell might expand the globs locally and produce unintended results.
        #[clap(value_parser, required = true, value_name = "URLS")]
        objects: Vec<Url>,
    },

    /// Extract a tar archive, storing the results in S3
    #[clap(group(ArgGroup::new("input").required(true)))]
    Extract {
        /// Read the tar archive to extract from a file
        #[clap(short = 'f', long, value_parser, group = "input")]
        file: Option<PathBuf>,

        /// Read the tar archive to extract from an S3 object
        ///
        /// The URL should specify the S3 bucket as well as the object name.  This URL will be used
        /// verbatim without any processing by ssstar.
        #[clap(short = 's', long, value_parser, group = "input", value_name = "URL")]
        s3: Option<Url>,

        /// Read the tar archive to extract from stdin
        #[clap(short = 't', long, group = "input")]
        stdin: bool,

        /// URL of S3 bucket (and optional prefix) to which archive will be extracted.
        ///
        /// Every file in the archive will be written as an S3 object with this URL prepended.
        ///
        /// For example if the URL is s3://foo/bar/, and the tar archive contains a single file
        /// a/b/c/d.txt, then the S3 object s3://foo/bar/a/b/c/d.txt will be created.  Note that a
        /// trailing "/" isn't implied.  To continue the previous example, if you specify the URL
        /// s3://foo/bar (without the trailing '/'), then the object s3://foo/bara/b/c/d.txt is
        /// created.
        #[clap(value_parser, value_name = "URL")]
        target: Url,
    },
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    // If verbose output is enabled, enabling logging.
    // If not, log events will be ignored
    if args.global.verbose {
        use tracing_subscriber::prelude::*;
        use tracing_subscriber::{fmt, EnvFilter};

        // Configure a custom event formatter
        let format = fmt::layer()
            .with_level(true) // include level in output
            .with_target(true) // targets aren't that useful but filters operate on targets so they're important to know
            .with_thread_ids(false) // thread IDs are useless when using async code
            .with_thread_names(false) // same with thread names
            .with_timer(fmt::time::LocalTime::rfc_3339());

        // Get the log filter from the RUST_LOG env var, or if not set use a reasonable default
        let filter = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new("h2=warn,debug"))
            .unwrap();

        // Create a `fmt` subscriber that uses our custom event format, and set it
        // as the default.
        tracing_subscriber::registry()
            .with(filter)
            .with(format)
            .with(fmt::Layer::new().with_writer(std::io::stderr))
            .init();
    }

    // Report panics with the prettier color-eyre
    color_eyre::install().unwrap();

    // Set up the tokio runtime
    let mut builder = tokio::runtime::Builder::new_multi_thread();

    debug!(?args.global.worker_threads, ?args.global.max_blocking_threads, "Initializing tokio runtime");

    if let Some(worker_threads) = args.global.worker_threads {
        builder.worker_threads(worker_threads);
    }
    if let Some(max_blocking_threads) = args.global.max_blocking_threads {
        builder.max_blocking_threads(max_blocking_threads);
    }

    let rt = builder.enable_all().build().unwrap();

    rt.block_on(async move {
        tracing::info!("This is an info message bitch!");

        println!("Hello, world!");

        println!("{:#?}", args);

        color_eyre::Result::<()>::Ok(())
    })?;

    Ok(())
}
