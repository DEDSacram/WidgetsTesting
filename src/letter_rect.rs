use egui::Align2;
use egui::FontId;
use egui::Stroke;
use egui::Color32;
use egui::Pos2;
use egui::Rect;
use egui::vec2;
use egui::Sense;
use egui::Response;
use egui::Ui;
use egui::Widget;


/// A visual LColumn. A horizontal or vertical line (depending on [`Layout`]).
///
/// Usually you'd use the shorter version [`Ui::LColumn`].
///
/// ```
/// # egui::__run_test_ui(|ui| {
/// // These are equivalent:
/// ui.LColumn();
/// ui.add(egui::LColumn::default());
/// # });
/// ```
#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]

pub struct Alphabet {
    letters : Vec<LColumn>
}
impl Default for Alphabet {
    fn default() -> Self {
        Self {
            letters: Vec::new()
        }
    }
}
impl Alphabet{
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget for Alphabet{
    fn ui(self, ui: &mut Ui) -> Response {
        let Alphabet {
            mut letters
        } = self;
        let available_space = ui.available_size_before_wrap();


        let boxsize = available_space.x/26.0;
        for n in 'A'..='Z' {
            letters.push(LColumn::new().letter(n).rectsize(boxsize));
        }
   
        let (rect, response) = ui.allocate_at_least(vec2(available_space.x,boxsize), Sense::hover());
        if ui.is_rect_visible(response.rect) {
         
        }
        for elem in letters {
            ui.add(elem);
        }

        response
    }
}






pub struct LColumn {
    size : f32,
    letter: char
}

impl Default for LColumn {
    fn default() -> Self {
        Self {
            size : 20.0,
            letter: 'X',
        }
    }
}

impl LColumn {
    /// How much space we take up. The line is painted in the middle of this.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn letter(mut self, letter: char) -> Self {
        self.letter = letter;
        self
    }

    pub fn rectsize(mut self, size : f32) -> Self {
        self.size = size;
        self

    }

 
}

impl Widget for LColumn {
    fn ui(self, ui: &mut Ui) -> Response {
        let LColumn {
            size,
            letter
        } = self;


        let (rect, response) = ui.allocate_at_least(vec2(size,size), Sense::hover());
        let corner_radius : f32 = 1.0;
        let stroke_width : f32 = 1.5;
        let stroke = Stroke::new( stroke_width, Color32::DARK_GRAY);
        if ui.is_rect_visible(response.rect) {
    
            // ui.painter().hline(rect.x_range(), rect.center().y, stroke);
            ui.painter().rect(rect, corner_radius, Color32::TRANSPARENT, stroke);
            ui.painter().text(rect.center(), Align2::CENTER_CENTER, letter, FontId::proportional(size/2.0), Color32::BLACK);
            // ui.painter().rect(Rect::from_min_max(Pos2::from([x,y]),Pos2::from([x2,y2])), corner_radius, Color32::BLACK, stroke)
        }

        response
    }
}