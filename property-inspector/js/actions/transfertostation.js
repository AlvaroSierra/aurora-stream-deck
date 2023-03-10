
function transfertostation(inContext) {

    Action.call(this, inContext);

    let piSaveSettings = this.saveSettingsCustom;

    document.getElementById("placeholder").innerHTML = "<div type=\"textarea\" class=\"sdpi-item\" id=\"required_text\">\n" +
        "    <div class=\"sdpi-item-label\">ATC position</div>\n" +
        "    <span class=\"sdpi-item-value\">\n" +
        "        <input type=\"text\"  id=\"navid\" required/>\n" +
        "    </span>\n" +
        "</div>"
    document.getElementById('navid').value = (settings.navaid === undefined ? "" : settings.navaid);
    document.getElementById('navid').addEventListener("input", navaidChanged)

    function navaidChanged(inEvent) {
        settings.navaid = inEvent.target.value;
        piSaveSettings();
    }
}
