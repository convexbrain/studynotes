# nRF52840メモ

## 開発環境（ハードウェア）

### SparkFun Pro nRF52840 Mini
* https://www.switch-science.com/catalog/5508/
* 回路図
  * https://cdn.sparkfun.com/assets/learn_tutorials/8/2/0/nrf52840-breakout-mdbt50q-v10.pdf

### Bumpy
* https://www.switch-science.com/catalog/5557/

## 開発環境（ソフトウェア）

### GNU Arm Embedded Toolchain
* https://developer.arm.com/tools-and-software/open-source-software/developer-tools/gnu-toolchain/gnu-rm/downloads
  * ダウンロードしてインストール
  * 最後、Add path...のチェックボックスはオフのままでよい

### nRF5 SDK
* https://www.nordicsemi.com/Software-and-Tools/Software/nRF5-SDK
  * ダウンロードして解凍
    * ```nRF5_SDK_15.3.0_59ac345/components/```
      * ```toolchain/gcc/Makefile.windows``` をGNU Arm Embedded Toolchainのインストール先に合わせて編集
        ```
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
* BSPをのせかえたもの：```examples/blinky/```
  * （```custom_board.h``` は https://github.com/sparkfun/nRF52840_Breakout_MDBT50Q/blob/master/Firmware/nRF5_SDK/components/boards/sparkfun_nrf52840_mini.h をrenameして使用している）
  * ```spf52840_blank/armgcc/``` で ```make``` → ```DONE nrf52840_xxaa```

### FLASH書き込みテスト

* Bumpyを接続：https://docs.electronut.in/bumpy/#bluey
  * 3.3-VDDの接続はしない（ボードには別途USB-Micro経由で給電する）
* ```examples/blinky/spf52840_blank/armgcc/``` で
  ```
  $ /c/Program\ Files\ \(x86\)/GNU\ Tools\ ARM\ Embedded/8\ 2019-q3-update/bin/arm-none-eabi-gdb
  (gdb) target extended-remote COM3
  (gdb) monitor swdp_scan
  (gdb) attach 1
  (gdb) load _build/nrf52840_xxaa.hex
  (gdb) run
  ```
  * ```COM3``` はBumpyをつないだ時に出るシリアルポート番号の若いほう
  * ポート番号が10以上の場合は ```\\.\COM10``` とする

---

### nRF Command Line Tools
* https://www.nordicsemi.com/Software-and-Tools/Development-Tools/nRF-Command-Line-Tools/Download#infotabs

---

### ToDo
* SoftDeviceのテスト
  * BLEのサンプル
* デバッグ
  * VSCodeでgdb起動
  * Bumpy経由UART出力
* Rust
  * no-std ARM
  * nRF5_SDK FFI
  * panicなどをUARTから出す
  * RTOS
  * BLE通信
