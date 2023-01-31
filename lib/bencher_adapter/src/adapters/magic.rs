use crate::{
    results::adapter_results::AdapterResults, Adapter, AdapterCpp, AdapterGo, AdapterJava,
    AdapterJson, AdapterRust,
};

pub struct AdapterMagic;

impl Adapter for AdapterMagic {
    fn parse(input: &str) -> Option<AdapterResults> {
        AdapterJson::parse(input)
            .or_else(|| AdapterCpp::parse(input))
            .or_else(|| AdapterGo::parse(input))
            .or_else(|| AdapterJava::parse(input))
            .or_else(|| AdapterRust::parse(input))
    }
}

#[cfg(test)]
mod test_magic {
    use super::AdapterMagic;
    use crate::adapters::{
        cpp::{catch2::test_cpp_catch2, google::test_cpp_google},
        go::bench::test_go_bench,
        java::jmh::test_java_jmh,
        json::test_json,
        rust::{bench::test_rust_bench, criterion::test_rust_criterion},
        test_util::convert_file_path,
    };

    #[test]
    fn test_adapter_magic_json_latency() {
        let results = convert_file_path::<AdapterMagic>("./tool_output/json/report_latency.json");
        test_json::validate_adapter_json_latency(results);
    }

    #[test]
    fn test_adapter_magic_cpp_google() {
        let results = convert_file_path::<AdapterMagic>("./tool_output/cpp/google/two.txt");
        test_cpp_google::validate_adapter_cpp_google(results);
    }

    #[test]
    fn test_adapter_magic_cpp_catch2() {
        let results = convert_file_path::<AdapterMagic>("./tool_output/cpp/catch2/four.txt");
        test_cpp_catch2::validate_adapter_cpp_catch2(results);
    }

    #[test]
    fn test_adapter_magic_go_bench() {
        let results = convert_file_path::<AdapterMagic>("./tool_output/go/bench/five.txt");
        test_go_bench::validate_adapter_go_bench(results);
    }

    #[test]
    fn test_adapter_magic_java_jmh() {
        let results = convert_file_path::<AdapterMagic>("./tool_output/java/jmh/six.json");
        test_java_jmh::validate_adapter_java_jmh(results);
    }

    #[test]
    fn test_adapter_magic_rust_bench() {
        let results = convert_file_path::<AdapterMagic>("./tool_output/rust/bench/many.txt");
        test_rust_bench::validate_adapter_rust_bench(results);
    }

    #[test]
    fn test_adapter_magic_rust_criterion() {
        let results = convert_file_path::<AdapterMagic>("./tool_output/rust/criterion/many.txt");
        test_rust_criterion::validate_adapter_rust_criterion(results);
    }
}
