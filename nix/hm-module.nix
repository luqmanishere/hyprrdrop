self: {
  config,
  pkgs,
  lib,
  ...
}: let
  inherit (lib.types) package;
  inherit (lib.modules) mkIf;
  inherit (lib.options) mkOption mkEnableOption;

  cfg = config.programs.hyprrdrop;
in {
  options.programs.hyprrdrop = {
    enable = mkEnableOption "hyprrdrop";
    package = mkOption {
      description = "The hyprrdrop package";
      type = package;
      default = self.packages.${pkgs.stdenv.hostPlatform.system}.hyprrdrop;
    };
  };

  config = mkIf cfg.enable {
    home.packages = [cfg.package];
  };
}
