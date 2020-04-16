# Rust - LabVIEW Integration Examples
This repo demonstrates Rust-LabVIEW integration with a basic example and a more advanced use case. Why combine these two programming approaches? 

[NI.com](https://www.ni.com/en-us/shop/labview.html) describes LabVIEW as "systems engineering software for applications that require test, measurement, and control with rapid access to hardware and data insights". From *[This Week in Rust](https://this-week-in-rust.org/)*, "[Rust](https://www.rust-lang.org/) is a systems language pursuing the trifecta: safety, concurrency, and speed". 

Combining Rust with LabVIEW's automated test and measurement systems can help demonstrate the benefits of Rust to the scientific and engineering community.

These examples were written with:
- Rust 1.42.0
- LabVIEW 2019
- LabVIEW NXG 4.0
- NI-DAQmx 19.0

## Interoperability Overview
In 2015, the Rust team published a [Rust Once, Rust Everywhere](https://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html) article. A few snippets are particularly interesting: 

>To communicate with other languages, Rust provides a foreign function interface (FFI). Following Rust's design principles, the FFI provides a zero-cost abstraction where function calls between Rust and C have identical performance to C function calls."

>Despite guaranteeing memory safety, Rust does not have a garbage collector or runtime, and one of the benefits of this is that Rust code can be called from C with no setup at all."

If Rust plays nice with C, then we can integrate it smoothely into LabVIEW. The [Call Library Funcion Node](https://zone.ni.com/reference/en-XX/help/371361R-01/glang/call_library_function/) allows LabVIEW 20XX to directly call external libraries. Similarly, LabVIEW NXG uses the [Shared Library Interface](https://www.ni.com/documentation/en/labview/latest/language-integration/shared-library-interfaces/) to interact with external code.

The Rust-perspective on interacting with foreign code is well documented in the [manual](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code) and the [Rust Omnibus](http://jakegoulding.com/rust-ffi-omnibus/basics/). Basically, you can use the [libc crate](https://crates.io/crates/libc) to build dynamically linked libraries that interoperate with C code.

## Getting Started Example
Using the [Rust Omnibus Integers](http://jakegoulding.com/rust-ffi-omnibus/integers/) approach, `rust-labview-basic` demonstrates a simple incrementing function writen in Rust and then consumed by a [LabVIEW VI (.vi)](https://www.ni.com/getting-started/labview-basics/environment) and a [LabVIEW NXG VI (.gvi)](https://www.ni.com/documentation/en/labview/latest/create-first/create-vi/).

## More Advanced Example
It is relatively common for teams to share known measurement systems and to specify configuration files that define how to acquire data from their measurement hardware. In this example, a team uses [YAML](https://en.wikipedia.org/wiki/YAML) to define their analog input measurements. For example:

```yml
cDAQ_configurations:
  - configuration_name: medium_voltage
    hardware_channels: cDAQ1Mod1/ai0:3
    min_voltage: -5
    max_voltage: 5
    sample_rate_Hz: 100
```
After looking through the [NI Tools Network](http://www.ni.com/labview-tools-network/), I could not find any pre-existing libraries for parsing YAML. Fortunately there is a [yaml-rust](https://crates.io/crates/yaml-rust) crate.

Using the [Rust Omnibus Objects](http://jakegoulding.com/rust-ffi-omnibus/objects/) approach, `rust-labview-yaml` handles the YAML parsing so the corresponding LabVIEW and LabVIEW NXG systems can correctly configure the analog input measurement hardware.

To use the examples in the `lv2019` and `NXG` directories, [simulate the following CompactDAQ hardware in NI MAX](http://www.ni.com/tutorial/3698/en/):
- NI cDAQ-9189 "cDAQ1"
- NI 9201 "cDAQ1Mod1"
- NI 9205 "cDAQ1Mod2"
- NI 9206 "cDAQ1Mod3"

## Additional ideas
With the Rust trifecta and LabVIEW's domain expertise, there are a lot of project opportunities. Here are a few ideas:

- Rust program using the [FPGA C API](https://www.ni.com/en-us/support/documentation/supplemental/09/introduction-to-the-fpga-interface-c-api.html) running on a [Linux Real-Time](https://www.ni.com/en-us/innovations/white-papers/13/introduction-to-ni-linux-real-time.html) target
- Highly parallelized data file publishing service
- LabVIEW NXG UI for prototyping Rust signal analysis libraries
- Rust-built server managing distributed measurement systems (nodes programmed with LabVIEW Real-Time and LabVIEW FPGA)

