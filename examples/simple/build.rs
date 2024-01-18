fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = tauri_winres::WindowsResource::new();
        res.set_icon("app.ico");
        #[cfg(feature = "dpiaware")]
        res.set_manifest(r#"
            <?xml version="1.0" encoding="UTF-8" standalone="yes"?>
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
            <asmv3:application xmlns:asmv3="urn:schemas-microsoft-com:asm.v3" >
                <asmv3:windowsSettings xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">
                    <dpiAware>true</dpiAware>
                </asmv3:windowsSettings>
            </asmv3:application>
            <dependency>
                <dependentAssembly>
                    <assemblyIdentity
                        type="win32"
                        name="Microsoft.Windows.Common-Controls"
                        version="6.0.0.0"
                        processorArchitecture="*"
                        publicKeyToken="6595b64144ccf1df"
                        language="*"
                    />
                </dependentAssembly>
            </dependency>
            </assembly>
        "#);
        res.compile().unwrap();
    }
}
