
function setwp(inContext) {

    Action.call(this, inContext);

    let piSaveSettings = this.saveSettingsCustom;

    document.getElementById("placeholder").innerHTML = "<div type=\"textarea\" class=\"sdpi-item\" id=\"required_text\">\n" +
        "    <div class=\"sdpi-item-label\">Label</div>\n" +
        "    <span class=\"sdpi-item-value\">\n" +
        "        <input type=\"text\"  id=\"wp\" required/>\n" +
        "    </span>\n" +
        "</div>"
    document.getElementById('wp').value = (settings.wp === undefined ? "" : settings.wp);
    document.getElementById('wp').addEventListener("input", altitudeChange)

    function altitudeChange(inEvent) {
        settings.wp = inEvent.target.value;
        piSaveSettings();
    }
}
