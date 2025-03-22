<h1 align="center"> TG-SOURCES: RP2P 📺️ </h1>

<div align = "center">
<a href="https://t.me/ArcaneDevStudio" target="_blank" rel="noopener noreferrer">
    <img src="https://img.shields.io/badge/Telegram-@ArcaneDevStudio-blue?style=flat-square&logo=telegram" alt="Telegram">
</a>
<a href="https://github.com/Nam4ik/TG-SOURCES-RP2P/actions", target="_blank", rel="noopener noreferrer">
    <img src="https://github.com/Nam4ik/TG-SOURCES-RP2P/actions/workflows/rust.yml/badge.svg?event=push", alt="rust.yml">
</a> 
<img src="https://img.shields.io/badge/Version-v0.5-blue.svg">
</div>
<br>

<div align = "center">
  
**Porject created from my telegram channel.** This "network" was created to show the basic principles of network and 
p2p networking, change the branch in order to change the pseudo-p2p to a more advanced version. The pre-release can have
bugs and unstable functions.
</div>

<h2 align = "center"> Using 🖥️ </h2> 

<h3 align = "center"> Installation 🏗️ </h3>

To install the project run this
```shell
git clone https://github.com/TG-SOURCES-RP2P.git ~/rp2p
cd rp2p
cargo build --release 
cd target/release/
./rp2p
```

If after run output similar to this one all secesfully: `Сервер запущен на порту 8080`

To send message for the client you can use `telnet` or `netcat`.
```shell
ip a
nc 0.0.0.0 8080 #Edit to your IP
```
Works only in wi-fi net. (Before compile edit var listener ip from `127.0.0.1:8080` to `0.0.0.0:8080`) 

<h3 align = "center" >Next update 🧠</h3>

**0.6 ver updates:**
- Add the second network, not just client, p2p.
- Add work with global network
- Args and settings
- Remote shell

**[How it works? 🇷🇺](https://t.me/ArcaneDevStudio/116)**

<h3 align = "center">Bug report`s ⛔️ </h3>


