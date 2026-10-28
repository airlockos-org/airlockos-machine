# User-owned NixOS settings for the firewall VM.
{ pkgs, ... }:
{
  environment.systemPackages = with pkgs; [ ];
}
