default: build-release

build-debug *args:
    cargo build --bins {{args}}

build-release *args: (build-debug '--release' args)

install: build-release
    sudo install -Dm755 "./target/release/ideapad_applet" "/usr/bin/ideapad_applet";
    sudo install -Dm755 "./target/release/ideapad_applet_writer" "/usr/bin/ideapad_applet_writer";

    sudo install -Dm644 "./data/10-ideapad-applet.rules" "/etc/polkit-1/rules.d/10-ideapad-applet.rules"
    sudo install -Dm644 "./data/com.ferreroianjose.ideapad-applet.desktop" "/usr/share/applications/com.ferreroianjose.ideapad-applet.desktop"
    sudo install -Dm644 "./data/com.ferreroianjose.ideapad-applet.policy" "/usr/share/polkit-1/actions/com.ferreroianjose.ideapad-applet.policy"
    
    @echo "Installation complete!"

uninstall:
    sudo rm -f "/usr/bin/ideapad_applet";
    sudo rm -f "/usr/bin/ideapad_applet_writer";

    sudo rm -f "/etc/polkit-1/rules.d/10-ideapad-applet.rules"
    sudo rm -f "/usr/share/applications/com.ferreroianjose.ideapad-applet.desktop"
    sudo rm -f "/usr/share/polkit-1/actions/com.ferreroianjose.ideapad-applet.policy"

    @echo "Uninstallation complete!"
