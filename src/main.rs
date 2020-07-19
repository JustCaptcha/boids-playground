use three;
use three::Object;

mod boid;

const BACKGROUND_C: u32 = 0xF0E0B6;

fn main() {
    // add window
    let mut win = three::Window::new("rboids - not a flock more");
    win.scene.background = three::Background::Color(BACKGROUND_C);

    // add camera
    let cam = win.factory.perspective_camera(60.0, 1.0..1000.0);
    cam.look_at([5.0, 5.0, 5.0], [0.0, 0.0, 0.0], None);
    win.scene.add(&cam);

    // add origin
    let origin = {
        let geometry = three::Geometry::uv_sphere(2.0, 12, 12);
        let material = three::material::Wireframe {
            color: three::color::GREEN,
        };
        win.factory.mesh(geometry, material)
    };

    let mut controls = three::controls::Orbit::builder(&cam)
        .position([10.0, 10.0, 10.0])
        .up([0.0, 1.0, 0.0])
        .build();

    let x_edge = {
        let geometry = three::Geometry::uv_sphere(0.2, 12, 12);
        let material = three::material::Wireframe {
            color: three::color::RED,
        };
        win.factory.mesh(geometry, material)
    };

    let y_edge = {
        let geometry = three::Geometry::uv_sphere(0.2, 12, 12);
        let material = three::material::Wireframe {
            color: three::color::GREEN,
        };
        win.factory.mesh(geometry, material)
    };

    let z_edge = {
        let geometry = three::Geometry::uv_sphere(0.2, 12, 12);
        let material = three::material::Wireframe {
            color: three::color::BLUE,
        };
        win.factory.mesh(geometry, material)
    };

    let cone = {
        let geometry = three::Geometry::cylinder(0.0, 1.0, 1.5, 12);
        let material = three::material::Basic {
            color: three::color::BLACK,
            map: None,
        };
        win.factory.mesh(geometry, material)
    };

    let blind = {
        let geometry = three::Geometry::cylinder(0.0, 1.0, 1.5, 12);
        let material = three::material::Wireframe {
            color: three::color::RED,
        };
        win.factory.mesh(geometry, material)
    };

    x_edge.set_position([5.0, 0.0, 0.0]);
    win.scene.add(&x_edge);

    y_edge.set_position([0.0, 5.0, 0.0]);
    win.scene.add(&y_edge);

    z_edge.set_position([0.0, 0.0, 5.0]);
    win.scene.add(&z_edge);

    origin.set_position([0.0, 0.0, 0.0]);
    win.scene.add(&origin);

    cone.set_position([0.0, 0.0, 0.0]);
    win.scene.add(&cone);

    blind.set_position([0.0, -1.0, 0.0]);
    win.scene.add(&blind);

    // create boid
    let mut boxy: boid::Boid = boid::Boid::new(vec![0.0, 0.0, 0.0], vec![0.5, 1.2, 0.0]);

    // start scene
    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        controls.update(&win.input);

        // compute new boxy velocity and set it
        boxy.frame_update(win.input.delta_time());
        cone.set_transform(boxy.pos_array(), boxy.rot_array(), 1.0);

        // render scene
        win.render(&cam);
    }
}
