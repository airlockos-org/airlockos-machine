# User-owned NixOS settings for the build controller.
{ pkgs, ... }:
{
  environment.systemPackages = with pkgs; [ ];
}
