use crate::icon::render_icns_to_png;
use crate::mac_safety_list::SAFETY_BUNDLE_PATHS;
use breeze_domain::AppId;
use breeze_ports::{AppEnumerationError, InstalledApp, InstalledAppsPort};
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const FIXED_ROOTS: [&str; 4] = [
    "/Applications",
    "/Applications/Utilities",
    "/System/Applications",
    "/System/Applications/Utilities",
];

struct Cache {
    signature: u64,
    apps: Vec<InstalledApp>,
}

pub struct MacInstalledApps {
    cache: Mutex<Option<Cache>>,
}

impl Default for MacInstalledApps {
    fn default() -> Self {
        MacInstalledApps {
            cache: Mutex::new(None),
        }
    }
}

impl MacInstalledApps {
    pub fn new() -> Self {
        MacInstalledApps::default()
    }
}

impl InstalledAppsPort for MacInstalledApps {
    fn installed_apps(&self) -> Result<Vec<InstalledApp>, AppEnumerationError> {
        let signature = roots_signature();
        if let Some(cached) = self.cached(signature) {
            return Ok(cached);
        }
        let apps = scan_all();
        let mut guard = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        *guard = Some(Cache {
            signature,
            apps: apps.clone(),
        });
        Ok(apps)
    }
}

impl MacInstalledApps {
    fn cached(&self, signature: u64) -> Option<Vec<InstalledApp>> {
        let guard = self.cache.lock().unwrap_or_else(|e| e.into_inner());
        guard
            .as_ref()
            .filter(|cache| cache.signature == signature)
            .map(|cache| cache.apps.clone())
    }
}

fn roots() -> Vec<PathBuf> {
    let mut roots: Vec<PathBuf> = FIXED_ROOTS.iter().map(PathBuf::from).collect();
    if let Some(home) = std::env::var_os("HOME") {
        roots.push(PathBuf::from(home).join("Applications"));
    }
    roots
}

// Signature = empreinte des dates de modification des racines existantes. Un
// changement (app installée/retirée) invalide le cache ; rien n'est écrit sur disque.
fn roots_signature() -> u64 {
    let mut hasher = DefaultHasher::new();
    for root in roots() {
        if let Ok(modified) = std::fs::metadata(&root).and_then(|meta| meta.modified()) {
            root.hash(&mut hasher);
            modified.hash(&mut hasher);
        }
    }
    hasher.finish()
}

fn scan_all() -> Vec<InstalledApp> {
    let mut found: BTreeMap<AppId, InstalledApp> = BTreeMap::new();
    for root in roots() {
        collect_root(&root, &mut found);
    }
    // Les applications de la liste de sécurité rangées hors des dossiers parcourus
    // (Finder, Trousseau d'accès…) figurent aussi au catalogue, verrouillées.
    for bundle in SAFETY_BUNDLE_PATHS {
        if let Some(app) = read_bundle(Path::new(bundle)) {
            found.entry(app.id.clone()).or_insert(app);
        }
    }
    let mut apps: Vec<InstalledApp> = found.into_values().collect();
    apps.sort_by_key(|app| app.name.to_lowercase());
    apps
}

fn collect_root(root: &Path, found: &mut BTreeMap<AppId, InstalledApp>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|ext| ext == "app") {
            if let Some(app) = read_bundle(&path) {
                found.entry(app.id.clone()).or_insert(app);
            }
        }
    }
}

fn read_bundle(app_dir: &Path) -> Option<InstalledApp> {
    let info = app_dir.join("Contents/Info.plist");
    let dict = plist::Value::from_file(&info).ok()?;
    let dict = dict.as_dictionary()?;
    let raw_id = dict.get("CFBundleIdentifier")?.as_string()?;
    let id = AppId::parse(raw_id).ok()?;
    let name = display_name(dict, app_dir);
    let icon_png = resolve_icns(app_dir, dict).and_then(|icns| render_icns_to_png(&icns));
    Some(InstalledApp { id, name, icon_png })
}

fn display_name(dict: &plist::Dictionary, app_dir: &Path) -> String {
    dict.get("CFBundleDisplayName")
        .and_then(plist::Value::as_string)
        .or_else(|| dict.get("CFBundleName").and_then(plist::Value::as_string))
        .map(str::to_owned)
        .unwrap_or_else(|| bundle_stem(app_dir))
}

fn bundle_stem(app_dir: &Path) -> String {
    app_dir
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("Application")
        .to_owned()
}

// Résout CFBundleIconFile en un .icns réel, en refusant tout chemin qui s'échapperait
// de Contents/Resources (neutralise un « ../ » hostile dans le plist).
fn resolve_icns(app_dir: &Path, dict: &plist::Dictionary) -> Option<PathBuf> {
    let raw = dict.get("CFBundleIconFile")?.as_string()?;
    let resources = app_dir.join("Contents/Resources").canonicalize().ok()?;
    for candidate in [resources.join(raw), resources.join(format!("{raw}.icns"))] {
        if let Ok(real) = candidate.canonicalize() {
            if real.starts_with(&resources) && real.is_file() {
                return Some(real);
            }
        }
    }
    None
}
