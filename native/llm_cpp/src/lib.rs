use llama_cpp::{LlamaModel, LlamaParams, SessionParams};
use llama_cpp::standard_sampler::StandardSampler;
use rustler::{NifStruct, Resource, ResourceArc};
use std::sync::Mutex;

pub struct ModelRef(pub LlamaModel);
impl Resource for ModelRef {}

pub struct SessionRef(pub Mutex<llama_cpp::LlamaSession>);
impl Resource for SessionRef {}

#[derive(NifStruct)]
#[module = "LlamaBinding.Model"]
pub struct Model {
    pub resource: ResourceArc<ModelRef>,
}

impl Model {
    pub fn new(model: LlamaModel) -> Self {
        Self {
            resource: ResourceArc::new(ModelRef(model)),
        }
    }
}

#[derive(NifStruct)]
#[module = "LlamaBinding.Session"]
pub struct Session {
    pub resource: ResourceArc<SessionRef>,
}

impl Session {
    pub fn new(session: llama_cpp::LlamaSession) -> Self {
        Self {
            resource: ResourceArc::new(SessionRef(Mutex::new(session))),
        }
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn load_model(path: String) -> Result<Model, String> {
    let model = LlamaModel::load_from_file(path, LlamaParams::default())
        .map_err(|e| format!("Failed to load model: {:?}", e))?;
    Ok(Model::new(model))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn create_session(model: Model) -> Result<Session, String> {
    let session = model
        .resource
        .0
        .create_session(SessionParams::default())
        .map_err(|e| e.to_string())?;
    Ok(Session::new(session))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn set_context(session: Session, prompt: String) -> Result<(), String> {
    let mut ctx = session.resource.0.lock().map_err(|e| e.to_string())?;
    ctx.advance_context(&prompt).map_err(|e| e.to_string())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn complete(session: Session, max_tokens: usize) -> Result<String, String> {
    let mut ctx = session.resource.0.lock().map_err(|e| e.to_string())?;
    let tokens = ctx
        .start_completing_with(StandardSampler::default(), max_tokens)
        .map_err(|e| e.to_string())? // Handle the Result from start_completing_with
        .into_strings();

    let output: Vec<String> = tokens.collect();
    Ok(output.join(" "))
}

rustler::init!("Elixir.LlamaBinding", [load_model, create_session, set_context, complete]);
