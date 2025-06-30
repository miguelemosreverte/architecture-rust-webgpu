use wgpu::{Device, Buffer, BufferUsages};
use wgpu::util::DeviceExt;
use bytemuck::NoUninit;

pub struct VertexBuffer<T: NoUninit> {
    pub buffer: Buffer,
    pub count: u32,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: NoUninit> VertexBuffer<T> {
    pub fn new(device: &Device, data: &[T]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(data),
            usage: BufferUsages::VERTEX,
        });

        Self {
            buffer,
            count: data.len() as u32,
            _phantom: std::marker::PhantomData,
        }
    }
}

pub struct IndexBuffer {
    pub buffer: Buffer,
    pub count: u32,
}

impl IndexBuffer {
    pub fn new(device: &Device, indices: &[u16]) -> Self {
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: BufferUsages::INDEX,
        });

        Self {
            buffer,
            count: indices.len() as u32,
        }
    }
}