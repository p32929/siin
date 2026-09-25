# siin

A silent batch installer for Windows, written in Rust 🦀 — give it a JSON list of apps and it downloads and installs them all silently, one after another. Like [Ninite](https://ninite.com/), but the list is yours and the source is open.

On [crates.io](https://crates.io/crates/siin).

![siin](https://github.com/p32929/siin/assets/6418354/c6920f4a-f271-4e34-aeab-9811039a9a0e)

## How to install
You can either download the binary from the release: https://github.com/p32929/siin/releases/latest
or using cargo `cargo install siin`

## How to use
1. First create a JSON file like this: https://api.npoint.io/45bf94782cf51c2ad900
2. Host the JSON file in https://gist.github.com/ or https://www.npoint.io/ or https://pastebin.com/ or anywhere you like
3. Get the URL 
4. Run SIIN CLI as Admin
    > If you have downloaded the EXE file from the release, just run the exe file as administrator
    
    > If you have installed siin using cargo, start CMD ( command prompt ) as administrator then run siin
5. Paste the URL and Hit enter
6. Done!!!

## JSON structure

Host a JSON file anywhere public (a GitHub Gist, [npoint.io](https://www.npoint.io/), Pastebin — [example](https://api.npoint.io/45bf94782cf51c2ad900)) and give siin its URL:

```json
[
    {
        "title":"name of the app",
        "url":"direct_download_link_of_the_app",
        "alt": "Optional, but if you want to pass any argument to the installer. Like VS code has some arguments. More info here: https://silentinstallhq.com/visual-studio-code-silent-install-how-to-guide/ "
    }
]
```

## License

MIT License — Copyright (c) Fayaz Bin Salam. See [LICENSE](LICENSE) for the full text.

## Contributing

Contributions are warmly welcomed and greatly appreciated! Whether it's a bug fix, new feature, or improvement, your input helps make this project better for everyone.

Before submitting a pull request, please:

1. Create an issue describing the feature or bug fix you'd like to work on
2. Wait for discussion and approval to ensure alignment with project goals
3. Fork the repository and create your feature branch
4. Submit your pull request with a clear description of changes

This approach helps avoid duplicate efforts and ensures smooth collaboration. Thank you for considering contributing!

## Share

Sharing this repository with your friends is just one click away from here

[![facebook](https://user-images.githubusercontent.com/6418354/179013321-ac1d1452-0689-493f-9066-940cf2302b6e.png)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/p32929/siin/)
[![twitter](https://user-images.githubusercontent.com/6418354/179013351-7d8d6d1c-4ce2-46ab-bef8-4c4765a1b888.png)](https://twitter.com/intent/tweet?url=https://github.com/p32929/siin/)
[![tumblr](https://user-images.githubusercontent.com/6418354/179013343-3111f55a-3b90-40c7-8487-9777348672b0.png)](https://www.tumblr.com/share?v=3&u=https://github.com/p32929/siin/)
[![pocket](https://user-images.githubusercontent.com/6418354/179013334-b095c45f-becf-49f4-9ee1-5a731a9b1f85.png)](https://getpocket.com/save?url=https://github.com/p32929/siin/)
[![pinterest](https://user-images.githubusercontent.com/6418354/179013331-44cd9206-11b1-4b65-becb-5863b61c828f.png)](https://pinterest.com/pin/create/button/?url=https://github.com/p32929/siin/)
[![reddit](https://user-images.githubusercontent.com/6418354/179013338-7416ae3f-73ba-4522-86e1-1374d7082d22.png)](https://www.reddit.com/submit?url=https://github.com/p32929/siin/)
[![linkedin](https://user-images.githubusercontent.com/6418354/179013327-ca7b7102-1da8-4b1c-858f-1a6e5f21bd70.png)](https://www.linkedin.com/shareArticle?mini=true&url=https://github.com/p32929/siin/)
[![whatsapp](https://user-images.githubusercontent.com/6418354/179013353-f477fa0b-3e6f-4138-a357-c9991b23ff88.png)](https://api.whatsapp.com/send?text=https://github.com/p32929/siin/)

<!-- hire-block -->

---

## 💼 Using this at a company?

I do fixed-price delivery work on my own projects. One invoice, one date, no hourly billing:

| | |
|---|---|
| **White-label build** — this project rebranded, extended and deployed as yours | **$6,500** · 3 weeks |
| **Custom app from scratch** on my own stack, signed and auto-updating | **$12,500** · 6 weeks |
| **Production-hardening sprint** — 72 hours on this project, for your load and your security review | **$999** |
| **Ongoing capacity** — one project-week of my time reserved every month | **$9,000 / month** |

Full details → **[p32929.github.io/hire](https://p32929.github.io/hire/)** · Email **[fayazdevinbox@uberip.com](mailto:fayazdevinbox@uberip.com)** — scoping and quotes are free and I answer within one business day.
