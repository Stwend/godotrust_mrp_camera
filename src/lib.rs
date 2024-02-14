#![allow(unused)]

use godot::engine::Engine;
use godot::engine::notify::Node3DNotification;
use godot::prelude::*;


struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {
    fn on_level_init(_level: InitLevel) {
        if _level == InitLevel::Scene {
            Engine::singleton().register_singleton("GlobalCameraManager".into(), GlobalCameraManagerBase::new_alloc().upcast());
        }
    }

    fn on_level_deinit(_level: InitLevel) {
        if _level != InitLevel::Scene {return;}

        if let Some(camera_manager_singleton) = Engine::singleton().get_singleton("GlobalCameraManager".into()) {
            Engine::singleton().unregister_singleton("GlobalCameraManager".into());
            camera_manager_singleton.free();
        }
    }
}


#[derive(GodotClass)]
#[class(base=Node)]
pub struct GlobalCameraManagerBase {
    camera: Gd<Camera3D>,
    base: Base<Node>,
}

#[godot_api]
impl INode for GlobalCameraManagerBase {
    fn init(base: Base<Self::Base>) -> Self {
        Self{
            camera: Camera3D::new_alloc(),
            base
        }
    }
}

#[godot_api]
impl GlobalCameraManagerBase {
    pub fn get_instance() -> Option<Gd<GlobalCameraManagerBase>> {
        if let Some(obj) = Engine::singleton().get_singleton("GlobalCameraManager".into()) {
            return Some(obj.cast::<GlobalCameraManagerBase>());
        }
        None
    }

    fn register_socket(&mut self, socket: Gd<CameraSocket>) -> Option<Gd<Camera3D>> {
        return Some(self.camera.clone());
    }
}



#[derive(GodotClass)]
#[class(base=Node3D, init)]
pub struct CameraSocket {
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for CameraSocket {
    fn on_notification(&mut self, what: Node3DNotification) {
        match what {
            Node3DNotification::Ready => {
                self.base_mut().call_deferred("_register".into(), &[]);
            },
            _ => {},
        }
    }
}

#[godot_api]
impl CameraSocket {
    #[func]
    pub fn _register(&mut self) {
        if let Some(mut cam_manager) = GlobalCameraManagerBase::get_instance() {
            let mut bound_manager = cam_manager.bind_mut();
            if let Some(mut camera) = bound_manager.register_socket(self.to_gd()) {
                if camera.is_instance_valid() {
                    if camera.get_parent().is_some() {
                        camera.reparent(self.base_mut().clone().upcast());
                    } else {
                        self.base_mut().add_child(camera.upcast());
                        godot_print!("Hooray! Camera added to socket.");
                    }
                } else {
                    godot_print!("No instance existing for {}", camera);
                }
            }
        }
    }
}
