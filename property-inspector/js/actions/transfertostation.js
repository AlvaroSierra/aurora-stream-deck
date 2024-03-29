
function transfertostation(inContext) {

    Action.call(this, inContext);

    let piSaveSettings = this.saveSettingsCustom;

    document.getElementById("placeholder").innerHTML = "<div type=\"textarea\" class=\"sdpi-item\" id=\"required_text\">\n" +
        "    <div class=\"sdpi-item-label\">ATC position</div>\n" +
        "    <span class=\"sdpi-item-value\">\n" +
        "        <input type=\"text\"  id=\"station\" required/>\n" +
        "    </span>\n" +
        "</div>"
    document.getElementById('station').value = (settings.station === undefined ? "" : settings.station);
    document.getElementById('station').addEventListener("input", stationChange)

    function stationChange(inEvent) {
        settings.station = inEvent.target.value;
        piSaveSettings();
    }
}
