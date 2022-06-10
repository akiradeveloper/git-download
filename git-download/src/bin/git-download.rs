use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser, serde::Deserialize)]
#[clap(name = "git-download")]
struct Opts {
    #[clap(long, default_value = "download.toml")]
    file: PathBuf,
}

mod toml_file {
    use std::path::PathBuf;

    #[derive(serde::Deserialize)]
    pub struct Root {
        pub dependencies: Vec<Dependency>,
    }
    #[derive(serde::Deserialize)]
    pub struct Dependency {
        pub repo: String,
        pub branch: String,
        pub src: PathBuf,
        pub dst: PathBuf,
    }
}

#[derive(Debug)]
struct Copy {
    src: PathBuf,
    dst: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let toml_path = opts.file;
    let toml_contents = std::fs::read_to_string(toml_path)?;
    let toml_root: toml_file::Root = toml::from_str(&toml_contents)?;

    let mut h = HashMap::new();
    for dep in toml_root.dependencies {
        let k = (dep.repo, dep.branch);
        let v = Copy {
            src: dep.src,
            dst: dep.dst,
        };
        h.entry(k).or_insert(vec![]).push(v);
    }

    let mut downloders = vec![];
    for ((repo, branch), copies) in h {
        let mut x = git_download::repo(repo).branch_name(branch);
        for copy in copies {
            x = x.add_file(copy.src, copy.dst);
        }
        downloders.push(x);
    }

    for downloader in downloders {
        downloader.exec()?;
    }

    Ok(())
}
