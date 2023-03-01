use nannou::glam::Vec2;
use nannou::geom::*;

#[allow(unused)]
#[derive(Debug, Clone , Copy)]
pub struct Camera{
    pub theta: usize,     //camera's theta(θ) in 'spherical coordinate system'
    pub phi: usize,       //camera's phi(φ) in 'spherical coordinate system'
    pub a: isize,         //distance between projective plane and origin
    pub c: isize          //camera's radius in 'spherical coordinate system'(distance between origin and camera)
}

impl Camera {
    #[allow(unused)]
    pub fn proj3t(op:Vec3, camera: Camera) -> Vec2{
        if (camera.phi as f32).to_radians().tan().abs()*(camera.phi as f32).to_radians().cos() != 0.0{

            let n: Vec3 = vec3((camera.theta as f32).to_radians().cos()*(camera.phi as f32).to_radians().sin(), (camera.theta as f32).to_radians().sin()*(camera.phi as f32).to_radians().sin(), (camera.phi as f32).to_radians().cos());

            let ob : Vec3 = vec3(
                op.clone().x*(camera.c as f32 - camera.a as f32)/(camera.c as f32 - op.clone().dot(n.clone())) + n.clone().x*(camera.c as f32)*(camera.a as f32 - op.clone().dot(n.clone()))/(camera.c as f32 - op.clone().dot(n.clone())),
                op.clone().y*(camera.c as f32 - camera.a as f32)/(camera.c as f32 - op.clone().dot(n.clone())) + n.clone().y*(camera.c as f32)*(camera.a as f32 - op.clone().dot(n.clone()))/(camera.c as f32 - op.clone().dot(n.clone())),
                op.clone().z*(camera.c as f32 - camera.a as f32)/(camera.c as f32 - op.clone().dot(n.clone())) + n.clone().z*(camera.c as f32)*(camera.a as f32 - op.clone().dot(n.clone()))/(camera.c as f32 - op.clone().dot(n.clone()))
            );
            //let ob: Vec3 = op.clone()*((camera.c as f32) - op.clone().dot(n.clone())) + n.clone()*(camera.c as f32)*(op.clone().dot(n.clone())-(camera.a as f32));

            let ab: Vec3 = vec3(
                ob.clone().x - n.clone().x*(camera.a as f32), 
                ob.clone().y - n.clone().y*(camera.a as f32), 
                ob.clone().z - n.clone().z*(camera.a as f32)
            );
            // let ab: Vec3 = ob.clone() - n.clone()*(camera.a as f32);

            let mut v: Vec3 = vec3(0.0, 0.0, 0.0);

                if camera.phi < 90 || camera.phi >= 270 {
                    v= vec3(
                        ((camera.theta as f32).to_radians().cos()*(camera.phi as f32).to_radians().sin()*(-1.0))/(camera.phi as f32).to_radians().tan().abs(),
                        ((camera.theta as f32).to_radians().sin()*(camera.phi as f32).to_radians().sin()*(-1.0))/(camera.phi as f32).to_radians().tan().abs(),
                        (camera.phi as f32).to_radians().sin().powf(2.0)/((camera.phi as f32).to_radians().tan().abs()*(camera.phi as f32).to_radians().cos())
                    );
                } else {
                    v=vec3(
                        ((camera.theta as f32).to_radians().cos()*(camera.phi as f32).to_radians().sin()*(-1.0))/(camera.phi as f32).to_radians().tan().abs()*(-1.0),
                        ((camera.theta as f32).to_radians().sin()*(camera.phi as f32).to_radians().sin()*(-1.0))/(camera.phi as f32).to_radians().tan().abs()*(-1.0),
                        (camera.phi as f32).to_radians().sin().powf(2.0)/((camera.phi as f32).to_radians().tan().abs()*(camera.phi as f32).to_radians().cos()*(-1.0))
                    );
                }

            if ab.clone().cross(v.clone()).z >= 0.0 {
                if camera.phi < 90 || camera.phi >= 270{
                    if camera.phi < 180 {
                        vec2(ab.clone().cross(v.clone()).length(), ab.clone().dot(v.clone()))
                    } else {
                        vec2(ab.clone().cross(v.clone()).length()*(-1.0), ab.clone().dot(v.clone())*(-1.0))
                    }
                } else{
                    if camera.phi < 180 {
                        vec2(ab.clone().cross(v.clone()).length()*(-1.0), ab.clone().dot(v.clone()))
                    } else {
                        vec2(ab.clone().cross(v.clone()).length(), ab.clone().dot(v.clone())*(-1.0))
                    }
                }
            } else {
                if camera.phi < 90 || camera.phi >= 270{
                    if camera.phi < 180 {
                        vec2(ab.clone().cross(v.clone()).length()*(-1.0), ab.clone().dot(v.clone()))
                    } else {
                        vec2(ab.clone().cross(v.clone()).length(), ab.clone().dot(v.clone())*(-1.0))
                    }
                    
                } else{
                    if camera.phi < 180 {
                        vec2(ab.clone().cross(v.clone()).length(), ab.clone().dot(v.clone()))
                    } else {
                        vec2(ab.clone().cross(v.clone()).length()*(-1.0), ab.clone().dot(v.clone())*(-1.0))
                    }
                    
                }
            }
            
        } else {
            vec2(op.clone().x, op.clone().y)
            
        }
    }
}

