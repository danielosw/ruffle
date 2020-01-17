//! Management of async loaders

use crate::avm1::{Object, TObject, Value};
use crate::context::{ActionQueue, ActionType};
use crate::display_object::{DisplayObject, MorphShape, TDisplayObject};
use crate::player::{Player, NEWEST_PLAYER_VERSION};
use crate::tag_utils::SwfMovie;
use gc_arena::{Collect, CollectionContext};
use generational_arena::{Arena, Index};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use url::form_urlencoded;

pub type Handle = Index;

type Error = Box<dyn std::error::Error>;

/// Holds all in-progress loads for the player.
pub struct LoadManager<'gc>(Arena<Loader<'gc>>);

unsafe impl<'gc> Collect for LoadManager<'gc> {
    fn trace(&self, cc: CollectionContext) {
        for (_, loader) in self.0.iter() {
            loader.trace(cc)
        }
    }
}

impl<'gc> LoadManager<'gc> {
    /// Construct a new `LoadManager`.
    pub fn new() -> Self {
        Self(Arena::new())
    }

    /// Add a new loader to the `LoadManager`.
    ///
    /// This function returns the loader handle for later inspection. A loader
    /// handle is valid for as long as the load operation. Once the load
    /// finishes, the handle will be invalidated (and the underlying loader
    /// deleted).
    pub fn add_loader(&mut self, loader: Loader<'gc>) -> Handle {
        let handle = self.0.insert(loader);
        self.0
            .get_mut(handle)
            .unwrap()
            .introduce_loader_handle(handle);

        handle
    }

    /// Retrieve a loader by handle.
    pub fn get_loader(&self, handle: Handle) -> Option<&Loader<'gc>> {
        self.0.get(handle)
    }

    /// Retrieve a loader by handle for mutation.
    pub fn get_loader_mut(&mut self, handle: Handle) -> Option<&mut Loader<'gc>> {
        self.0.get_mut(handle)
    }

    /// Kick off a movie clip load.
    ///
    /// Returns the loader's async process, which you will need to spawn.
    pub fn load_movie_into_clip(
        &mut self,
        player: Weak<Mutex<Player>>,
        target_clip: DisplayObject<'gc>,
        fetch: Pin<Box<dyn Future<Output = Result<Vec<u8>, Error>>>>,
        target_broadcaster: Option<Object<'gc>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>> {
        let loader = Loader::Movie {
            self_handle: None,
            target_clip,
            target_broadcaster,
        };
        let handle = self.add_loader(loader);

        let loader = self.get_loader_mut(handle).unwrap();
        loader.introduce_loader_handle(handle);

        loader.movie_loader(player, fetch)
    }

    /// Indicates that a movie clip has initialized (ran it's first frame).
    ///
    /// Interested loaders will be invoked from here.
    pub fn movie_clip_on_load(
        &mut self,
        loaded_clip: DisplayObject<'gc>,
        clip_object: Option<Object<'gc>>,
        root: DisplayObject<'gc>,
        queue: &mut ActionQueue<'gc>,
    ) {
        let mut invalidated_loaders = vec![];

        for (index, loader) in self.0.iter_mut() {
            if loader.movie_clip_loaded(loaded_clip, clip_object, root, queue) {
                invalidated_loaders.push(index);
            }
        }

        for index in invalidated_loaders {
            self.0.remove(index);
        }
    }

    /// Kick off a form data load into an AVM1 object.
    ///
    /// Returns the loader's async process, which you will need to spawn.
    pub fn load_form_into_object(
        &mut self,
        player: Weak<Mutex<Player>>,
        target_object: Object<'gc>,
        fetch: Pin<Box<dyn Future<Output = Result<Vec<u8>, Error>>>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>> {
        let loader = Loader::Form {
            self_handle: None,
            target_object,
        };
        let handle = self.add_loader(loader);

        let loader = self.get_loader_mut(handle).unwrap();
        loader.introduce_loader_handle(handle);

        loader.form_loader(player, fetch)
    }
}

impl<'gc> Default for LoadManager<'gc> {
    fn default() -> Self {
        Self::new()
    }
}

/// A struct that holds garbage-collected pointers for asynchronous code.
pub enum Loader<'gc> {
    /// Loader that is loading a new movie into a movieclip.
    Movie {
        /// The handle to refer to this loader instance.
        self_handle: Option<Handle>,

        /// The target movie clip to load the movie into.
        target_clip: DisplayObject<'gc>,

        /// Event broadcaster (typically a `MovieClipLoader`) to fire events
        /// into.
        target_broadcaster: Option<Object<'gc>>,
    },

    /// Loader that is loading form data into an AVM1 object scope.
    Form {
        /// The handle to refer to this loader instance.
        self_handle: Option<Handle>,

        /// The target AVM1 object to load form data into.
        target_object: Object<'gc>,
    },
}

unsafe impl<'gc> Collect for Loader<'gc> {
    fn trace(&self, cc: CollectionContext) {
        match self {
            Loader::Movie { target_clip, .. } => target_clip.trace(cc),
            Loader::Form { target_object, .. } => target_object.trace(cc),
        }
    }
}

impl<'gc> Loader<'gc> {
    /// Set the loader handle for this loader.
    ///
    /// An active loader handle is required before asynchronous loader code can
    /// run.
    pub fn introduce_loader_handle(&mut self, handle: Handle) {
        match self {
            Loader::Movie { self_handle, .. } => *self_handle = Some(handle),
            Loader::Form { self_handle, .. } => *self_handle = Some(handle),
        }
    }

    /// Construct a future for the given movie loader.
    ///
    /// The given future should be passed immediately to an executor; it will
    /// take responsibility for running the loader to completion.
    ///
    /// If the loader is not a movie then the returned future will yield an
    /// error immediately once spawned.
    pub fn movie_loader(
        &mut self,
        player: Weak<Mutex<Player>>,
        fetch: Pin<Box<dyn Future<Output = Result<Vec<u8>, Error>>>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>> {
        let handle = match self {
            Loader::Movie { self_handle, .. } => self_handle.expect("Loader not self-introduced"),
            _ => return Box::pin(async { Err("Non-movie loader spawned as movie loader".into()) }),
        };

        let player = player
            .upgrade()
            .expect("Could not upgrade weak reference to player");

        Box::pin(async move {
            player.lock().expect("Could not lock player!!").update(
                |avm, uc| -> Result<(), Error> {
                    let (clip, broadcaster) = match uc.load_manager.get_loader(handle) {
                        Some(Loader::Movie {
                            target_clip,
                            target_broadcaster,
                            ..
                        }) => (*target_clip, *target_broadcaster),
                        _ => unreachable!(),
                    };

                    clip.as_movie_clip().unwrap().unload(uc);

                    clip.as_movie_clip()
                        .unwrap()
                        .replace_with_movie(uc.gc_context, None);

                    if let Some(broadcaster) = broadcaster {
                        avm.insert_stack_frame_for_method(
                            clip,
                            broadcaster,
                            NEWEST_PLAYER_VERSION,
                            uc,
                            "broadcastMessage",
                            &["onLoadStart".into(), Value::Object(broadcaster)],
                        );
                        avm.run_stack_till_empty(uc)?;
                    }

                    Ok(())
                },
            )?;

            let data = fetch.await;
            if let Ok(data) = data {
                let movie = Arc::new(SwfMovie::from_data(&data));

                player
                    .lock()
                    .expect("Could not lock player!!")
                    .update(|avm, uc| {
                        let (clip, broadcaster) = match uc.load_manager.get_loader(handle) {
                            Some(Loader::Movie {
                                target_clip,
                                target_broadcaster,
                                ..
                            }) => (*target_clip, *target_broadcaster),
                            _ => unreachable!(),
                        };

                        if let Some(broadcaster) = broadcaster {
                            avm.insert_stack_frame_for_method(
                                clip,
                                broadcaster,
                                NEWEST_PLAYER_VERSION,
                                uc,
                                "broadcastMessage",
                                &[
                                    "onLoadProgress".into(),
                                    Value::Object(broadcaster),
                                    data.len().into(),
                                    data.len().into(),
                                ],
                            );
                            avm.run_stack_till_empty(uc)?;
                        }

                        let mut mc = clip
                            .as_movie_clip()
                            .expect("Attempted to load movie into not movie clip");

                        mc.replace_with_movie(uc.gc_context, Some(movie.clone()));
                        mc.post_instantiation(uc.gc_context, clip, avm.prototypes().movie_clip);

                        let mut morph_shapes = fnv::FnvHashMap::default();
                        mc.preload(uc, &mut morph_shapes);

                        // Finalize morph shapes.
                        for (id, static_data) in morph_shapes {
                            let morph_shape = MorphShape::new(uc.gc_context, static_data);
                            uc.library
                                .library_for_movie_mut(movie.clone())
                                .register_character(
                                    id,
                                    crate::character::Character::MorphShape(morph_shape),
                                );
                        }

                        if let Some(broadcaster) = broadcaster {
                            avm.insert_stack_frame_for_method(
                                clip,
                                broadcaster,
                                NEWEST_PLAYER_VERSION,
                                uc,
                                "broadcastMessage",
                                &["onLoadComplete".into(), Value::Object(broadcaster)],
                            );
                            avm.run_stack_till_empty(uc)?;
                        }

                        Ok(())
                    })
            } else {
                //TODO: Inspect the fetch error.
                //This requires cooperation from the backend to send abstract
                //error types we can actually inspect.
                player.lock().expect("Could not lock player!!").update(
                    |avm, uc| -> Result<(), Error> {
                        let (clip, broadcaster) = match uc.load_manager.get_loader(handle) {
                            Some(Loader::Movie {
                                target_clip,
                                target_broadcaster,
                                ..
                            }) => (*target_clip, *target_broadcaster),
                            _ => unreachable!(),
                        };

                        if let Some(broadcaster) = broadcaster {
                            avm.insert_stack_frame_for_method(
                                clip,
                                broadcaster,
                                NEWEST_PLAYER_VERSION,
                                uc,
                                "broadcastMessage",
                                &[
                                    "onLoadError".into(),
                                    Value::Object(broadcaster),
                                    "LoadNeverCompleted".into(),
                                ],
                            );
                            avm.run_stack_till_empty(uc)?;
                        }

                        Ok(())
                    },
                )
            }
        })
    }

    pub fn form_loader(
        &mut self,
        player: Weak<Mutex<Player>>,
        fetch: Pin<Box<dyn Future<Output = Result<Vec<u8>, Error>>>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>> {
        let handle = match self {
            Loader::Form { self_handle, .. } => self_handle.expect("Loader not self-introduced"),
            _ => return Box::pin(async { Err("Non-form loader spawned as form loader".into()) }),
        };

        let player = player
            .upgrade()
            .expect("Could not upgrade weak reference to player");

        Box::pin(async move {
            let data = fetch.await?;

            player.lock().unwrap().update(|avm, uc| {
                let loader = uc.load_manager.get_loader(handle);
                let that = match loader {
                    Some(Loader::Form { target_object, .. }) => *target_object,
                    None => return Err("Loader expired during loading".into()),
                    _ => return Err("Non-movie loader spawned as movie loader".into()),
                };

                for (k, v) in form_urlencoded::parse(&data) {
                    that.set(&k, v.into_owned().into(), avm, uc)?;
                }

                Ok(())
            })
        })
    }

    /// Event handler morally equivalent to `onLoad` on a movie clip.
    ///
    /// Returns `true` if the loader has completed and should be removed.
    ///
    /// Used to fire listener events on clips and terminate completed loaders.
    pub fn movie_clip_loaded(
        &mut self,
        loaded_clip: DisplayObject<'gc>,
        clip_object: Option<Object<'gc>>,
        root: DisplayObject<'gc>,
        queue: &mut ActionQueue<'gc>,
    ) -> bool {
        let (clip, broadcaster) = match self {
            Loader::Movie {
                target_clip,
                target_broadcaster,
                ..
            } => (*target_clip, *target_broadcaster),
            _ => return false,
        };

        if DisplayObject::ptr_eq(loaded_clip, clip) {
            if let Some(broadcaster) = broadcaster {
                queue.queue_actions(
                    clip,
                    root,
                    ActionType::Method {
                        object: broadcaster,
                        name: "broadcastMessage",
                        args: vec![
                            "onLoadInit".into(),
                            clip_object.map(|co| co.into()).unwrap_or(Value::Undefined),
                        ],
                    },
                    false,
                );
            }

            true
        } else {
            false
        }
    }
}
