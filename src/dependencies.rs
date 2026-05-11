use std::path::PathBuf;

use miette::Diagnostic;
use which::which;

#[derive(Debug, Clone, thiserror::Error, Diagnostic)]
pub enum DependencyError {
    #[error("`gpg` not found in PATH")]
    #[diagnostic(
        code(deps::gpg),
        help(
            "Install GnuPG:\n\
                 \n\
                 • macOS:    brew install gnupg\n\
                 • Debian:   apt install gnupg\n\
                 • Arch:     pacman -S gnupg\n\
                 • Windows:  scoop install gpg  (or download from https://gnupg.org/download/)\n\
                 \n\
                 After installing, verify with: gpg --version"
        )
    )]
    Gpg,

    #[error("`git` not found in PATH")]
    #[diagnostic(
        code(deps::git),
        help(
            "Install Git:\n\
                 \n\
                 • macOS:    brew install git  (or use Xcode Command Line Tools)\n\
                 • Debian:   apt install git\n\
                 • Arch:     pacman -S git\n\
                 • Windows:  scoop install git  (or download from https://git-scm.com/download/win)\n\
                 \n\
                 After installing, verify with: git --version"
        )
    )]
    Git,

    #[error("`protoc` not found in PATH")]
    #[diagnostic(
        code(deps::protoc),
        help(
            "Install the Protocol Buffers compiler:\n\
                \n\
                • macOS:    brew install protobuf\n\
                • Debian:   apt install protobuf-compiler\n\
                • Arch:     pacman -S protobuf\n\
                • Windows:  scoop install protobuf  (or download from https://github.com/protocolbuffers/protobuf/releases)\n\
                \n\
                After installing, verify with: protoc --version"
        )
    )]
    Protoc,

    #[error("`protoc-gen-go` not found in PATH")]
    #[diagnostic(
        code(deps::protoc_gen_go),
        help(
            "Install the Go protobuf plugin:\n\
                 \n\
                 go install google.golang.org/protobuf/cmd/protoc-gen-go@latest\n\
                 \n\
                 Make sure $(go env GOPATH)/bin is in your PATH."
        )
    )]
    ProtocGenGo,

    #[error("`protoc-gen-go-grpc` not found in PATH")]
    #[diagnostic(
        code(deps::protoc_gen_go_grpc),
        help(
            "Install the Go gRPC plugin:\n\
                 \n\
                 go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest\n\
                 \n\
                 Make sure $(go env GOPATH)/bin is in your PATH."
        )
    )]
    ProtocGenGoGrpc,

    #[error("`go` not found in PATH")]
    #[diagnostic(
        code(deps::go),
        help(
            "Install Go:\n\
                 \n\
                 • macOS:    brew install go\n\
                 • Debian:   apt install golang-go\n\
                 • Arch:     pacman -S go\n\
                 • Windows:  scoop install go  (or download from https://go.dev/dl/)\n\
                 \n\
                 After installing, verify with: go version"
        )
    )]
    Go,

    #[error("`java` not found in PATH")]
    #[diagnostic(
        code(deps::java),
        help(
            "Install a JDK (17 or newer recommended):\n\
                 \n\
                 • macOS:    brew install openjdk\n\
                 • Debian:   apt install default-jdk\n\
                 • Arch:     pacman -S jdk-openjdk\n\
                 • Windows:  scoop install openjdk\n\
                 \n\
                 After installing, verify with: java --version"
        )
    )]
    Java,

    #[error("`mvn` not found in PATH")]
    #[diagnostic(
        code(deps::maven),
        help(
            "Install Apache Maven:\n\
                 \n\
                 • macOS:    brew install maven\n\
                 • Debian:   apt install maven\n\
                 • Arch:     pacman -S maven\n\
                 • Windows:  scoop install maven  (or download from https://maven.apache.org/download.cgi)\n\
                 \n\
                 After installing, verify with: mvn --version"
        )
    )]
    Maven,

    #[error("`cargo` not found in PATH")]
    #[diagnostic(
        code(deps::cargo),
        help(
            "Install the Rust toolchain via rustup:\n\
                 \n\
                 curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n\
                 \n\
                 After installing, restart your shell and verify with: cargo --version"
        )
    )]
    Cargo,

    #[error("`protoc-gen-prost` not found in PATH")]
    #[diagnostic(
        code(deps::protoc_gen_prost),
        help(
            "Install the prost protoc plugin:\n\
                 \n\
                 cargo install protoc-gen-prost\n\
                 \n\
                 Make sure ~/.cargo/bin is in your PATH."
        )
    )]
    ProtocGenProst,

    #[error("`protoc-gen-prost-crate` not found in PATH")]
    #[diagnostic(
        code(deps::protoc_gen_prost_crate),
        help(
            "Install the prost crate generator plugin:\n\
                 \n\
                 cargo install protoc-gen-prost-crate\n\
                 \n\
                 Make sure ~/.cargo/bin is in your PATH."
        )
    )]
    ProtocGenProstCrate,

    #[error("`protoc-gen-tonic` not found in PATH")]
    #[diagnostic(
        code(deps::protoc_gen_tonic),
        help(
            "Install the tonic gRPC plugin:\n\
                 \n\
                 cargo install protoc-gen-tonic\n\
                 \n\
                 Make sure ~/.cargo/bin is in your PATH."
        )
    )]
    ProtocGenTonic,

    #[error("`node` not found in PATH")]
    #[diagnostic(
        code(deps::node),
        help(
            "Install Node.js (LTS recommended):\n\
                 \n\
                 • macOS:    brew install node\n\
                 • Debian:   apt install nodejs npm\n\
                 • Arch:     pacman -S nodejs npm\n\
                 • Windows:  scoop install nodejs-lts\n\
                 • Anywhere: use https://github.com/nvm-sh/nvm\n\
                 \n\
                 After installing, verify with: node --version"
        )
    )]
    Node,

    #[error("`npm` not found in PATH")]
    #[diagnostic(
        code(deps::npm),
        help(
            "npm comes with Node.js. Install Node.js:\n\
                 \n\
                 • macOS:    brew install node\n\
                 • Debian:   apt install nodejs npm\n\
                 • Anywhere: https://nodejs.org/\n\
                 \n\
                 Verify with: npm --version"
        )
    )]
    Npm,

    #[error("`protoc-gen-js` not found in PATH")]
    #[diagnostic(
        code(deps::protoc_gen_js),
        help(
            "Install the JavaScript protobuf plugin:\n\
                 \n\
                 npm install -g protoc-gen-js\n\
                 \n\
                 Or download from https://github.com/protocolbuffers/protobuf-javascript/releases\n\
                 and place the binary in your PATH."
        )
    )]
    ProtocGenJs,

    #[error("`protoc-gen-grpc-web` not found in PATH")]
    #[diagnostic(
        code(deps::protoc_gen_grpc_web),
        help(
            "Install the gRPC-Web plugin:\n\
                 \n\
                 Download from https://github.com/grpc/grpc-web/releases\n\
                 and place `protoc-gen-grpc-web` in your PATH.\n\
                 \n\
                 On macOS: brew install protoc-gen-grpc-web"
        )
    )]
    ProtocGenGrpcWeb,

    #[error("`protoc-gen-ts_proto` not found in PATH")]
    #[diagnostic(
        code(deps::protoc_gen_ts_proto),
        help(
            "Install ts-proto:\n\
                 \n\
                 npm install -g ts-proto\n\
                 \n\
                 Or as a devDependency in your project:\n\
                 npm install --save-dev ts-proto\n\
                 \n\
                 Verify with: which protoc-gen-ts_proto"
        )
    )]
    ProtocGenTsProto,

    #[error("`tsc` not found in PATH")]
    #[diagnostic(
        code(deps::tsc),
        help(
            "Install the TypeScript compiler:\n\
                 \n\
                 npm install -g typescript\n\
                 \n\
                 Or as a devDependency:\n\
                 npm install --save-dev typescript\n\
                 \n\
                 Verify with: tsc --version"
        )
    )]
    Tsc,

    #[error("`python3` not found in PATH")]
    #[diagnostic(
        code(deps::python),
        help(
            "Install Python 3.9 or newer:\n\
                 \n\
                 • macOS:    brew install python\n\
                 • Debian:   apt install python3 python3-pip python3-venv\n\
                 • Arch:     pacman -S python python-pip\n\
                 • Windows:  scoop install python  (or download from https://python.org/)\n\
                 \n\
                 After installing, verify with: python3 --version"
        )
    )]
    Python,

    #[error("`grpc_tools.protoc` not available")]
    #[diagnostic(
        code(deps::grpcio_tools),
        help(
            "Install grpcio-tools (provides protoc with Python and gRPC plugins):\n\
                 \n\
                 pip install grpcio-tools\n\
                 \n\
                 Verify with: python3 -m grpc_tools.protoc --version"
        )
    )]
    GrpcioTools,

    #[error("`twine` not found in PATH")]
    #[diagnostic(
        code(deps::twine),
        help(
            "Install twine (PyPI upload tool):\n\
                 \n\
                 pip install twine\n\
                 \n\
                 Verify with: twine --version"
        )
    )]
    Twine,

    #[error("`build` (PEP 517 builder) not available")]
    #[diagnostic(
        code(deps::build),
        help(
            "Install the build module:\n\
                 \n\
                 pip install build\n\
                 \n\
                 Verify with: python3 -m build --version"
        )
    )]
    Build,
}

pub fn gpg() -> Result<PathBuf, DependencyError> {
    which("gpg").map_err(|_| DependencyError::Gpg)
}

pub fn git() -> Result<PathBuf, DependencyError> {
    which("git").map_err(|_| DependencyError::Git)
}

#[macro_export]
macro_rules! git {
    ($ctx:expr, env: [$(($k:expr, $v:expr)),* $(,)?], $($arg:expr),+ $(,)?) => {{
        let ctx = &$ctx;
        let mut cmd = ::tokio::process::Command::new("git");
        cmd.args([$($arg),+]).current_dir(&ctx.target_path);
        $( cmd.env($k, $v); )*
        ctx.run(&mut cmd).await
    }};

    ($ctx:expr, $($arg:expr),+ $(,)?) => {{
        let ctx = &$ctx;
        let mut cmd = ::tokio::process::Command::new("git");
        cmd.args([$($arg),+]).current_dir(&ctx.target_path);
        ctx.run(&mut cmd).await
    }};
}

pub fn protoc() -> Result<PathBuf, DependencyError> {
    which("protoc").map_err(|_| DependencyError::Protoc)
}

pub fn protoc_gen_go() -> Result<PathBuf, DependencyError> {
    which("protoc-gen-go").map_err(|_| DependencyError::ProtocGenGo)
}

pub fn protoc_gen_go_grpc() -> Result<PathBuf, DependencyError> {
    which("protoc-gen-go-grpc").map_err(|_| DependencyError::ProtocGenGoGrpc)
}

pub fn go() -> Result<PathBuf, DependencyError> {
    which("go").map_err(|_| DependencyError::Go)
}

pub fn java() -> Result<PathBuf, DependencyError> {
    which("java").map_err(|_| DependencyError::Java)
}

pub fn maven() -> Result<PathBuf, DependencyError> {
    which("mvn").map_err(|_| DependencyError::Maven)
}

pub fn cargo() -> Result<PathBuf, DependencyError> {
    which("cargo").map_err(|_| DependencyError::Cargo)
}

pub fn protoc_gen_prost() -> Result<PathBuf, DependencyError> {
    which("protoc-gen-prost").map_err(|_| DependencyError::ProtocGenProst)
}

pub fn protoc_gen_prost_crate() -> Result<PathBuf, DependencyError> {
    which("protoc-gen-prost-crate").map_err(|_| DependencyError::ProtocGenProstCrate)
}

pub fn protoc_gen_tonic() -> Result<PathBuf, DependencyError> {
    which("protoc-gen-tonic").map_err(|_| DependencyError::ProtocGenTonic)
}

pub fn node() -> Result<PathBuf, DependencyError> {
    which("node").map_err(|_| DependencyError::Node)
}

pub fn npm() -> Result<PathBuf, DependencyError> {
    which("npm").map_err(|_| DependencyError::Npm)
}

pub fn protoc_gen_js() -> Result<PathBuf, DependencyError> {
    which("protoc-gen-js").map_err(|_| DependencyError::ProtocGenJs)
}

pub fn protoc_gen_grpc_web() -> Result<PathBuf, DependencyError> {
    which("protoc-gen-grpc-web").map_err(|_| DependencyError::ProtocGenGrpcWeb)
}

pub fn protoc_gen_ts_proto() -> Result<PathBuf, DependencyError> {
    which("protoc-gen-ts_proto").map_err(|_| DependencyError::ProtocGenTsProto)
}

pub fn tsc() -> Result<PathBuf, DependencyError> {
    which("tsc").map_err(|_| DependencyError::Tsc)
}

pub fn python() -> Result<PathBuf, DependencyError> {
    which("python3")
        .or_else(|_| which("python"))
        .map_err(|_| DependencyError::Python)
}

pub fn twine() -> Result<PathBuf, DependencyError> {
    which("twine").map_err(|_| DependencyError::Twine)
}

/// Verifies grpcio-tools is importable (provides the protoc-based code gen).
pub fn grpcio_tools() -> Result<(), DependencyError> {
    let py = python()?;
    let status = std::process::Command::new(py)
        .args(["-c", "import grpc_tools.protoc"])
        .status()
        .map_err(|_| DependencyError::GrpcioTools)?;
    if status.success() {
        Ok(())
    } else {
        Err(DependencyError::GrpcioTools)
    }
}

/// Verifies the `build` module is importable.
pub fn python_build() -> Result<(), DependencyError> {
    let py = python()?;
    let status = std::process::Command::new(py)
        .args(["-c", "import build"])
        .status()
        .map_err(|_| DependencyError::Build)?;
    if status.success() {
        Ok(())
    } else {
        Err(DependencyError::Build)
    }
}
