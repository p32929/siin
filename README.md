# siin

A silent batch installer for Windows, written in Rust. Give it a list of apps and it downloads them all, then installs them silently one after another, while you do something else.

Setting up a new or freshly reset PC usually means downloading installers one by one and clicking Next again and again. [Ninite](https://ninite.com/) helps, but only with the apps on its own list. With siin the list is yours: any app with a direct download link, with your own installer options, kept online so you can use it on every PC.

![siin](https://github.com/p32929/siin/assets/6418354/c6920f4a-f271-4e34-aeab-9811039a9a0e)

## Features

1. **Your own app list:** any app with a direct download link, in a simple JSON file.
2. **Silent installs,** one after another, with no clicking through installers.
3. **Custom installer options:** pass extra arguments to an installer when it needs them.
4. **Keep your list online** (a GitHub Gist, npoint.io, Pastebin or anywhere public) and reuse it on every new PC.
5. **Downloads first, then installs,** with a progress bar while it downloads.

## Install

1. Download `siin.exe` from the [latest release](https://github.com/p32929/siin/releases/latest).
2. Or, if you have Rust: `cargo install siin` (also on [crates.io](https://crates.io/crates/siin)).

## How to use

1. Make a JSON file with your apps (see the format below, or this [example](https://api.npoint.io/45bf94782cf51c2ad900)).
2. Host it anywhere public, for example a [GitHub Gist](https://gist.github.com/), [npoint.io](https://www.npoint.io/) or [Pastebin](https://pastebin.com/), and copy its link.
3. Run siin as administrator. If you downloaded the `.exe`, right-click it and choose **Run as administrator**. If you installed it with cargo, open Command Prompt as administrator and type `siin`.
4. Paste the link and press Enter. siin shows the list, downloads everything, then installs each app.

## JSON format

```json
[
    {
        "title": "name of the app",
        "url": "direct_download_link_of_the_app",
        "alt": "Optional: extra arguments for the installer"
    }
]
```

1. `title`: the app's name, shown in the list.
2. `url`: a direct download link to the installer.
3. `alt`: optional extra arguments for the installer, for example the silent-install options some apps need. [silentinstallhq.com](https://silentinstallhq.com/) lists them for many apps.

## Good to know

siin runs the installers you point it to, so only use download links you trust. It is made for Windows.

## License

MIT License — Copyright (c) Fayaz Bin Salam. See [LICENSE](LICENSE) for the full text.

## Contributing

Contributions are very welcome, thank you! Before you start on a pull request, please open an issue and tell me what you would like to add or change. That way we can agree on it first, and your time is not wasted on something that might not fit. Thanks for helping make siin better.

## You might also like

<p><a href="https://openstore.pfay.workers.dev/more/p32929/siin/0"><picture><source media="(prefers-color-scheme: dark)" srcset="https://openstore.pfay.workers.dev/more/p32929/siin/0-dark.svg"><img src="https://openstore.pfay.workers.dev/more/p32929/siin/0-light.svg" width="50%" align="top" alt="More apps by p32929 on OpenStore (1)"></picture></a><a href="https://openstore.pfay.workers.dev/more/p32929/siin/1"><picture><source media="(prefers-color-scheme: dark)" srcset="https://openstore.pfay.workers.dev/more/p32929/siin/1-dark.svg"><img src="https://openstore.pfay.workers.dev/more/p32929/siin/1-light.svg" width="50%" align="top" alt="More apps by p32929 on OpenStore (2)"></picture></a><br><a href="https://openstore.pfay.workers.dev/more/p32929/siin/2"><picture><source media="(prefers-color-scheme: dark)" srcset="https://openstore.pfay.workers.dev/more/p32929/siin/2-dark.svg"><img src="https://openstore.pfay.workers.dev/more/p32929/siin/2-light.svg" width="50%" align="top" alt="More apps by p32929 on OpenStore (3)"></picture></a><a href="https://openstore.pfay.workers.dev/more/p32929/siin/3"><picture><source media="(prefers-color-scheme: dark)" srcset="https://openstore.pfay.workers.dev/more/p32929/siin/3-dark.svg"><img src="https://openstore.pfay.workers.dev/more/p32929/siin/3-light.svg" width="50%" align="top" alt="More apps by p32929 on OpenStore (4)"></picture></a></p>

<p><a href="https://p32929.github.io/share/?repo=siin"><picture><source media="(prefers-color-scheme: dark)" srcset="https://p32929.github.io/readme/footer-share-dark.svg"><img src="https://p32929.github.io/readme/footer-share-light.svg" width="50%" align="top" alt="Share this project with your friends in one click"></picture></a><a href="https://p32929.github.io/support/"><picture><source media="(prefers-color-scheme: dark)" srcset="https://p32929.github.io/readme/footer-support-dark.svg"><img src="https://p32929.github.io/readme/footer-support-light.svg" width="50%" align="top" alt="If you like my work and want to support me, feel free to support"></picture></a><br><a href="https://openstore.pfay.workers.dev/add"><picture><source media="(prefers-color-scheme: dark)" srcset="https://p32929.github.io/readme/footer-join-dark.svg"><img src="https://p32929.github.io/readme/footer-join-light.svg" width="100%" align="top" alt="Put your own apps on OpenStore: free, your listing lives in your own repo"></picture></a></p>
