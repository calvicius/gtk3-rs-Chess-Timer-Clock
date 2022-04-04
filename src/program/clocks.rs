use gtk::{self, prelude::*};

static mut STATUS: Option<Timer> = None;

const CSSCLOCKS: &str = "
#frm_clock {
    border: 1px solid #1E1E1E; 
    border-radius: 0px;
    /*border-color: #1E1E1E;*/
}

#box_clock2 {
    padding-left: 15px;
    padding-right: 15px;
    padding-top: 8px;
    padding-bottom: 8px;
    background: #AAAAAA;
}
#box_clock1 {
    padding-left: 15px;
    padding-right: 15px;
    padding-top: 8px;
    padding-bottom: 8px;
    background: #EEEEEE;
}
#lbl_clock2 {
    font-size: 24px;
    font-weight: bold;
    color: #FFFFFF;
}
#lbl_clock1 {
    font-size: 24px;
    font-weight: bold;
    color: #000000;
}
#lbl_clock1_red, #lbl_clock2_red {
    font-size: 24px;
    font-weight: bold;
    color: #CC0000;
}
";

#[derive(Clone)]
struct Player {
    pub player: String,
    minutes: i32,
}

impl Player {
    pub fn new(player: String, minutes: &i32) -> Self {
        Player {
            player,
            minutes: *minutes,
        }
    }

    pub fn get_minutes(&mut self, lbl: gtk::Label) -> String {
        lbl.text().to_string()
    }
}

#[derive(Clone)]
pub struct Timer {
    playing: bool,
    current_player: i32,
    p1time: Player, // time of player 1
    p2time: Player, //    "       "   2
    pub hbox_clocks: gtk::Box,
    min1: gtk::Label,
    min2: gtk::Label,
    sec1: gtk::Label,
    sec2: gtk::Label,
    sep1: gtk::Label,
    sep2: gtk::Label,
    btn_white: gtk::Button,
    btn_black: gtk::Button,
    btn_start: gtk::Button,
}

unsafe impl Send for Timer {}

impl Timer {
    pub fn new(limit_mins: i32) -> Self {
        // Cargamos el CSS
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(CSSCLOCKS.as_bytes())
            .expect("Failed to load CSS");
        // Damos el Css provisto a la pantalla predeterminada para que las reglas de CSS 
        // que agregamos se puedan aplicar a nuestra ventana.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_USER,
        );

        let playing = false;
        let current_player = 1;
        let p1time = Player::new("Player1".to_string(), &limit_mins);
        let p2time = Player::new("Player2".to_string(), &limit_mins);

        let hbox_clocks = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
        let hbox_labels = gtk::Box::new(gtk::Orientation::Horizontal, 5);

        //white clock
        let txt = format!("{}: ", p1time.player);
        let lbl_white = gtk::Label::new(Some(&txt));

        let hlblbox1 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        hlblbox1.set_widget_name("box_clock1");
        let frm1 = gtk::Frame::new(None);
        frm1.set_widget_name("frm_clock");
        let min1 = gtk::Label::new(Some(&pad_zero(limit_mins)));
        min1.set_widget_name("lbl_clock1");
        hlblbox1.pack_start(&min1, false, true, 0);
        let sep1  = gtk::Label::new(Some(":"));
        sep1.set_widget_name("lbl_clock1");
        hlblbox1.pack_start(&sep1, false, true, 0);
        let sec1 = gtk::Label::new(Some("00"));
        sec1.set_widget_name("lbl_clock1");
        hlblbox1.pack_start(&sec1, false, true, 0);
        frm1.add(&hlblbox1);

        let empty_label = gtk::Label::new(Some("   "));

        //black clock
        let txt = format!("{}: ", p2time.player);
        let lbl_black = gtk::Label::new(Some(&txt));

        let hlblbox2 = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        hlblbox2.set_widget_name("box_clock2");
        let frm2 = gtk::Frame::new(None);
        frm2.set_widget_name("frm_clock");

        let min2 = gtk::Label::new(Some(&pad_zero(limit_mins)));
        min2.set_widget_name("lbl_clock2");
        hlblbox2.pack_start(&min2, false, true, 0);
        let sep2  = gtk::Label::new(Some(":"));
        sep2.set_widget_name("lbl_clock2");
        hlblbox2.pack_start(&sep2, false, true, 0);
        let sec2 = gtk::Label::new(Some("00"));
        sec2.set_widget_name("lbl_clock2");
        hlblbox2.pack_start(&sec2, false, true, 0);
        frm2.add(&hlblbox2);

        hbox_labels.pack_start(&lbl_white, false, true, 5);
        hbox_labels.pack_start(&frm1, false, true, 5);
        hbox_labels.pack_start(&empty_label, false, true, 5);
        hbox_labels.pack_start(&lbl_black, false, true, 5);
        hbox_labels.pack_start(&frm2, false, true, 5);

        vbox.pack_start(&hbox_labels, false, true, 5);

        let hbox_btns = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        let btn_start = gtk::Button::with_label("Start Game");
        let btn_white = gtk::Button::with_label("Start White");
        let btn_black = gtk::Button::with_label("Start Black");

        hbox_btns.pack_start(&btn_start, true, true, 3);
        hbox_btns.pack_start(&btn_white, true, true, 3);
        hbox_btns.pack_start(&btn_black, true, true, 3);

        vbox.pack_start(&hbox_btns, false, true, 5);

        hbox_clocks.pack_start(&vbox, false, true, 5);

        let t = Timer {
            playing,
            current_player,
            hbox_clocks,
            p1time,
            p2time,
            min1,
            min2,
            sec1,
            sec2,
            sep1,
            sep2,
            btn_white,
            btn_black,
            btn_start,
        };

        unsafe { STATUS = Some(t.clone()); };
        
        //start_timer();
        t
    }


    pub fn do_closures(&mut self) {

        self.btn_start.connect_clicked (move |btn| {
            let mut timer: Timer;
            unsafe {
                timer = STATUS.clone().unwrap();
                timer.current_player = 1;
                timer.playing = true;
                STATUS = Some(timer);
            }
            btn.set_sensitive(false);
            start_timer();
        });

        self.btn_white.connect_clicked (move |_btn| {
            let mut timer: Timer;
            unsafe {
                timer = STATUS.clone().unwrap();
                timer.current_player = 1;
                timer.playing = true;
                STATUS = Some(timer);
            }
        });

        self.btn_black.connect_clicked (move |_btn| {
            let mut timer: Timer;
            unsafe {
                timer = STATUS.clone().unwrap();
                timer.current_player = 2;
                timer.playing = true;
                STATUS = Some(timer);
            }
        });
    }
    
}


fn start_timer() {
    let mut timer: Timer;
    unsafe {
        timer = STATUS.clone().unwrap();
        timer.playing = true;
        STATUS = Some(timer);
    }
    
    let mut p1sec = 60;
    let mut p2sec = 60;
    let timerid = move || {
        //let mut timer = clon.clone();
        let mut timer: Timer;
        unsafe {
            timer = STATUS.clone().unwrap();
        }
        // player 1
        if timer.current_player == 1 {
            if timer.playing {
                // disable btn white / enable black
                timer.btn_white.set_sensitive(false);
                timer.btn_black.set_sensitive(true);
                timer.p1time.minutes = 
                    timer.p1time.get_minutes(timer.min1.clone()).parse::<i32>().unwrap();
                if p1sec == 60 {
                    timer.p1time.minutes = timer.p1time.minutes - 1;
                }
                p1sec = p1sec - 1;
                time_warning(p1sec);
                timer.sec1.set_label(&pad_zero(p1sec));
                timer.min1.set_label(&pad_zero(timer.p1time.minutes));
                if p1sec == 0 {
                    // If minutes and seconds are zero stop timer with the clearInterval method.
                    if p1sec == 0 && timer.p1time.minutes == 0 {
                        // Stop timer.
                        timer.playing = false;
                        timer.btn_white.set_sensitive(false);
                        timer.btn_black.set_sensitive(false);
                    }
                    p1sec = 60;
                }
                timer.sec1.show_all();
            }
        }
        else {
            // disable btn black / enable white
            timer.btn_white.set_sensitive(true);
            timer.btn_black.set_sensitive(false);
            timer.p2time.minutes = 
                    timer.p2time.get_minutes(timer.min2.clone()).parse::<i32>().unwrap();
                if p2sec == 60 {
                    timer.p2time.minutes = timer.p2time.minutes - 1;
                }
                p2sec = p2sec - 1;
                time_warning(p2sec);
                timer.sec2.set_label(&pad_zero(p2sec));
                timer.min2.set_label(&pad_zero(timer.p2time.minutes));
                if p2sec == 0 {
                    // If minutes and seconds are zero stop timer with the clearInterval method.
                    if p2sec == 0 && timer.p2time.minutes == 0 {
                        // Stop timer.
                        timer.playing = false;
                        timer.btn_white.set_sensitive(false);
                        timer.btn_black.set_sensitive(false);
                    }
                    p2sec = 60;
                }
                timer.sec2.show_all();
        }
        unsafe {
            STATUS = Some(timer.clone());
        }
        glib::Continue(if timer.playing {true} else {false})
    };
    
    let one_sec = std::time::Duration::from_millis(1000);
    glib::timeout_add(one_sec, timerid);
}



fn time_warning (p1sec: i32) {
    let timer: Timer;
    unsafe {
        timer = STATUS.clone().unwrap();
    }
    // Change the numbers to red during the last 30 seconds.
    // perhaps when < 5 / 10% of limit time ?
    if timer.p1time.minutes < 1 && p1sec <= 30 {
        if timer.current_player == 1 {
            timer.min1.set_widget_name("lbl_clock1_red");
            timer.sep1.set_widget_name("lbl_clock1_red");
            timer.sec1.set_widget_name("lbl_clock1_red");
        } else {
            timer.min2.set_widget_name("lbl_clock1_red");
            timer.sep2.set_widget_name("lbl_clock1_red");
            timer.sec2.set_widget_name("lbl_clock1_red");
        }
    }
    unsafe {
        STATUS = Some(timer);
    }
}


fn pad_zero (number: i32) -> String {
    if number < 10 {
        return format!("0{}", number);
    }
    number.to_string()
}

