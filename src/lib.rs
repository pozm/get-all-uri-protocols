use winreg::{enums::HKEY_CLASSES_ROOT, RegKey};

#[derive(Debug)]
pub struct UriProtocol {
    pub protocol : String,
    pub description : String,
    pub command : String,
}

pub fn get_protocols() -> Result<Vec<UriProtocol>,String> {
    let hkey_cr = RegKey::predef(HKEY_CLASSES_ROOT);

    let mut protocols = vec![];


    for key_ in hkey_cr.enum_keys() {
        if let Ok(key) = key_ {
            let v = hkey_cr.open_subkey(&key).or(Err("unable to open key"))?;
            if let Ok(proto) = v.get_value::<String,&str>("URL Protocol") {
                if let Ok(default) = v.get_value::<String,&str>("") {
                    if let Ok(shell) = v.open_subkey("shell\\open\\command") {
                        if let Ok(command_to_run) = shell.get_value::<String,&str>("") {
                            if default.starts_with("URL:") {
                                let uri = &default[4..];
                                protocols.push(UriProtocol {
                                    protocol : key.to_string(),
                                    description : uri.to_string(),
                                    command : command_to_run.to_string(),
                                });
                                // println!("{}, {} | {}", key,uri,command_to_run);
                            }
                        }
                    }
                }

            }
        }
    }


    Ok(protocols)
}