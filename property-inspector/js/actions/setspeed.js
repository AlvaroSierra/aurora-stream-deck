
function setspeed(inContext) {

    Action.call(this, inContext);

    let piSaveSettings = this.saveSettingsCustom;

    document.getElementById("placeholder").innerHTML = "<div type=\"textarea\" class=\"sdpi-item\" id=\"required_text\">\n" +
        "    <div class=\"sdpi-item-label\">Label</div>\n" +
        "    <span class=\"sdpi-item-value\">\n" +
        "        <input type=\"text\"  id=\"spd\" required/>\n" +
        "    </span>\n" +
        "</div>"
    document.getElementById('spd').value = (settings.spd === undefined ? "" : settings.spd);
    document.getElementById('spd').addEventListener("input", speedChanged)

    function speedChanged(inEvent) {
        settings.spd = inEvent.target.value;
        piSaveSettings();
    }
}
