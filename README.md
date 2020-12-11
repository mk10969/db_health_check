# Rust Pinger

-   prepare

    -   これ入れないと diesel_cliが build できない。

    ```
    brew install mysql
    brew install postgresql
    ```

    -   cargo-make 有効化

    ```
    cargo install --force cargo-make
    ```

    Makefile.toml でスクリプトを記述でき、それを実行することができる。

    ```
    cargo make
    ```

    プロジェクト・ルートで上記実行。

    ```
    makers xxxxxx
    ```

    Makefile.toml の tasks.xxxxxx
