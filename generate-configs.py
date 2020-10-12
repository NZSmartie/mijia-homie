#!/usr/bin/env python3
import json
from collections import defaultdict
import os


def mutate_grafana_model(model, mac_to_name):
    panel_to_reading_types = defaultdict(set)
    seen = set()

    for panel_idx, panel in enumerate(model["panels"]):
        for target in panel["targets"]:
            if not target["measurement"].startswith("MijiaBridge"):
                continue
            try:
                (_prefix, mac, reading_type) = target["measurement"].split("_")
            except:
                continue

            panel_to_reading_types[panel_idx].add(reading_type)
            seen.add((mac, reading_type))

    for panel_idx, panel in enumerate(model["panels"]):
        reading_types = panel_to_reading_types[panel_idx]
        if len(reading_types) != 1:
            # Don't add readings to a panel with both temperature and humidity,
            # or with no readings of either type.
            continue
        (reading_type,) = reading_types

        for mac, name in mac_to_name.items():
            if (mac, reading_type) not in seen:
                print(f"Adding {mac} {reading_type} {name} to {panel_idx}")
                panel["targets"].append(
                    {
                        "alias": name,
                        "groupBy": [
                            {"params": ["$__interval"], "type": "time"},
                            {"params": ["none"], "type": "fill"},
                        ],
                        "measurement": f"MijiaBridge_{mac}_{reading_type}",
                        "orderByTime": "ASC",
                        "policy": "default",
                        "resultFormat": "time_series",
                        "select": [
                            [
                                {"params": ["value"], "type": "field"},
                                {"params": [], "type": "mean"},
                            ]
                        ],
                        "tags": [],
                    }
                )


def mutate_smarthome_items_model(model, mac_to_name):
    for mac, name in mac_to_name.items():
        if f"MijiaBridge_{mac}" not in model:
            print(f"adding MijiaBridge_{mac} => {name}")
            model[f"MijiaBridge_{mac}"] = {
                "class": "org.eclipse.smarthome.core.items.ManagedItemProvider$PersistedItem",
                "value": {
                    "groupNames": ["MijiaBridge"],
                    "itemType": "Group",
                    "tags": [],
                    "label": f"{name} sensor",
                },
            }
        for key_suffix, label_suffix in [
            ("Temperature", "temperature"),
            ("Humidity", "humidity"),
            ("BatteryLevel", "battery level"),
        ]:
            key = f"MijiaBridge_{mac}_{key_suffix}"
            value = {
                "class": "org.eclipse.smarthome.core.items.ManagedItemProvider$PersistedItem",
                "value": {
                    "groupNames": [f"MijiaBridge_{mac}"],
                    "itemType": "Number",
                    "tags": [],
                    "label": f"{name} {label_suffix}",
                },
            }
            if key not in model:
                print(f"adding {key} => {name}")
                model[key] = value
            assert model[key] == value


def metadata_key(mac, title):
    return f"ga:{title}"


def new_metadata(title, value):
    return (
        {
            "class": "org.eclipse.smarthome.core.items.Metadata",
            "value": {
                "key": {"segments": ["ga", title]},
                "value": value,
                "configuration": {},
            },
        },
    )


def mutate_smarthome_metadata_model(model, mac_to_name):
    for mac, name in mac_to_name.items():
        for title, value in [
            ("MijiaBridge_{mac}", "Thermostat"),
            ("MijiaBridge_{mac}_Temperature", "thermostatTemperatureAmbient"),
            ("MijiaBridge_{mac}_Humidity", "thermostatHumidityAmbient"),
        ]:
            key = metadata_key(mac, title)
            if key not in model:
                model[key] = new_metadata(title, value)


def link_key(mac, title_suffix, homie_suffix):
    return (
        f"MijiaBridge_{mac}_{title_suffix} -> "
        f"mqtt:homie300:19078e8a:mijia-bridge-raspi{mac}#{homie_suffix}"
    )


def new_link(mac, title_suffix, homie_suffix):
    return {
        "class": "org.eclipse.smarthome.core.thing.link.ItemChannelLink",
        "value": {
            "channelUID": {
                "segments": [
                    "mqtt",
                    "homie300",
                    "19078e8a",
                    "mijia-bridge-raspi",
                    f"{mac}#{homie_suffix}",
                ]
            },
            "configuration": {"properties": {"profile": "system:default"}},
            "itemName": f"MijiaBridge_{mac}_{title_suffix}",
        },
    }


def mutate_smarthome_link_model(model, mac_to_name):
    for mac, name in mac_to_name.items():
        for title_suffix, homie_suffix in [
            ("Temperature", "temperature"),
            ("Humidity", "humidity"),
            ("BatteryLevel", "battery"),
        ]:
            key = link_key(mac, title_suffix, homie_suffix)
            if key not in model:
                model[key] = new_link(mac, title_suffix, homie_suffix)


def add_named_sensors_to_grafana(mac_to_name, infilename, outfilename):
    with open(infilename) as f:
        model = json.load(f)

    mutate_grafana_model(model, mac_to_name)

    with open(outfilename, "w") as f:
        json.dump(model, f, indent=2)
        f.write("\n")


def add_named_sensors_to_smarthome_items(mac_to_name, infilename, outfilename):
    with open(infilename) as f:
        model = json.load(f)

    mutate_smarthome_items_model(model, mac_to_name)

    with open(outfilename, "w") as f:
        json.dump(model, f, indent=2)
        f.write("\n")


def add_named_sensors_to_smarthome_metadata(mac_to_name, infilename, outfilename):
    with open(infilename) as f:
        model = json.load(f)

    mutate_smarthome_metadata_model(model, mac_to_name)

    with open(outfilename, "w") as f:
        json.dump(model, f, indent=2)
        f.write("\n")


def add_named_sensors_to_smarthome_link(mac_to_name, infilename, outfilename):
    with open(infilename) as f:
        model = json.load(f)

    mutate_smarthome_link_model(model, mac_to_name)

    with open(outfilename, "w") as f:
        json.dump(model, f, indent=2)
        f.write("\n")


if __name__ == "__main__":
    namesfilename = os.environ.get("NAMES_INPUT")
    if not namesfilename:
        print(
            "Please set NAMES_INPUT and either GRAFANA_ or SMARTHOME_JSONDB_ "
            "INPUT and OUTPUT."
        )
        exit(1)

    with open(namesfilename) as f:
        mac_to_name = dict(l.strip().replace(":", "").split("=") for l in f)

    grafana_input = os.environ.get("GRAFANA_INPUT")
    grafana_output = os.environ.get("GRAFANA_OUTPUT")
    if grafana_input and grafana_output:
        add_named_sensors_to_grafana(
            mac_to_name, grafana_input, grafana_output,
        )

    smarthome_jsondb_input = os.environ.get("SMARTHOME_JSONDB_INPUT")
    smarthome_jsondb_output = os.environ.get("SMARTHOME_JSONDB_OUTPUT")
    if smarthome_jsondb_input and smarthome_jsondb_output:
        if not os.path.exists(smarthome_jsondb_output):
            os.mkdir(smarthome_jsondb_output)
        add_named_sensors_to_smarthome_items(
            mac_to_name,
            f"{smarthome_jsondb_input}/org.eclipse.smarthome.core.items.Item.json",
            f"{smarthome_jsondb_output}/org.eclipse.smarthome.core.items.Item.json",
        )
        # TODO: org.eclipse.smarthome.core.items.Metadata.json
        add_named_sensors_to_smarthome_metadata(
            mac_to_name,
            f"{smarthome_jsondb_input}/org.eclipse.smarthome.core.items.Metadata.json",
            f"{smarthome_jsondb_output}/org.eclipse.smarthome.core.items.Metadata.json",
        )
        # TODO: maybe also org.eclipse.smarthome.core.thing.Thing.json?

        # org.eclipse.smarthome.core.thing.link.ItemChannelLink.json
        add_named_sensors_to_smarthome_link(
            mac_to_name,
            f"{smarthome_jsondb_input}/org.eclipse.smarthome.core.thing.link.ItemChannelLink.json",
            f"{smarthome_jsondb_output}/org.eclipse.smarthome.core.thing.link.ItemChannelLink.json",
        )