use alpm::{DepMod, Depend};

/// The response from resolving dependencies.
///
/// Note that just because resolving retuend Ok() does not mean it is safe to bindly start
/// installing these packages.
#[derive(Debug, Default)]
pub struct Actions<'a> {
    /// There were duplicate packages in install/build. This means that two different packages with
    /// the same name want to be installed. As this can not be done it should be treated as a hard
    /// error.
    pub duplicates: Vec<String>,
    /// Some of the targets or dependencies could not be satisfied. This should be treated as
    /// a hard error.
    pub missing: Vec<Vec<String>>,
    /// Targets that are up to date.
    pub unneeded: Vec<String>,
    /// Aur packages to build.
    pub build: Vec<Base>,
    /// Repo packages to install.
    pub install: Vec<RepoPackage<'a>>,
    /// Conflicts. Do note that even with conflicts it can still be possible to continue and
    /// install the packages. Although that is not checked here.
    ///
    /// For example installing pacman-git will conflict with pacman. But the install will still
    /// succeed as long as the user hits yes to pacman's prompt to remove pacman.
    ///
    /// However other cases are more complex and can not be automatically resolved. So it is up to
    /// the user to decide how to handle these.
    pub conflicts: Vec<Conflict>,
    /// Inner conflict. The same rules that apply to conflicts apply here.
    pub inner_conflicts: Vec<Conflict>,
}

/// Wrapper around a package for extra metadata.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Package<T> {
    /// The underlying package
    pub pkg: T,
    /// If the package is only needed to build the targets.
    pub make: bool,
    /// If the package is a target.
    pub target: bool,
}

/// Wrapper around raur_ext::Package for extra metadata.
pub type AurPackage = Package<raur_ext::Package>;

/// Wrapper around alpm::Package for extra metadata.
pub type RepoPackage<'a> = Package<alpm::Package<'a>>;

/// A package base.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Base {
    /// The name of the pkgbase.
    pub pkgbase: String,
    /// The packages that should be installed from the pkgbase.
    pub pkgs: Vec<AurPackage>,
}

/// A conflict
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Conflict {
    /// The name of the package.
    pub pkg: String,
    /// The packages conflicting with it.
    pub conflicting: Vec<Conflicting>,
}

/// A package that has conflicted with something
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Conflicting {
    /// The name of the package.
    pub pkg: String,
    /// The conflict that cause the confliction if it is different from the pkgname.
    pub conflict: Option<String>,
}

impl Conflict {
    /// Crate a new conflict.
    pub fn new(pkg: String) -> Self {
        Conflict {
            pkg,
            conflicting: Vec::with_capacity(1),
        }
    }

    /// Push a new conflicting to the conflict.
    pub fn push(&mut self, pkg: String, conflict: &Depend) {
        let conflict = if pkg != conflict.name() || conflict.depmod() != DepMod::Any {
            Some(conflict.to_string())
        } else {
            None
        };

        self.conflicting.push(Conflicting { pkg, conflict });
    }
}

/// An AUR package that should be updated.
#[derive(Debug)]
pub struct AurUpdate<'a> {
    /// The local package.
    pub local: alpm::Package<'a>,
    /// The AUR package.
    pub remote: raur_ext::Package,
}

/// Collection of AUR updates and missing packages
pub struct AurUpdates<'a> {
    /// The updates.
    pub updates: Vec<AurUpdate<'a>>,
    /// Foreign that were not found in the AUR.
    pub missing: Vec<alpm::Package<'a>>,
}
