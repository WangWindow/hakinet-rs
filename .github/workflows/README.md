# GitHub Workflows Documentation

This directory contains GitHub Actions workflows for the Hakinet project.

## Workflows Overview

### 1. `ci-cd.yml` - Continuous Integration and Deployment
**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` branch
- Release publications

**Features:**
- Runs tests and linting
- Builds for both Linux and Windows
- Uploads build artifacts
- Automatically attaches binaries to releases

### 2. `build-linux.yml` - Linux Build Pipeline
**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` branch
- Release publications

**Features:**
- Installs libpcap development libraries
- Builds optimized Linux binaries
- Strips debug symbols for smaller size
- Creates release archives

### 3. `build-windows.yml` - Windows Build Pipeline
**Triggers:**
- Push to `main` or `develop` branches
- Pull requests to `main` branch
- Release publications

**Features:**
- Downloads and configures WinPcap development pack
- Builds Windows binaries with MSVC
- Creates ZIP archives for distribution

### 4. `release.yml` - Automated Release Pipeline
**Triggers:**
- Push of version tags (e.g., `v1.0.0`)

**Features:**
- Creates GitHub releases automatically
- Builds and attaches platform-specific binaries
- Includes installation instructions
- Generates release notes with cute cat ASCII art

## Usage

### Creating a Release

1. **Tag your commit:**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. **The release workflow will automatically:**
   - Create a GitHub release
   - Build binaries for Linux and Windows
   - Attach the binaries to the release
   - Generate release notes

### Manual Workflow Triggers

You can also trigger workflows manually from the GitHub Actions tab:

1. Go to your repository on GitHub
2. Click on "Actions" tab
3. Select the workflow you want to run
4. Click "Run workflow"

## Artifacts

### Linux Builds
- **File:** `hakinet-linux-x86_64.tar.gz`
- **Contains:**
  - `hakinet` (packet capture tool)
  - `hakinet-scan` (network scanner)
  - `README.md`
  - `install.sh` (installation script)

### Windows Builds
- **File:** `hakinet-windows-x86_64.zip`
- **Contains:**
  - `hakinet.exe`
  - `hakinet-scan.exe`
  - `README.md`
  - `INSTALL.txt` (installation instructions)

## Dependencies

### Linux
- `libpcap-dev` - Packet capture library
- `pkg-config` - Package configuration tool

### Windows
- WinPcap Development Pack - Downloaded automatically during build

## Environment Variables

The workflows set the following environment variables:

### Windows-specific
- `LIB` - Library search paths
- `LIBPCAP_LIBDIR` - libpcap library directory
- `LIBPCAP_INCLUDEDIR` - libpcap header directory

### Universal
- `CARGO_TERM_COLOR=always` - Colorized cargo output

## Caching

All workflows use cargo caching to speed up builds:
- Registry cache
- Git dependencies cache
- Build artifacts cache

Cache keys are based on:
- Operating system
- Cargo.lock hash
- Workflow type (test/build/release)

## Troubleshooting

### Common Issues

1. **WinPcap download fails on Windows:**
   - The workflow will retry automatically
   - Check if the WinPcap URL is still valid

2. **Linux libpcap installation fails:**
   - Package manager might be temporarily unavailable
   - Re-run the workflow

3. **Tests fail:**
   - Check the test output in the workflow logs
   - Ensure all dependencies are properly installed

4. **Release creation fails:**
   - Verify the tag format matches `v*` pattern
   - Check GitHub token permissions

### Debug Tips

1. **Enable debug logging:**
   Add this to your workflow file:
   ```yaml
   env:
     ACTIONS_STEP_DEBUG: true
   ```

2. **Check artifact contents:**
   Download artifacts from the Actions tab to verify they contain expected files

3. **Local testing:**
   You can test the build process locally:
   ```bash
   # Linux
   cargo build --release --target x86_64-unknown-linux-gnu

   # Windows (with cross-compilation setup)
   cargo build --release --target x86_64-pc-windows-msvc
   ```

## Security Considerations

- Workflows run in isolated GitHub-hosted runners
- No secrets are exposed in logs
- Built binaries are signed by GitHub's infrastructure
- All dependencies are downloaded from official sources

## Customization

### Adding New Targets

To add support for new platforms, update the matrix in the workflows:

```yaml
matrix:
  include:
    - target: x86_64-unknown-linux-gnu
      os: ubuntu-latest
    - target: x86_64-pc-windows-msvc
      os: windows-latest
    - target: x86_64-apple-darwin     # Add macOS
      os: macos-latest
```

### Modifying Release Notes

Edit the release template in `release.yml`:

```yaml
body: |
  ## Your custom release notes here
  ...
```

---

```
   /\_/\
  ( ^.^ ) "Happy building! These workflows will help automate your releases, meow!"
   > ^ <
```
