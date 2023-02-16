all: clean build-plugin build-pi build-resources

clean:
	-del /S /Q "%AppData%\Elgato\StreamDeck\Plugins\com.alvaro.aurorastream.sdPlugin\*"

build-resources:
	-mkdir "%AppData%\Elgato\StreamDeck\Plugins\com.alvaro.aurorastream.sdPlugin\\"
	xcopy /E /Y "Plugin" "%AppData%\Elgato\StreamDeck\Plugins\com.alvaro.aurorastream.sdPlugin\\"

build-pi:
	-mkdir "%AppData%\Elgato\StreamDeck\Plugins\com.alvaro.aurorastream.sdPlugin\\PropertyInspector\\"
	xcopy /E /Y "property-inspector" "%AppData%\Elgato\StreamDeck\Plugins\com.alvaro.aurorastream.sdPlugin\PropertyInspector\\"

build-plugin:
	cd rust && cargo build
	-mkdir "%AppData%\Elgato\StreamDeck\Plugins\com.alvaro.aurorastream.sdPlugin\\"
	copy "rust\target\debug\aurorastream.exe" "%AppData%\Elgato\StreamDeck\Plugins\com.alvaro.aurorastream.sdPlugin\aurorastream.exe"


