# Rust in Java

Using JNI.

A minimal example.

```
cargo install jni
```

```
$ javac -h src src/HelloWorld.java
```

```
$ cargo build
```

```
$ LD_LIBRARY_PATH=target/debug/ java src/HelloWorld.java
```
