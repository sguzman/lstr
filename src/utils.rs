//! Shared utility functions for the lstr application.

/// Formats a size in bytes into a human-readable string using binary prefixes (KiB, MiB).
pub fn format_size(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    const TIB: f64 = GIB * 1024.0;

    let bytes = bytes as f64;

    if bytes < KIB {
        format!("{} B", bytes)
    } else if bytes < MIB {
        format!("{:.1} KiB", bytes / KIB)
    } else if bytes < GIB {
        format!("{:.1} MiB", bytes / MIB)
    } else if bytes < TIB {
        format!("{:.1} GiB", bytes / GIB)
    } else {
        format!("{:.1} TiB", bytes / TIB)
    }
}

// Unit tests for utility functions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.0 KiB");
        assert_eq!(format_size(1536), "1.5 KiB");
        let mib = 1024 * 1024;
        assert_eq!(format_size(mib), "1.0 MiB");
        assert_eq!(format_size(mib + mib / 2), "1.5 MiB");
        let gib = mib * 1024;
        assert_eq!(format_size(gib), "1.0 GiB");
    }
}
