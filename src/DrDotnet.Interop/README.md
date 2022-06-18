# DrDotnet.Interop

This part handles the automatic generation of an interface to do the interoperability between C# code (UI) and Rust code (profilers).   
This is based on [FFIDJI](https://github.com/ogxd/ffidji), which is a tool written in Rust which is a code generation tool for easy interoperability between several languages.   
Currently, there is a prebuilt version of FFIDJI for Windows (ffidji.exe) for simplicity. If you need the regenerate the interface on another platform, you'll need to clone FFIDJI and build it for your platform.