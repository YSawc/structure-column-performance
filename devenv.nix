{ pkgs, inputs, ... }:

{
  packages = [
    inputs.fenix.packages.${pkgs.system}.complete.toolchain
    pkgs.wrk
    pkgs.libiconv
  ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
    pkgs.darwin.apple_sdk.frameworks.Security
    pkgs.darwin.apple_sdk.frameworks.CoreFoundation
    pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  services.mysql = {
    enable = true;
    package = pkgs.mariadb;
    initialDatabases = [
      { 
        name = "structure_comparison";
        schema = ./migrations/001_init.sql;
      }
    ];
    ensureUsers = [
      {
        name = "dev";
        password = "dev";
        ensurePermissions = {
          "structure_comparison.*" = "ALL PRIVILEGES";
        };
      }
    ];
  };

  enterShell = ''
    echo "🦀 Rust development environment loaded"
    echo "📊 Structure column performance comparison project"
    echo "🐬 MariaDB available at localhost:3306"
    echo "📦 Database: structure_comparison"
    echo "👤 User: dev / Password: dev"
  '';
}