//#![windows_subsystem = "windows"]
    
extern crate runge_kutta;
use runge_kutta::*;

extern crate piston_window;
extern crate image;
use piston_window::*;
use image::*;

fn lorenz(dqdt: &mut [FP], q: &[FP]) {
    const PAR_P: FP = 10.0;
    const PAR_R: FP = 28.0;
    const PAR_B: FP = 8.0 / 3.0;
    // 0:t, 1:x, 2:y, 3:z
    dqdt[0] = 1.0;
    dqdt[1] = -PAR_P * q[1] + PAR_P * q[2];
    dqdt[2] = -q[1] * q[3] + PAR_R * q[1] - q[2];
    dqdt[3] = q[1] * q[2] - PAR_B * q[3];
}

fn lorenz_draw(canvas: &mut ImageBuffer<Rgba<u8>, std::vec::Vec<u8>>, q: &[FP]) {
    const SCALE: FP = 6.0;
    let (width, height) = (640, 480);

    let x = ((width/2) as FP) + q[1] * SCALE;
    let y = ((height*3/4) as FP) - q[3] * SCALE;
    let c = 128.0 - q[2] * 10.0;
    let g = q[0];

    let x = x as u32;
    let y = y as u32;
    let c = if c < 0.0 {0.0} else {if c > 255.0 {255.0} else {c}} as u8;
    let g = g as u8;

    canvas.put_pixel(x, y, Rgba([255 - c, g, c, 255]));
}

fn main() {
    let (width, height) = (640, 480);

    let mut window: PistonWindow = WindowSettings::new(
            "Window",
            [width, height]
        ).exit_on_esc(true).build().unwrap();
    let mut canvas = ImageBuffer::new(width, height);
    let mut texture: G2dTexture = Texture::from_image(
            &mut window.factory,
            &canvas,
            &TextureSettings::new()
        ).unwrap();

    let mut sim = RungeKutta::new(4);
    sim.init_val(|q| {q[1] = 1.0});
    println!("{:?}", sim);

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&event, |c, g| {
                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });

            for _ in 0..1024 {
                sim.step(1.0 / 1024.0,
                         |dqdt, q| lorenz(dqdt, q));
                lorenz_draw(&mut canvas, sim.get_val());
                //println!("{:?}", sim.get_val());
            }
        }
    }
}
