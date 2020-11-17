# rust-chibicc
## main

origin(Rui Ueyama) : https://github.com/rui314/chibicc

document: [低レイヤを知りたい人のためのCコンパイラ作成入門](https://www.sigbus.info/compilerbook)

# chibicc: A Small C Compiler
レポジトリ名通り、Rui Ueyama([rui314](https://github.com/rui314))さんによるコード量が少ないCコンパイラの実装。
# 実行
1. ```bash
   git clone https://github.com/smirror/rust-chibicc.git
   ```
2.  - 今、実行可能なテストを全て行う場合
        ```bash
        bash test.sh
        ```

    - 個別にテストを行う場合
        ```bash
        cd rust-chibicc
        cargo run -- "(test cases)"
        ```
# 実装した機能
- [x]  Step 1: Print one number
- [x]  Step 2: Operetor '+' & '-'
- [x]  Step 3: replace space' ' (but not completely)
- [ ]  Step 4: Error messages



# Reference
- [utam0k/r9cc](https://github.com/utam0k/r9cc)
- [itome/nine-cc](https://github.com/itome/nine-cc)
- [AK-10/rust_chibicc](https://github.com/AK-10/rust_chibicc)