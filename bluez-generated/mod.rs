// This code was autogenerated with `dbus-codegen-rust --system-bus --destination org.bluez --path /org/bluez/hci0/dev_A4_C1_38_1E_0A_E8 --client nonblock --futures`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus_futures as dbusf;
use dbus::tree;

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> dbusf::MethodReply<String>;
}

pub fn org_freedesktop_dbus_introspectable_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    T: OrgFreedesktopDBusIntrospectable,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Introspectable", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        let xml = d.introspect()?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(xml);
        Ok(vec!(rm))
    };
    let m = factory.method("Introspect", Default::default(), h);
    let m = m.out_arg(("xml", "s"));
    let i = i.add_m(m);
    i
}

pub trait OrgBluezDevice1 {
    fn disconnect(&self) -> dbusf::MethodReply<()>;
    fn connect(&self) -> dbusf::MethodReply<()>;
    fn connect_profile(&self, uuid: &str) -> dbusf::MethodReply<()>;
    fn disconnect_profile(&self, uuid: &str) -> dbusf::MethodReply<()>;
    fn pair(&self) -> dbusf::MethodReply<()>;
    fn cancel_pairing(&self) -> dbusf::MethodReply<()>;
    fn address(&self) -> dbusf::MethodReply<String>;
    fn address_type(&self) -> dbusf::MethodReply<String>;
    fn name(&self) -> dbusf::MethodReply<String>;
    fn alias(&self) -> dbusf::MethodReply<String>;
    fn set_alias(&self, value: String) -> dbusf::MethodReply<()>;
    fn class(&self) -> dbusf::MethodReply<u32>;
    fn appearance(&self) -> dbusf::MethodReply<u16>;
    fn icon(&self) -> dbusf::MethodReply<String>;
    fn paired(&self) -> dbusf::MethodReply<bool>;
    fn trusted(&self) -> dbusf::MethodReply<bool>;
    fn set_trusted(&self, value: bool) -> dbusf::MethodReply<()>;
    fn blocked(&self) -> dbusf::MethodReply<bool>;
    fn set_blocked(&self, value: bool) -> dbusf::MethodReply<()>;
    fn legacy_pairing(&self) -> dbusf::MethodReply<bool>;
    fn rssi(&self) -> dbusf::MethodReply<i16>;
    fn connected(&self) -> dbusf::MethodReply<bool>;
    fn uuids(&self) -> dbusf::MethodReply<Vec<String>>;
    fn modalias(&self) -> dbusf::MethodReply<String>;
    fn adapter(&self) -> dbusf::MethodReply<dbus::Path<'static>>;
    fn manufacturer_data(&self) -> dbusf::MethodReply<::std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>>;
    fn service_data(&self) -> dbusf::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>;
    fn tx_power(&self) -> dbusf::MethodReply<i16>;
    fn services_resolved(&self) -> dbusf::MethodReply<bool>;
}

pub fn org_bluez_device1_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    T: OrgBluezDevice1,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.bluez.Device1", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.disconnect()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Disconnect", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.connect()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Connect", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let uuid: &str = i.read()?;
        let d = fclone(minfo);
        d.connect_profile(uuid)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("ConnectProfile", Default::default(), h);
    let m = m.in_arg(("UUID", "s"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let uuid: &str = i.read()?;
        let d = fclone(minfo);
        d.disconnect_profile(uuid)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("DisconnectProfile", Default::default(), h);
    let m = m.in_arg(("UUID", "s"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.pair()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Pair", Default::default(), h);
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let d = fclone(minfo);
        d.cancel_pairing()?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("CancelPairing", Default::default(), h);
    let i = i.add_m(m);

    let p = factory.property::<&str, _>("Address", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.address()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("AddressType", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.address_type()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Name", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.name()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Alias", Default::default());
    let p = p.access(tree::Access::ReadWrite);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.alias()?);
        Ok(())
    });
    let fclone = f.clone();
    let p = p.on_set(move |iter, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        d.set_alias(iter.read()?)?;
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u32, _>("Class", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.class()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<u16, _>("Appearance", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.appearance()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Icon", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.icon()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("Paired", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.paired()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("Trusted", Default::default());
    let p = p.access(tree::Access::ReadWrite);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.trusted()?);
        Ok(())
    });
    let fclone = f.clone();
    let p = p.on_set(move |iter, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        d.set_trusted(iter.read()?)?;
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("Blocked", Default::default());
    let p = p.access(tree::Access::ReadWrite);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.blocked()?);
        Ok(())
    });
    let fclone = f.clone();
    let p = p.on_set(move |iter, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        d.set_blocked(iter.read()?)?;
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("LegacyPairing", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.legacy_pairing()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<i16, _>("RSSI", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.rssi()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("Connected", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.connected()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<Vec<&str>, _>("UUIDs", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.uuids()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<&str, _>("Modalias", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.modalias()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<dbus::Path, _>("Adapter", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.adapter()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<::std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg>>>, _>("ManufacturerData", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.manufacturer_data()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>, _>("ServiceData", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.service_data()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<i16, _>("TxPower", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.tx_power()?);
        Ok(())
    });
    let i = i.add_p(p);

    let p = factory.property::<bool, _>("ServicesResolved", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.services_resolved()?);
        Ok(())
    });
    let i = i.add_p(p);
    i
}

pub trait OrgFreedesktopDBusProperties {
    fn get(&self, interface: &str, name: &str) -> dbusf::MethodReply<arg::Variant<Box<dyn arg::RefArg + 'static>>>;
    fn set(&self, interface: &str, name: &str, value: arg::Variant<Box<dyn arg::RefArg>>) -> dbusf::MethodReply<()>;
    fn get_all(&self, interface: &str) -> dbusf::MethodReply<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>;
}

pub fn org_freedesktop_dbus_properties_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Signal: Default,
    T: OrgFreedesktopDBusProperties,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.freedesktop.DBus.Properties", data);
    let f = ::std::sync::Arc::new(f);
    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface: &str = i.read()?;
        let name: &str = i.read()?;
        let d = fclone(minfo);
        let value = d.get(interface, name)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(value);
        Ok(vec!(rm))
    };
    let m = factory.method("Get", Default::default(), h);
    let m = m.in_arg(("interface", "s"));
    let m = m.in_arg(("name", "s"));
    let m = m.out_arg(("value", "v"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface: &str = i.read()?;
        let name: &str = i.read()?;
        let value: arg::Variant<Box<dyn arg::RefArg>> = i.read()?;
        let d = fclone(minfo);
        d.set(interface, name, value)?;
        let rm = minfo.msg.method_return();
        Ok(vec!(rm))
    };
    let m = factory.method("Set", Default::default(), h);
    let m = m.in_arg(("interface", "s"));
    let m = m.in_arg(("name", "s"));
    let m = m.in_arg(("value", "v"));
    let i = i.add_m(m);

    let fclone = f.clone();
    let h = move |minfo: &tree::MethodInfo<tree::MTFn<D>, D>| {
        let mut i = minfo.msg.iter_init();
        let interface: &str = i.read()?;
        let d = fclone(minfo);
        let properties = d.get_all(interface)?;
        let rm = minfo.msg.method_return();
        let rm = rm.append1(properties);
        Ok(vec!(rm))
    };
    let m = factory.method("GetAll", Default::default(), h);
    let m = m.in_arg(("interface", "s"));
    let m = m.out_arg(("properties", "a{sv}"));
    let i = i.add_m(m);
    let s = factory.signal("PropertiesChanged", Default::default());
    let s = s.arg(("interface", "s"));
    let s = s.arg(("changed_properties", "a{sv}"));
    let s = s.arg(("invalidated_properties", "as"));
    let i = i.add_s(s);
    i
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface: String,
    pub changed_properties: ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

pub trait OrgBluezBattery1 {
    fn percentage(&self) -> dbusf::MethodReply<u8>;
}

pub fn org_bluez_battery1_server<F, T, D>(factory: &tree::Factory<tree::MTFn<D>, D>, data: D::Interface, f: F) -> tree::Interface<tree::MTFn<D>, D>
where
    D: tree::DataType,
    D::Method: Default,
    D::Property: Default,
    T: OrgBluezBattery1,
    F: 'static + for <'z> Fn(& 'z tree::MethodInfo<tree::MTFn<D>, D>) -> & 'z T,
{
    let i = factory.interface("org.bluez.Battery1", data);
    let f = ::std::sync::Arc::new(f);
    let p = factory.property::<u8, _>("Percentage", Default::default());
    let p = p.access(tree::Access::Read);
    let fclone = f.clone();
    let p = p.on_get(move |a, pinfo| {
        let minfo = pinfo.to_method_info();
        let d = fclone(&minfo);
        a.append(d.percentage()?);
        Ok(())
    });
    let i = i.add_p(p);
    i
}
