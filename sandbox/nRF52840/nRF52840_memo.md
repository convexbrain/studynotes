# nRF52840メモ

---

## 開発環境（ハードウェア）

### SparkFun Pro nRF52840 Mini
* https://www.switch-science.com/catalog/5508/
* 回路図
  * https://cdn.sparkfun.com/assets/learn_tutorials/8/2/0/nrf52840-breakout-mdbt50q-v10.pdf

### Bumpy
* https://www.switch-science.com/catalog/5557/

---

## 開発環境（ソフトウェア on Windows10）

### GNU Arm Embedded Toolchain
* https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads
  * ダウンロードしてインストール
  * 最後、Add path...のチェックボックスはオフのままでよい

### nRF5 SDK
* https://www.nordicsemi.com/Software-and-Tools/Software/nRF5-SDK
  * ダウンロード ```nRF5_SDK_15.3.0_59ac345.zip```

### Bluetooth LE Explorer
* https://www.microsoft.com/ja-jp/p/bluetooth-le-explorer/9n0ztkf1qd98
  * インストール

### nRF Command Line Tools
* https://www.nordicsemi.com/Software-and-Tools/Development-Tools/nRF-Command-Line-Tools/Download#infotabs
  * （未使用）

### nRF Connect for Desktop
* https://www.nordicsemi.com/Software-and-Tools/Development-Tools/nRF-Connect-for-desktop
  * （未使用）

---

## ビルド～デバッグ

### nRF5 SDKの準備
* ```nRF5_SDK_15.3.0_59ac345.zip``` を解凍
  * ```nRF5_SDK_15.3.0_59ac345/```
    * ```components/toolchain/gcc/Makefile.windows``` をGNU Arm Embedded Toolchainのインストール先に合わせて編集
      ```makefile
      GNU_INSTALL_ROOT := C:/Program Files (x86)/GNU Tools ARM Embedded/8 2019-q3-update/bin/
      GNU_VERSION := 8.3.1
      #GNU_INSTALL_ROOT := C:/Program Files (x86)/GNU Tools ARM Embedded/7 2018-q2-update/bin/
      #GNU_VERSION := 7.3.1
      GNU_PREFIX := arm-none-eabi
      ```

### サンプルのビルドテスト
* ```nRF5_SDK_15.3.0_59ac345/examples/peripheral/blinky/pca10056/blank/armgcc/```
  * MSYS/MSYS2/Cygwinなどで ```make``` を実行
  * 正常終了して ```DONE nrf52840_xxaa``` と出る
* BSPをのせかえたもの： ```examples/blinky/```
  * （```custom_board.h``` は https://github.com/sparkfun/nRF52840_Breakout_MDBT50Q/blob/master/Firmware/nRF5_SDK/components/boards/sparkfun_nrf52840_mini.h をrenameして使用している）
  * ```make/``` で ```make``` 実行 → ```DONE nrf52840_xxaa```

### 書き込みと実行

* Bumpyを接続：https://docs.electronut.in/bumpy/#bluey
  * 3.3-VDDの接続はしない（ボードには別途USB-Micro経由で給電する）
* ```examples/blinky/spf52840_blank/armgcc/``` で
  ```sh
  $ /c/Program\ Files\ \(x86\)/GNU\ Tools\ ARM\ Embedded/8\ 2019-q3-update/bin/arm-none-eabi-gdb
  (gdb) target extended-remote COM3
  (gdb) monitor swdp_scan
  (gdb) attach 1
  (gdb) load _build/nrf52840_xxaa.hex
  (gdb) run
  ```
  * ```COM3``` はBumpyをつないだ時に出るシリアルポート番号の若いほう
  * ポート番号が10以上の場合は ```\\.\COM10``` とする

### SoftDevice BLEのサンプル
* ```nRF5_SDK_15.3.0_59ac345/examples/ble_peripheral/ble_app_uart/```
  * BSPのせかえ： ```examples/ble_app_uart/```
    * ```make/``` で ```make``` 実行 → ```DONE nrf52840_xxaa```
* BumpyのUARTを接続 https://docs.electronut.in/bumpy/#using-the-uart-via-usb-on-bumpy
  * Rx-P17
  * Tx-P15
  * COM4（Bumpyをつないだ時に出るシリアルポート番号の大きいほう）をTeraTermなどで開く

### VSCodeでのデバッグ
* Cortex-Debugをインストール https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug
* ```examples/ble_app_uart/``` のフォルダを開く
  * ```.vscode/launch.json``` を編集、```BMPGDBSerialPort``` と ```armToolchainPath``` は環境に合わせる
    ```json
    {
        "version": "0.2.0",
        "configurations": [
            {
                "type": "cortex-debug",
                "request": "launch",
                "name": "Debug (Bumpy)",

                "servertype": "bmp",
                "cwd": "${workspaceRoot}",
                "device": "nRF52840_xxAA",
                "executable": "./make/_build/nrf52840_xxaa.out",
                "preLaunchCommands": [
                    "load ../../nRF5_SDK_15.3.0_59ac345/components/softdevice/s140/hex/s140_nrf52_6.1.1_softdevice.hex"
                ],

                "BMPGDBSerialPort": "COM3",
                "interface": "swd",
                "targetId": 1,

                "svdFile": "../../nRF5_SDK_15.3.0_59ac345/modules/nrfx/mdk/nrf52840.svd",

                "runToMain": true,
                "armToolchainPath": "C:/Program Files (x86)/GNU Tools ARM Embedded/8 2019-q3-update/bin"
            }
        ]
    }
    ```
  * F5などでデバッグ開始
    * ```preLaunchCommands``` はFLASH Eraseなどしない限り二度目以降は不要
    * COM4ターミナルに ```UART started.``` と表示される

### Bluetooth LE Explorerによるテスト
* ```Start```を押してスキャン
* ```Nordic_UART``` のデバイスを選択して接続
  * ```6E400002-B5A3-F393-E0A9-E50E24DCCA9E``` (RX Characteristic) を選択
    * ```UTF8``` をチェック
    * ```Write Value``` に文字を入力し ```Write``` を押す
    * COM4ターミナルに、入力した文字が表示される
  * ```6E400003-B5A3-F393-E0A9-E50E24DCCA9E``` (TX Characteristic) を選択
    * ```UTF8``` をチェック、 ```Noitfy``` をオン
    * COM4ターミナルに文字を入力し、最後にEnterで```\n```を入力
    * ```Read Value``` に、入力した文字が表示される

---

## Rust

### 準備

* nightly版の使用
  * ```$ rustup default nightly```
* ターゲット Cortex-M4 w/ FPU の追加
  * ```$ rustup target add thumbv7em-none-eabihf```
* [cargo-binutils](https://crates.io/crates/cargo-binutils) をインストールしておくと便利
  * ```$ cargo install cargo-binutils```
  * ```$ rustup component add llvm-tools-preview```

### サンプル
* ```rust/led_switch/```
  * ```.cargo/config```
    * ```arm-none-eabi-ld``` のパスは環境に合わせる
    * （```link.x``` は ```cortex-m-rt``` クレート内にあり、これを経由して ```memory.x``` が読み込まれる）
  * ```.vscode/launch.json```
    * ```BMPGDBSerialPort``` と ```armToolchainPath``` は環境に合わせる
  * ビルドとテスト
    * ```cargo build```
    * VSCode F5などでデバッグ開始

### リファレンス
* [Rust Embedded](https://github.com/rust-embedded/wg)
  * [cortex-m クレート](https://crates.io/crates/cortex-m)
  * [cortex-m-rt クレート](https://crates.io/crates/cortex-m-rt)
  * [svd2rust クレート](https://crates.io/crates/svd2rust)
  * [The Embedded Rust Book](https://docs.rust-embedded.org/book/)
* [nrf-rs](https://github.com/nrf-rs)
  * [nrf52840-hal クレート](https://crates.io/crates/nrf52840-hal)
  * [nrf52840-pac クレート](https://crates.io/crates/nrf52840-pac)
