
function zoomtonavaid(inContext) {

    Action.call(this, inContext);

    var piSaveSettings = this.saveSettingsCustom;

    let navaid_input = "<div type=\"textarea\" class=\"sdpi-item\" id=\"required_text\">\n" +
        "    <div class=\"sdpi-item-label\">Some Text</div>\n" +
        "    <span class=\"sdpi-item-value\">\n" +
        "        <input type=\"text\"  id=\"navid\" required></input>\n" +
        "    </span>\n" +
        "</div>"



    document.getElementById("placeholder").innerHTML = navaid_input
    document.getElementById('navid').value = (settings.navaid === undefined ? "" : settings.navaid);
    document.getElementById('navid').addEventListener("input", navaidChanged)

    function navaidChanged(inEvent) {
        settings.navaid = inEvent.target.value;
        piSaveSettings();
    }
}
