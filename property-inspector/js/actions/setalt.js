
function setalt(inContext) {

    Action.call(this, inContext);

    let piSaveSettings = this.saveSettingsCustom;

    document.getElementById("placeholder").innerHTML = "<div type=\"textarea\" class=\"sdpi-item\" id=\"required_text\">\n" +
        "    <div class=\"sdpi-item-label\">Label</div>\n" +
        "    <span class=\"sdpi-item-value\">\n" +
        "        <input type=\"text\"  id=\"alt\" required/>\n" +
        "    </span>\n" +
        "</div>"
    document.getElementById('alt').value = (settings.alt === undefined ? "" : settings.alt);
    document.getElementById('alt').addEventListener("input", altitudeChange)

    function altitudeChange(inEvent) {
        settings.alt = inEvent.target.value;
        piSaveSettings();
    }
}
