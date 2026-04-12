use anyhow::Result;

pub fn init() -> Result<()> {
    let team_dir = std::env::current_dir()?.join(".xbreed").join("team");
    std::fs::create_dir_all(&team_dir)?;
    println!("initialized team dir at {}", team_dir.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_creates_team_dir() {
        use tempfile::tempdir;
        let tmp = tempdir().unwrap();
        let orig = std::env::current_dir().unwrap();
        std::env::set_current_dir(tmp.path()).unwrap();
        let result = init();
        std::env::set_current_dir(orig).unwrap();
        assert!(result.is_ok());
        assert!(tmp.path().join(".xbreed").join("team").exists());
    }
}
