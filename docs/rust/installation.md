# installation

## install by script

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## install binaries 

- `cargo install`: executable 프로그램 설치 위한 명령어
- [Installing Binaries with cargo install](https://doc.rust-lang.org/book/ch14-04-installing-binaries.html)

## install libraries

`cargo.toml` 파일에 디펜던시 추가 후 `cargo build`

```toml
[dependencies]
serde_json = "1.0.86"
```

```shell
cargo build
```

참고로 `cargo build` 후에 IntelliJ에서 `Load Cargo Changes` 버튼이 활성화 되는데, 해당 버튼을 클릭하지 않으면 IDE의 도움을 받지 못할 뿐, 라이브러리를 사용할 수 있다.
`Load Cargo Changes` 버튼은 IDE에서 새로 추가된 디펜던시를 추가하거나, `External Libraries`에 누락된 라이브러리가 있으면 가져와서 인덱싱 하는 등의 처리를 하기 위한 버튼일 뿐이다.

## 캐시 삭제

- cargo 통한 설치 또는 빌드 시 `~/.cargo/registry/cache` 하위 디렉토리에 캐싱된다 

```text
~/.cargo
    /bin # executable binaries 파일들이 설치 
    /registry
        /cache
            /github.com-1ecc6299db9ec823 # crate 파일들 위치
        /index
            /github.com-1ecc6299db9ec823
        /src
            /github.com-1ecc6299db9ec823 # 실제 소스 코드 위치
                /serde_json-1.0.86
                    /src
                    /tests
                    ... etc ...
    env
```

- [cargo-cache 통해서 캐시 정리](https://stackoverflow.com/a/68854692) 가능
  - `cache`, `index` 디렉토리는 남지만, `src` 디렉토리가 삭제된다

```shell
$ cargo install cargo-cache
$ cargo cache -a

Clearing cache...

Cargo cache '/Users/rody/.cargo':

Total:                                  1.83 GB => 567.94 MB
  14 installed binaries:                           128.00 MB
  Registry:                             1.70 GB => 439.95 MB
    Registry index:                                306.86 MB
    726 crate archives:                            133.08 MB
    726 => 0 crate source checkouts:         1.26 GB => 0  B
  Git db:                                               0  B
    0 bare git repos:                                   0  B
    0 git repo checkouts:                               0  B

Size changed 1.83 GB => 567.94 MB (-1.26 GB, -68.9%)
```