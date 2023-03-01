mod la{
    pub mod transformation;
    pub use transformation::*;
}
use nannou::{prelude::*, event::ElementState};
use nannou_egui::{self, egui, Egui};
use crate::la::transformation::Camera;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model{
    camera: Camera,
    egui: Egui,
}

fn model(app: &App) -> Model {
    // Create window
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
        
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);


    Model {
        egui,
        camera: Camera {
            theta: 210,
            phi: 45,
            a: 1000,
            c: 2000
        }
    }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    // let settings = &mut model.settings;
    let camera = &mut model.camera; 

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Camera_Options").show(&ctx, |ui| {
        // Theta slider
        ui.label("Theta");
        ui.add(egui::Slider::new(&mut camera.theta, 0..=360));

        // Phi slider
        ui.label("Phi");
        ui.add(egui::Slider::new(&mut camera.phi, 1..=360));

        // c slider
        ui.label("C");
        ui.add(egui::Slider::new(&mut camera.c, 1..=2000));

        // a slider
        ui.label("A");
        ui.add(egui::Slider::new(&mut camera.a, camera.c*(-3)..=camera.c));
    });
    
    
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    // Let egui handle things like keyboard and mouse input.
    model.egui.handle_raw_event(event);
    match *event {
        #[allow(unused)]
        #[allow(deprecated)]
        nannou::winit::event::WindowEvent::CursorMoved { device_id, position, modifiers } => {println!("{:?}{:?}",position,modifiers)}
        #[allow(unused)]
        nannou::winit::event::WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {
            if input.virtual_keycode == Some(Key::Left) && model.camera.theta < 359 && input.state == ElementState::Pressed {
                model.camera.theta = model.camera.theta +1;
            } else if input.virtual_keycode == Some(Key::Left) && model.camera.theta == 359 && input.state == ElementState::Pressed {
                model.camera.theta = 1;
            } else if input.virtual_keycode == Some(Key::Right) && model.camera.theta >1 && input.state == ElementState::Pressed{
                model.camera.theta = model.camera.theta -1;
            } else if input.virtual_keycode == Some(Key::Right) && model.camera.theta ==1 && input.state == ElementState::Pressed{
                model.camera.theta = 359;
            } else if input.virtual_keycode == Some(Key::Down) && model.camera.phi > 1 && input.state == ElementState::Pressed{
                model.camera.phi = model.camera.phi -1;
            } else if input.virtual_keycode == Some(Key::Down) && model.camera.phi == 1 && input.state == ElementState::Pressed{
                model.camera.phi = 359;
            } else if input.virtual_keycode == Some(Key::Up) && model.camera.phi < 359 && input.state == ElementState::Pressed{
                model.camera.phi = model.camera.phi +1;
            } else if input.virtual_keycode == Some(Key::Up) && model.camera.phi == 359 && input.state == ElementState::Pressed{
                model.camera.phi = 1;
            }
            println!("{:?}",input);}
        _=> {}
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    // let settings = &model.settings;
    let camera = model.camera;

    let draw = app.draw();
    draw.background().color(BLACK);


    draw.line()
    .start(Camera::proj3t(vec3(0.0, 0.0, -1000.0), camera.clone()))
    .end(Camera::proj3t(vec3(0.0, 0.0, 1000.0), camera.clone()))
    .weight(3.0)
    .color(BLUE);
    draw.line()
    .start(Camera::proj3t(vec3(0.0, -1000.0, 0.0), camera.clone()))
    .end(Camera::proj3t(vec3(0.0, 1000.0, 0.0), camera.clone()))
    .weight(3.0)
    .color(ORANGE);
    draw.line()
    .start(Camera::proj3t(vec3(-1000.0, 0.0, 0.0), camera.clone()))
    .end(Camera::proj3t(vec3(1000.0, 0.0, 0.0), camera.clone()))
    .weight(3.0)
    .color(GREEN);

    let points_basis = (0..4).map(|i|{
        match i {
            0 => Camera::proj3t(vec3(-1000.0, -1000.0, 0.0), camera.clone()),
            1 => Camera::proj3t(vec3(-1000.0, 1000.0, 0.0), camera.clone()),
            2 => Camera::proj3t(vec3(1000.0, 1000.0, 0.0), camera.clone()),
            _ => Camera::proj3t(vec3(1000.0, -1000.0, 0.0), camera.clone())
        }
    });
    draw.polygon()
    .points(points_basis)
    .color(rgba8(40,40,40,120));

    for i in 0..20{
        let points_horizon_start  = Camera::proj3t(vec3(-1000.0 , -1000.0 + 100.0 * i as f32, 0.0), camera.clone());
        let points_horizon_end    = Camera::proj3t(vec3(1000.0  , -1000.0 + 100.0 * i as f32, 0.0), camera.clone());
        let points_vertical_start = Camera::proj3t(vec3(-1000.0 + 100.0 * i as f32, -1000.0 , 0.0), camera.clone());
        let points_vertical_end   = Camera::proj3t(vec3(-1000.0 + 100.0 * i as f32, 1000.0  , 0.0), camera.clone());
        draw.line()
        .start(points_horizon_start)
        .end(points_horizon_end)
        .weight(0.5)
        .color(rgba8(140,140,140,120));
        draw.line()
        .start(points_vertical_start)
        .end(points_vertical_end)
        .weight(0.5)
        .color(rgba8(140,140,140,120));
    }








    //draw sphere
    let pv : Vec3 = vec3(200.0, 200.0, 100.0);
    let radius : f32 = 100.0;
    for j in 0..30{
        let mut dsc_sphere : Vec<f32> =vec![];
        let mut points_spr : Vec<Vec<Vec3>> = vec![];
        for i in 0..60{
            let points_sqr = vec![
                pt3((360.0_f32.to_radians()/60.0*(i as f32     )).cos()*(360.0_f32.to_radians()/60.0*(j as f32     )).sin()*radius +pv.clone().x,  (360.0_f32.to_radians()/60.0*(i as f32     )).sin()*(360.0_f32.to_radians()/60.0*(j as f32     )).sin()*radius +pv.clone().y,   (360.0_f32.to_radians()/60.0*(j as f32     )).cos()*radius +pv.clone().z),
                pt3((360.0_f32.to_radians()/60.0*(i as f32 +1.0)).cos()*(360.0_f32.to_radians()/60.0*(j as f32     )).sin()*radius +pv.clone().x,  (360.0_f32.to_radians()/60.0*(i as f32 +1.0)).sin()*(360.0_f32.to_radians()/60.0*(j as f32     )).sin()*radius +pv.clone().y,   (360.0_f32.to_radians()/60.0*(j as f32     )).cos()*radius +pv.clone().z),
                pt3((360.0_f32.to_radians()/60.0*(i as f32 +1.0)).cos()*(360.0_f32.to_radians()/60.0*(j as f32 +1.0)).sin()*radius +pv.clone().x,  (360.0_f32.to_radians()/60.0*(i as f32 +1.0)).sin()*(360.0_f32.to_radians()/60.0*(j as f32 +1.0)).sin()*radius +pv.clone().y,   (360.0_f32.to_radians()/60.0*(j as f32 +1.0)).cos()*radius +pv.clone().z),
                pt3((360.0_f32.to_radians()/60.0*(i as f32     )).cos()*(360.0_f32.to_radians()/60.0*(j as f32 +1.0)).sin()*radius +pv.clone().x,  (360.0_f32.to_radians()/60.0*(i as f32     )).sin()*(360.0_f32.to_radians()/60.0*(j as f32 +1.0)).sin()*radius +pv.clone().y,   (360.0_f32.to_radians()/60.0*(j as f32 +1.0)).cos()*radius +pv.clone().z)
            ];
            let cam : Vec3 = pt3((camera.clone().theta as f32).to_radians().cos()*(camera.clone().phi as f32).to_radians().sin(), (camera.clone().theta as f32).to_radians().sin()*(camera.clone().phi as f32).to_radians().sin(), (camera.clone().phi as f32).to_radians().cos())*camera.clone().c as f32;
            let mut distance = 0.0;
            for int_i in 0..4{
                distance += (points_sqr.clone()[int_i] - cam).length().powf(2.0)/4.0;
            }
            points_spr.push(points_sqr.clone());
            dsc_sphere.push(distance.sqrt());
        }
        let mut dst_sphere : Vec<usize>= vec![];
        for i in 0..60{
            dst_sphere.push(0);
            for k in 0..60{
                if dsc_sphere[i] >= dsc_sphere[k]{
                    dst_sphere[i] += 1;
                }
            }
        }
        for s in (1..=60).rev(){
            for i in 0..60{
                if s == dst_sphere[i] {
                    let points_sphere = (0..4).map(|k|{
                        Camera::proj3t(points_spr.clone()[i][k], camera.clone()) 
                    });
                    draw
                    .polygon()
                    .points(points_sphere.clone())
                    .color(rgba8(10+(j as isize -15_isize).abs() as u8*5,48+(j as isize -15_isize).abs() as u8*5,18+(j as isize -15_isize).abs() as u8*5,255));
                    draw
                    .polyline()
                    .weight(0.2)
                    .points(points_sphere.clone())
                    .color(rgba8(0,100,200,180));
                }
            }
        }
    }













    //draw cube
    let cb: Vec3 = pt3(300.0, -200.0, -400.0);
    let x_cb: Vec3 = pt3(200.0, 0.0, 0.0);
    let y_cb: Vec3 = pt3(0.0, 200.0, 0.0);
    let z_cb: Vec3 = pt3(0.0, 0.0, 200.0);
    let color_cb : (u8,u8,u8)= (220,220,220);
    let points_cb = vec![
        vec![cb ,cb + x_cb,cb + x_cb + y_cb,cb + y_cb],
        vec![cb ,cb + z_cb,cb + z_cb + y_cb,cb + y_cb],
        vec![cb ,cb + x_cb,cb + x_cb + z_cb,cb + z_cb],

        vec![cb + z_cb,cb + x_cb + z_cb,cb + x_cb + y_cb + z_cb,cb + y_cb + z_cb],
        vec![cb + x_cb,cb + z_cb + x_cb,cb + z_cb + y_cb + x_cb,cb + y_cb + x_cb],
        vec![cb + y_cb,cb + x_cb + y_cb,cb + x_cb + z_cb + y_cb,cb + z_cb + y_cb],
        ] ;
    let mut dsc_cube : Vec<f32> = vec![];
    for i in 0..6{
        let mut distance : f32 = 0.0;
        let cam : Vec3 = pt3((camera.clone().theta as f32).to_radians().cos()*(camera.clone().phi as f32).to_radians().sin(), (camera.clone().theta as f32).to_radians().sin()*(camera.clone().phi as f32).to_radians().sin(), (camera.clone().phi as f32).to_radians().cos())*camera.clone().c as f32;
        for j in 0..points_cb[i].len(){
            distance +=  (points_cb.clone()[i][j] -cam).length().powf(2.0)/4.0;
        }
        dsc_cube.push(distance.sqrt()) 
    }
    println!("{:?}",dsc_cube.clone());
    let mut dst_cube : Vec<u8>= vec![];
    for int_i in 0..6{
        dst_cube.push(0);
        for int_j in 0..6{
            if dsc_cube[int_i] >= dsc_cube[int_j] {
                dst_cube[int_i] +=1;
            }
        }
    }
    println!("{:?}",dst_cube.clone());

    for k in (1..7).rev(){
        for i in 0..6{
            if k == dst_cube[i] {
                let points_cube = (0..4).map(|j|{
                    (Camera::proj3t(points_cb.clone()[i][j], camera.clone()) , rgba(color_cb.0 - dst_cube[i]*30,color_cb.1- dst_cube[i]*30,color_cb.2- dst_cube[i]*30,255))
                });
                draw.polygon()
                .points_colored(points_cube.clone());
            }
        }
    }


    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
