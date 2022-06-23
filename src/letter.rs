pub struct LColumn {
    letter: char
}

impl Default for LColumn {
    fn default() -> Self {
        Self {
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

 
}

impl Widget for LColumn {
    fn ui(self, ui: &mut Ui) -> Response {
        let LColumn {
            letter,
        } = self;

        // let available_space = ui.available_size_before_wrap();
        let available_space = ui.max_rect().size();
      
        println!("{:?}",ui.max_rect().size().x);

        
        // let size = vec2(available_space.x/20.0, available_space.x/20.0);
        let size = (available_space.x-6.0*27.0-6.0*2.0)/27.0;
        let (rect, response) = ui.allocate_at_least(vec2(size,size), Sense::hover());
        // let x: f32 = available_space.x/20.0;
        // let y: f32 = available_space.x/20.0;
        // let x2: f32 = &x+20.0;
        // let y2: f32 = &y+20.0;
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