use crate::render_backend::mesh::Mesh;
use crate::render_backend::instance::InstanceBuffer;

pub struct SceneObject {
    mesh: Mesh,
    instance_buffer: InstanceBuffer,
}

impl SceneObject {
    pub fn new(
        mesh: Mesh,
        instance_buffer: InstanceBuffer,
    ) -> Self {
        Self {
            mesh,
            instance_buffer,
        }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn instance_buffer(&self) -> &InstanceBuffer {
        &self.instance_buffer
    }

    pub fn instance_buffer_mut(&mut self) -> &mut InstanceBuffer {
        &mut self.instance_buffer
    }
}

pub struct Scene {
    objects: Vec<SceneObject>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: Mesh) {
        self.objects.push(object);
    }

    pub fn objects(&self) -> &[SceneObject] {
        &self.objects
    }

    pub fn objects_mut(&mut self) -> &mut [SceneObject] {
        &mut self.objects
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}