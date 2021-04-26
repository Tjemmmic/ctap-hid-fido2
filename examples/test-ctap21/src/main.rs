use ctap_hid_fido2;

use ctap_hid_fido2::bio_enrollment_params::TemplateInfo;
#[allow(unused_imports)]
use ctap_hid_fido2::util;
use ctap_hid_fido2::HidParam;
extern crate clap;
use clap::{App, Arg, SubCommand};
use ctap_hid_fido2::bio_enrollment_params::EnrollStatus1;

fn main() {
    let app = App::new("test-ctap21")
        .version("0.1.0")
        .author("gebo")
        .about("CTAP 2.1 command test app")
        .arg(
            Arg::with_name("pin")
                .help("pin")
                .short("p")
                .long("pin")
                .takes_value(true)
                .default_value("1234"),
        )
        .arg(
            Arg::with_name("info")
                .help("authenticatorGetInfo")
                .short("i")
                .long("info"),
        )
        .subcommand(SubCommand::with_name("bio_enrollment")
            .about("authenticatorBioEnrollment (0x09)")
            .arg(
                Arg::with_name("info")
                    .help("Get fingerprint sensor info")
                    .short("i")
                    .long("info"),
            )
            .arg(
                Arg::with_name("enumerate")
                    .help("Enumerate enrollments")
                    .short("e")
                    .long("enumerate"),
            )
            .arg(
                Arg::with_name("enroll")
                    .help("Enrolling fingerprint")
                    .short("n")
                    .long("enroll"),
            )
            .arg(
                Arg::with_name("rename")
                    .help("Rename/Set FriendlyName")
                    .short("r")
                    .long("rename")
                    .takes_value(true)
                    .value_name("templateId")
                    .value_name("templateFriendlyName"),
            )
            .arg(
                Arg::with_name("delete")
                    .help("Delete enrollment")
                    .short("d")
                    .long("delete")
                    .takes_value(true)
                    .value_name("templateId"),
            )
        );

    // Parse arguments
    let matches = app.get_matches();
    
    // Start
    ctap_hid_fido2::hello();

    match ctap_hid_fido2::enable_ctap_2_1(&HidParam::get_default_params()) {
        Ok(result) => println!("Enable CTAP 2.1 = {:?}", result),
        Err(error) => println!("- error: {:?}", error),
    };
    match ctap_hid_fido2::enable_ctap_2_1_pre(&HidParam::get_default_params()) {
        Ok(result) => println!("Enable CTAP 2.1 PRE = {:?}", result),
        Err(error) => println!("- error: {:?}", error),
    };

    if matches.is_present("info"){
        println!("get_info()");
        match ctap_hid_fido2::get_info(&HidParam::get_default_params()) {
            Ok(info) => println!("{}", info),
            Err(error) => println!("error: {:?}", error),
        };
    }

    let pin = matches.value_of("pin").unwrap();
    println!("Value for pin: {}", pin);
    println!("");
    println!("");

    // bio_enrollment
    if let Some(ref matches) = matches.subcommand_matches("bio_enrollment") {
        println!("used authenticatorBioEnrollment");
        println!("");
        
        if matches.is_present("info"){
            println!("Get fingerprint sensor info");
            match ctap_hid_fido2::bio_enrollment_get_fingerprint_sensor_info(&HidParam::get_default_params())
            {
                Ok(result) => {
                    println!("- {:?}", result);
                }
                Err(error) => {
                    println!(
                        "- bio_enrollment_get_fingerprint_sensor_info error: {:?}",
                        error
                    );
                }
            };
            println!("");
            println!("");
        }

        if matches.is_present("enumerate"){
            println!("Enumerate enrollments");
            match ctap_hid_fido2::bio_enrollment_enumerate_enrollments(
                &HidParam::get_default_params(),
                Some(pin),
            ) {
                Ok(template_infos) => {
                    for i in template_infos.iter() {
                        println!("- {}", i);
                    }
                }
                Err(error) => {
                    println!("- bio_enrollment_enumerate_enrollments error: {:?}", error);
                }
            };
            println!("");
            println!("");
        }

        if matches.is_present("rename"){
            let mut values = matches.values_of("rename").unwrap();
            let template_id = values.next().unwrap();
            let name = values.next();
            println!("Rename/Set FriendlyName");
            println!("- value for templateId: {:?}", template_id);
            println!("- value for templateFriendlyName: {:?}", name);
            println!("");

            match ctap_hid_fido2::bio_enrollment_set_friendly_name(
                &HidParam::get_default_params(),
                Some(pin),
                TemplateInfo::new(util::to_str_hex(template_id), name),
            ) {
                Ok(()) => {
                    println!("- Success");
                }
                Err(error) => {
                    println!("- bio_enrollment_enumerate_enrollments error: {:?}", error);
                }
            };
            println!("");
            println!("");
        }

        if matches.is_present("delete"){
            let template_id = matches.value_of("delete").unwrap();
            println!("Delete enrollment");
            println!("- value for templateId: {:?}", template_id);
            println!("");
        
            match ctap_hid_fido2::bio_enrollment_remove(
                &HidParam::get_default_params(),
                Some(pin),
                util::to_str_hex(template_id),
            ) {
                Ok(()) => {
                    println!("- Success");
                }
                Err(error) => {
                    println!("- bio_enrollment_remove error: {:?}", error);
                }
            };
            println!("");
            println!("");
        }

        if matches.is_present("enroll"){
            bio_enrollment(pin);
        }
    }

    /*
    println!("config()");
    match ctap_hid_fido2::config(&HidParam::get_default_params()) {
        Ok(result) => println!("- config : {:?}", result),
        Err(error) => println!("- config error: {:?}", error),
    };

    println!("selection()");
    match ctap_hid_fido2::selection(&HidParam::get_default_params()) {
        Ok(result) => println!("- selection : {:?}", result),
        Err(error) => println!("- selection error: {:?}", error),
    };
    */
}

fn bio_enrollment(pin: &str)->Result<(),String>{
    println!("bio_enrollment_begin");
    let enroll_status = match ctap_hid_fido2::bio_enrollment_begin(
        &HidParam::get_default_params(),
        Some(pin),
        Some(10000),
    ) {
        Ok(result) => {
            println!("{}",result.1);
            result.0
        }
        Err(error) => {
            let msg = format!("- bio_enrollment_begin error: {:?}", error);
            println!("{}",msg);
            return Err(msg)
        }
    };
    println!("");
    println!("");

    for _counter in 0..10 {
        if bio_enrollment_next(&enroll_status)? {
            break;
        }
    }
    Ok(())
}

fn bio_enrollment_next(enroll_status: &EnrollStatus1)->Result<bool,String>{
    println!("bio_enrollment_next");
    match ctap_hid_fido2::bio_enrollment_next(
        enroll_status,
        Some(10000),
    ) {
        Ok(result) => {
            println!("{}",result);
            println!("");
            println!("");
            return Ok(result.is_finish)
        }
        Err(error) => {
            let msg = format!("- bio_enrollment_next error: {:?}", error);
            println!("{}",msg);
            return Err(msg)
        }
    };
}