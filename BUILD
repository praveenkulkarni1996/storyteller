load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

rust_binary(
    name = "storyteller_main",
    srcs = ["storyteller.rs"],
)

rust_library(
    name = "storyteller",
    srcs = ["storyteller.rs"],
)

rust_test(
    name = "chapter_love_and_death_test",
    srcs = ["chapters/love_and_death.rs"],
    deps = [":storyteller"],
)
