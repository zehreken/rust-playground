use cgmath;
use ggez;
use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

pub fn run() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("ggez_test", "zehreken")
        .build()
        .expect("Could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut game = MainState::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MainState {
    frames: usize,
    text: graphics::Text,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> MainState {
        let font = graphics::Font::new(ctx, "/emulogic.ttf").unwrap();
        let text = graphics::Text::new(("Hello World!", font, 18.0));

        MainState { frames: 0, text }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let offset = self.frames as f32 / 10.0;
        let position = ggez::mint::Point2 {
            x: offset,
            y: offset,
        };
        graphics::draw(ctx, &self.text, (position,)).unwrap();

        self.frames += 1;
        if self.frames % 100 == 0 {}

        graphics::present(ctx)
    }
}
