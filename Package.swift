// swift-tools-version: 5.10
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "tb_ga",
    platforms: [.iOS("15.0")],
    products: [
        // Products define the executables and libraries a package produces, making them visible to other packages.
        .library(
            name: "tb_ga",
            targets: ["tb_ga"]),
    ],
    dependencies: [],
    targets: [.target(name: "tb_ga")]
)
