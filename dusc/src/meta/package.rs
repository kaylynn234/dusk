use super::{module::Module, DuskPath, Item};
use anyhow::{bail, Context, Result};
use parser::{AstNode, Parser};
use std::{collections::HashMap, fs, path::PathBuf};

#[derive(Debug)]
pub enum PackageKind {
    Binary,
    Library,
}

impl PackageKind {
    pub fn filename(&self) -> &'static str {
        match self {
            PackageKind::Binary => "main.dusk",
            PackageKind::Library => "lib.dusk",
        }
    }
}

#[derive(Debug)]
pub struct Package {
    kind: PackageKind,
    namespace: HashMap<DuskPath, Item>,
}

impl Package {
    pub fn new(kind: PackageKind) -> Self {
        Package {
            kind,
            namespace: HashMap::new(),
        }
    }

    pub fn build_module_tree(&mut self, path: PathBuf) -> Result<()> {
        let root_module = self.module_tree(path, DuskPath::Root)?;
        self.namespace
            .insert(DuskPath::Root, Item::Module(root_module));

        Ok(())
    }

    fn module_tree(&mut self, mut path: PathBuf, parent: DuskPath) -> Result<Module> {
        // If we're a directory, we can assume we're at the package root. As such we need to figure out our entry point.
        if path.is_dir() {
            path.push(self.kind.filename());
            return self.module_tree(path, DuskPath::Root);
        }

        // This early return will also catch issues with the file not existing. I'm not sure about whether we should
        // catch that here or look for it earlier. Guess we'll see.

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Could not open {}", path.to_string_lossy()))
            .context("Unresolved module")?;

        let parse_result = Parser::new(&contents).parse()?;

        // Module declarations are items (top-level statements) so we can just filter like so to check if we need to
        // parse anything else. In the future this may change but for now our lives are easy.
        let submodules = parse_result.iter().filter_map(|node| match node {
            AstNode::Module(module) => Some(module),
            _ => None,
        });

        for submodule in submodules {
            let submodule_name = &contents[submodule.name.span.clone()];
            let submodule_path = DuskPath::Scope {
                left: Box::new(parent.clone()),
                right: Box::new(DuskPath::Name(submodule_name.to_string())),
            };

            // TODO: Nicer error. I'd like to somehow point to a code snippet and a path in the future. This is okay
            // for the moment though.
            if self.namespace.contains_key(&submodule_path) {
                bail!("Module {} declared multiple times", submodule_name)
            }

            let current_level = path.with_file_name(&format!("{}.dusk", submodule_name));
            let mut level_down = path.clone();
            level_down.pop();
            level_down.push(format!("{}/module.dusk", submodule_name));

            let submodule = match (current_level.exists(), level_down.exists()) {
                (true, false) => self.module_tree(current_level, parent.clone())?,
                (false, true) => self.module_tree(level_down, submodule_path.clone())?,
                _ => bail!("Unresolved module"),
            };

            self.namespace
                .insert(submodule_path, Item::Module(submodule));
        }

        Ok(Module::new(contents, parse_result))
    }
}
