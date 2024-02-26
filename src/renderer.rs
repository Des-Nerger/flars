use {
	crate::{
		settings::{SCREEN_HEIGHT, SCREEN_WIDTH},
		utils::default,
	},
	ary::ary,
	glam::{IVec2, Mat4, Vec2},
	glium::{
		draw_parameters::{Blend, DrawParameters},
		implement_vertex,
		index::PrimitiveType::TrianglesList,
		program::Program,
		texture::{RawImage2d, Texture2d},
		uniform,
		uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior, SamplerWrapFunction},
		IndexBuffer, Surface, VertexBuffer,
	},
	glium_sdl2::SDL2Facade,
	std::path::Path,
};

pub struct Renderer {
	pub display: SDL2Facade,
	glProgram: Program,
	wholeScreen_vertices: VertexBuffer<Vertex>,
	wholeScreen_indices: IndexBuffer<Index>,
	projection: [[f32; 4]; 4],
}

impl Renderer {
	pub fn new(display: SDL2Facade) -> Self {
		let glProgram = Program::from_source(
			&display,
			r"
				#version 100

				uniform mat4 u_projection;
				attribute vec2 a_position;
				attribute vec4 a_color;
				attribute vec2 a_texCoord;
				varying vec2 v_texCoord;
				varying vec4 v_color;

				void main() {
					v_texCoord = a_texCoord;
					gl_Position = u_projection * vec4(a_position, 0.0, 1.0);
					v_color = a_color;
				}
			",
			r"
				#version 100
				precision mediump float;

				uniform sampler2D u_texture0;
				uniform sampler2D u_texture1;
				varying vec4 v_color;
				varying vec2 v_texCoord;
				void main() {
					gl_FragColor = v_color * ((int(v_texCoord.y) == 0)?
					                           texture2D(u_texture0, vec2(v_texCoord.x, fract(v_texCoord.y))):
					                           texture2D(u_texture1, vec2(v_texCoord.x, fract(v_texCoord.y))));
				}
			",
			None,
		)
		.unwrap();
		let wholeScreen_vertices = VertexBuffer::new(
			&display,
			&[
				Vertex { a_position: [0., SCREEN_HEIGHT as _], a_color: WHITE, a_texCoord: [0., 0.] },
				Vertex { a_position: [SCREEN_WIDTH as _, SCREEN_HEIGHT as _], a_color: WHITE, a_texCoord: [1., 0.] },
				Vertex { a_position: [0., 0.], a_color: WHITE, a_texCoord: [0., 1.] },
				Vertex { a_position: [SCREEN_WIDTH as _, 0.], a_color: WHITE, a_texCoord: [1., 1.] },
			],
		)
		.unwrap();
		let wholeScreen_indices = IndexBuffer::new(&display, TrianglesList, &[0, 1, 2, 2, 1, 3]).unwrap();
		Self {
			display,
			glProgram,
			wholeScreen_vertices,
			wholeScreen_indices,
			projection: Mat4::orthographic_rh_gl(0., SCREEN_WIDTH as _, 0., SCREEN_HEIGHT as _, 1., -1.)
				.to_cols_array_2d(),
		}
	}

	pub fn copy_wholeScreen(&self, surface: &mut impl Surface, texture: &Texture2d) {
		let o = self;
		o.geometryBuffers(surface, &[texture], &o.wholeScreen_vertices, &o.wholeScreen_indices);
	}

	pub fn geometry(
		&self,
		surface: &mut impl Surface,
		textures: &[&Texture2d],
		vertices: &[Vertex],
		indices: &[Index],
	) {
		let o = self;
		o.geometryBuffers(
			surface,
			textures,
			&VertexBuffer::new(&o.display, vertices).unwrap(),
			&IndexBuffer::new(&o.display, TrianglesList, indices).unwrap(),
		);
	}

	fn geometryBuffers(
		&self,
		surface: &mut impl Surface,
		textures: &[&Texture2d],
		vertices: &VertexBuffer<Vertex>,
		indices: &IndexBuffer<Index>,
	) {
		let o = self;
		const SAMPLER_BEHAVIOR: SamplerBehavior = SamplerBehavior {
			wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Mirror),
			minify_filter: MinifySamplerFilter::Nearest,
			magnify_filter: MagnifySamplerFilter::Nearest,
			depth_texture_comparison: None,
			max_anisotropy: 1,
		};
		surface
			.draw(
				vertices,
				indices,
				&o.glProgram,
				&uniform! {
					u_projection: o.projection,
					u_texture0: Sampler(textures[0], SAMPLER_BEHAVIOR),
					u_texture1: Sampler(textures[textures.len() - 1], SAMPLER_BEHAVIOR),
				},
				&DrawParameters { blend: Blend::alpha_blending(), ..default() },
			)
			.unwrap();
	}

	pub fn loadTexture2d(&self, filePath: impl AsRef<Path>) -> Texture2d {
		let image = image::io::Reader::open(filePath).unwrap().decode().unwrap().into_rgba8();
		let imageDimensions = image.dimensions();
		let image = RawImage2d::from_raw_rgba(image.into_raw(), imageDimensions);
		Texture2d::new(&self.display, image).unwrap()
	}
}

#[derive(Copy, Clone)]
pub struct Vertex {
	a_position: [f32; 2],
	a_color: Color,
	a_texCoord: [f32; 2],
}
impl Vertex {
	pub fn new(a_position: IVec2, a_color: Color, a_texCoord: Vec2) -> Self {
		Self { a_position: a_position.as_vec2().to_array(), a_color, a_texCoord: a_texCoord.to_array() }
	}
}
implement_vertex!(Vertex, a_position, a_color, a_texCoord);

pub type Color = [f32; 4];
pub const WHITE: Color = [1., 1., 1., 1.];
pub const ALMOST_BLACK: Color = {
	ary![=>
		0..=2: [(0xC as f32) / 255.; _],
		3: 1.,
	]
};

pub type Index = u32;
