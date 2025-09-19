// tests7.rs
//
// When building packages, some dependencies can neither be imported in
// `Cargo.toml` nor be directly linked; some preprocesses varies from code
// generation to set-up package-specific configurations.
//
// Cargo does not aim to replace other build tools, but it does integrate
// with them with custom build scripts called `build.rs`. This file is
// usually placed in the root of the project, while in this case the same
// directory of this exercise.
//
// It can be used to:
//
// - Building a bundled C library.
// - Finding a C library on the host system.
// - Generating a Rust module from a specification.
// - Performing any platform-specific configuration needed for the crate.
//
// When setting up configurations, we can `println!` in the build script
// to tell Cargo to follow some instructions. The generic format is:
//
//     println!("cargo:{}", your_command_in_string);
//
// Please see the official Cargo book about build scripts for more
// information:
// https://doc.rust-lang.org/cargo/reference/build-scripts.html
//
// In this exercise, we look for an environment variable and expect it to
// fall in a range. You can look into the testcase to find out the details.
//
// You should NOT modify this file. Modify `build.rs` in the same directory
// to pass this exercise.
//
//在构建包的时候，有些依赖关系既不能在// `Cargo.toml `中导入，也不能直接链接；从代码//生成到设置特定于封装的配置，一些预处理各不相同。//// Cargo的目的不是取代其他构建工具，而是通过名为“build.rs”的自定义构建脚本将//这些工具集成在一起。该文件//通常放在项目的根目录中，而在本例中，该目录与//本练习中的目录相同。////可以用来://// -构建捆绑的C库。// -在主机系统上查找C库。// -根据规范生成Rust模块。// -执行板条箱所需的任何特定于平台的配置。////在设置配置时，我们可以` println！`在构建脚本中//告诉Cargo遵循一些指令。通用格式为://// println！(“cargo:{}”，your _ command _ in _ string)；////有关更多//信息，请参见关于构建脚本的官方Cargo book://https://doc . rust-lang . org/Cargo/reference/build-scripts . html////在本练习中，我们查找一个环境变量，并期望它//落在一个范围内。您可以查看测试用例来找出细节。////您不应该修改此文件。修改同一个目录//中的“build.rs”即可通过本练习

// Execute `rustlings hint tests7` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e: u64 = s.parse().unwrap();
        assert!(timestamp >= e && timestamp < e + 10);
    }
}
