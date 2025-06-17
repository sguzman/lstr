use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs;
use std::process::Command;
use tempfile::tempdir;

// Platform-specific import for unix permissions
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

#[test]
fn test_nonexistent_path() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("lstr")?;
    cmd.arg("nonexistent/path/for/testing");
    cmd.assert().failure().stderr(predicate::str::contains("is not a directory"));
    Ok(())
}

#[test]
fn test_simple_view() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    fs::File::create(temp_dir.path().join("a.txt"))?;
    fs::create_dir(temp_dir.path().join("dir1"))?;
    fs::File::create(temp_dir.path().join("dir1/b.txt"))?;

    let mut cmd = Command::cargo_bin("lstr")?;
    cmd.arg(temp_dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("a.txt"))
        .stdout(predicate::str::contains("dir1"))
        .stdout(predicate::str::contains("b.txt"));
    Ok(())
}

#[test]
fn test_all_flag() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    fs::File::create(temp_dir.path().join(".hidden"))?;

    let mut cmd_no_all = Command::cargo_bin("lstr")?;
    cmd_no_all.arg(temp_dir.path());
    cmd_no_all.assert().success().stdout(predicate::str::contains(".hidden").not());

    let mut cmd_with_all = Command::cargo_bin("lstr")?;
    cmd_with_all.arg("-a").arg(temp_dir.path());
    cmd_with_all.assert().success().stdout(predicate::str::contains(".hidden"));
    Ok(())
}

#[test]
fn test_depth_flag() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    fs::create_dir(temp_dir.path().join("dir1"))?;
    fs::File::create(temp_dir.path().join("dir1/b.txt"))?;

    let mut cmd = Command::cargo_bin("lstr")?;
    cmd.arg("-L").arg("1").arg(temp_dir.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("dir1"))
        .stdout(predicate::str::contains("b.txt").not());
    Ok(())
}

#[test]
fn test_gitignore_flag() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    // 1. Initialize a true git repository
    Command::new("git").arg("init").current_dir(temp_path).output()?;
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_path)
        .output()?;
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_path)
        .output()?;

    // 2. Create and commit the .gitignore file
    let gitignore_path = temp_path.join(".gitignore");
    fs::write(&gitignore_path, "ignored.txt\nignored_dir/\n")?;
    Command::new("git").arg("add").arg(&gitignore_path).current_dir(temp_path).output()?;
    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("add gitignore")
        .current_dir(temp_path)
        .output()?;

    // 3. Create other files to be checked
    fs::File::create(temp_path.join("ignored.txt"))?;
    fs::File::create(temp_path.join("good.txt"))?;
    fs::create_dir(temp_path.join("ignored_dir"))?;
    fs::File::create(temp_path.join("ignored_dir/a.txt"))?;

    // 4. Run lstr, passing the temp path as an argument. This is more robust
    // than relying on `current_dir` for this specific test.
    let mut cmd = Command::cargo_bin("lstr")?;
    cmd.arg("-g").arg(temp_path);

    // 5. Assert that the correct files are included and excluded.
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("good.txt"))
        .stdout(predicate::str::contains("ignored.txt").not())
        .stdout(predicate::str::contains("ignored_dir").not());

    Ok(())
}

#[test]
#[cfg(unix)]
fn test_permissions_flag() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let file_path = temp_dir.path().join("test_file.txt");
    fs::File::create(&file_path)?;

    let perms = fs::Permissions::from_mode(0o550);
    fs::set_permissions(&file_path, perms)?;

    let mut cmd = Command::cargo_bin("lstr")?;
    cmd.arg("-p").arg(temp_dir.path());
    cmd.assert().success().stdout(predicate::str::contains("-r-xr-x---"));

    Ok(())
}

#[test]
fn test_git_status_flag() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path();

    Command::new("git").arg("init").current_dir(temp_path).output()?;
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(temp_path)
        .output()?;
    Command::new("git")
        .args(["config", "user.name", "Test User"])
        .current_dir(temp_path)
        .output()?;

    fs::write(temp_path.join("committed.txt"), "initial content")?;
    Command::new("git").args(["add", "committed.txt"]).current_dir(temp_path).output()?;
    Command::new("git").args(["commit", "-m", "initial commit"]).current_dir(temp_path).output()?;

    fs::write(temp_path.join("committed.txt"), "modified content")?;
    fs::write(temp_path.join("staged.txt"), "staged")?;
    Command::new("git").args(["add", "staged.txt"]).current_dir(temp_path).output()?;
    fs::write(temp_path.join("untracked.txt"), "untracked")?;

    let mut cmd = Command::cargo_bin("lstr")?;
    cmd.arg("-G").arg("-a").arg(temp_path);

    cmd.assert()
        .success()
        .stdout(predicate::str::is_match(r"M\s+.*committed\.txt").unwrap())
        .stdout(predicate::str::is_match(r"A\s+.*staged\.txt").unwrap())
        .stdout(predicate::str::is_match(r"\?\s+.*untracked\.txt").unwrap());

    Ok(())
}
