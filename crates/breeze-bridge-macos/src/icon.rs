use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

const SIPS: &str = "/usr/bin/sips";
const ICON_PIXELS: &str = "128";
const PNG_MAGIC: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const MAX_PNG_BYTES: u64 = 512 * 1024;
const SIPS_TIMEOUT: Duration = Duration::from_secs(3);
const POLL_STEP: Duration = Duration::from_millis(10);

// Convertit un .icns en PNG 128px via l'outil système, sans shell ni sortie réseau.
// Rend None sur le moindre doute (échec, dépassement, PNG douteux) : l'UI retombe
// alors sur la lettre pour cette app seulement.
pub fn render_icns_to_png(icns: &Path) -> Option<Vec<u8>> {
    let workdir = tempfile::tempdir().ok()?;
    let out = workdir.path().join("icon.png");
    let mut child = spawn_sips(icns, &out)?;
    if !finished_in_time(&mut child) {
        let _ = child.kill();
        let _ = child.wait();
        return None;
    }
    let bytes = std::fs::read(&out).ok()?;
    is_sound_png(&bytes).then_some(bytes)
}

fn spawn_sips(icns: &Path, out: &Path) -> Option<std::process::Child> {
    // Chemins passés en ARGUMENTS d'un binaire absolu : aucune interprétation par un shell.
    Command::new(SIPS)
        .args(["-s", "format", "png", "-z", ICON_PIXELS, ICON_PIXELS])
        .arg(icns)
        .arg("--out")
        .arg(out)
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .ok()
}

fn finished_in_time(child: &mut std::process::Child) -> bool {
    let deadline = Instant::now() + SIPS_TIMEOUT;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return status.success(),
            Ok(None) => {
                if Instant::now() >= deadline {
                    return false;
                }
                std::thread::sleep(POLL_STEP);
            }
            Err(_) => return false,
        }
    }
}

fn is_sound_png(bytes: &[u8]) -> bool {
    let len = bytes.len() as u64;
    len > PNG_MAGIC.len() as u64 && len <= MAX_PNG_BYTES && bytes.starts_with(&PNG_MAGIC)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_truncated_buffer_is_not_a_sound_png() {
        assert!(!is_sound_png(&[0x89, 0x50]));
    }

    #[test]
    fn a_buffer_without_the_png_magic_is_refused() {
        assert!(!is_sound_png(&[0x00, 0x01, 0x02, 0x03, 0x04]));
    }

    #[test]
    fn a_short_buffer_with_the_full_magic_is_accepted() {
        assert!(is_sound_png(&[
            0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00
        ]));
    }

    #[test]
    fn the_first_four_bytes_alone_are_not_enough() {
        assert!(!is_sound_png(&[0x89, 0x50, 0x4E, 0x47]));
    }
}
