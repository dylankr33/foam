{
  sprinkles ? (import ./npins).sprinkles,
  ...
}@overrides:
(import sprinkles).new {
  inherit overrides;
  sources = import ./npins;
  inputs =
    { sources, inputs }:
    {
      pkgs = import sources.nixpkgs {
        system = "x86_64-linux";
        overlays = [ (import sources.rust-overlay) ];
      };

      crane =
        let
          crane' = import sources.crane;
        in
        (if (builtins.isFunction crane') then crane' { inherit (inputs) pkgs; } else crane')
        .overrideToolchain
          (
            p:
            p.rust-bin.selectLatestNightlyWith (
              toolchain: toolchain.default.override { extensions = [ "rust-src" ]; }
            )
          );
    };
  outputs =
    { crane, pkgs }:
    let
      buildInputs = with pkgs; [
        libX11
        libXcursor
        libXrandr
        libXi
        libxcb
        libxkbcommon
        vulkan-loader
        wayland
        libglvnd
        libGL
      ];
      shellHook = ''
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath buildInputs)}
        export VK_LAYER_PATH="${pkgs.vulkan-validation-layers}/share/vulkan/explicit_layer.d

      '';
    in
    {
      packages.x86_64-linux.default = crane.buildPackage {
        inherit buildInputs shellHook;

        src = ./.;
        nativeBuildInputs = with pkgs; [ makeWrapper ];
      };
      devShells.x86_64-linux.default = crane.devShell {
        inherit buildInputs shellHook;

        packages = with pkgs; [
          rust-analyzer
          cargo-psp
          ppsspp
        ];
      };
    };
}
