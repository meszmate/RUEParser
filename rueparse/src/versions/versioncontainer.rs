use std::collections::HashMap;

use super::{EGame, EUnrealEngineObjectUE4Version, FPackageFileVersion};
use crate::assets::exports::texture::ETexturePlatform;
use crate::objects::core::serialization::FCustomVersionContainer;

#[derive(Debug)]
pub struct VersionContainer {
    game: EGame,
    ver: FPackageFileVersion,
    platform: ETexturePlatform,
    pub b_explicit_ver: bool,

    pub custom_versions: Option<FCustomVersionContainer>,
    pub options: HashMap<String, bool>,
    pub mapstruct_types: HashMap<String, (String, Option<String>)>,

    option_overrides: Option<HashMap<String, bool>>,
    mapstruct_overrides: Option<HashMap<String, (String, Option<String>)>>,
}

impl VersionContainer {
    pub fn new(
        game: Option<EGame>,
        platform: Option<ETexturePlatform>,
        ver: Option<FPackageFileVersion>,
        custom_versions: Option<FCustomVersionContainer>,
        option_overrides: Option<HashMap<String, bool>>,
        mapstruct_overrides: Option<HashMap<String, (String, Option<String>)>>,
    ) -> Self {
        let game = game.unwrap_or(EGame::GAME_UE4_LATEST);
        let platform = platform.unwrap_or(ETexturePlatform::DesktopMobile);
        let ver = ver.unwrap_or(FPackageFileVersion::default());

        Self {
            game,
            platform,
            ver,
            custom_versions,
            b_explicit_ver: false,
            mapstruct_types: HashMap::new(),
            options: HashMap::new(),
            option_overrides,
            mapstruct_overrides,
        }
    }

    pub fn get_game(&mut self) -> &EGame {
        self.init_options();
        self.init_map_struct_types();
        &self.game
    }

    pub fn get_ver(&mut self) -> &FPackageFileVersion {
        self.b_explicit_ver = self.ver.file_version_ue4 != 0 || self.ver.file_version_ue5 != 0;
        if !self.b_explicit_ver {
            self.ver = self.game.GetVersion();
        }
        &self.ver
    }

    pub fn get_platform(&mut self) -> &ETexturePlatform {
        self.init_options();
        self.init_map_struct_types();
        &self.platform
    }

    fn init_options(&mut self) {
        self.options.clear();

        // objects
        self.options.insert(String::from("MorphTarget"), true);

        // structs
        self.options.insert(
            String::from("Vector_NetQuantize_AsStruct"),
            self.game as u32 >= EGame::GAME_UE5_0,
        );

        // fields
        self.options.insert(
            String::from("RawIndexBuffer.HasShouldExpandTo32Bit"),
            self.game as u32 >= EGame::GAME_UE4_25
                && self.game as u32 != EGame::GAME_DeltaForceHawkOps,
        );
        self.options.insert(
            String::from("ShaderMap.UseNewCookedFormat"),
            self.game as u32 >= EGame::GAME_UE5_0,
        );
        self.options.insert(
            String::from("SkeletalMesh.UseNewCookedFormat"),
            self.game as u32 >= EGame::GAME_UE4_24,
        );
        self.options.insert(
            String::from("SkeletalMesh.HasRayTracingData"),
            (self.game as u32 >= EGame::GAME_UE4_27) || self.game as u32 == EGame::GAME_UE4_25_Plus,
        );
        self.options.insert(
            String::from("StaticMesh.HasLODsShareStaticLighting"),
            (EGame::GAME_UE4_15 > self.game as u32) || (self.game as u32 >= EGame::GAME_UE4_16),
        );
        self.options.insert(
            String::from("StaticMesh.HasRayTracingGeometry"),
            self.game as u32 >= EGame::GAME_UE4_25,
        );
        self.options.insert(
            String::from("StaticMesh.HasVisibleInRayTracing"),
            self.game as u32 >= EGame::GAME_UE4_26,
        );
        self.options.insert(
            String::from("StaticMesh.UseNewCookedFormat"),
            self.game as u32 >= EGame::GAME_UE4_23,
        );
        self.options.insert(
            String::from("VirtualTextures"),
            self.game as u32 >= EGame::GAME_UE4_23,
        );
        self.options.insert(
            String::from("SoundWave.UseAudioStreaming"),
            self.game as u32 >= EGame::GAME_UE4_25 && self.override_use_audio_streaming(),
        );
        self.options.insert(
            String::from("AnimSequence.HasCompressedRawSize"),
            self.game as u32 >= EGame::GAME_UE4_17,
        );
        self.options.insert(
            String::from("StaticMesh.HasNavCollision"),
            self.ver.file_version_ue4 as u32
                >= EUnrealEngineObjectUE4Version::STATIC_MESH_STORE_NAV_COLLISION as u32
                && self.game != EGame::GearsOfWar4
                && self.game != EGame::TEKKEN7,
        );

        // special general property workarounds
        self.options
            .insert(String::from("ByteProperty.TMap64Bit"), false);
        self.options
            .insert(String::from("ByteProperty.TMap16Bit"), false);
        self.options
            .insert(String::from("ByteProperty.TMap8Bit"), false);

        // defaults
        self.options
            .insert(String::from("StripAdditiveRefPose"), false);
        self.options.insert(
            String::from("SkeletalMesh.KeepMobileMinLODSettingOnDesktop"),
            false,
        );
        self.options.insert(
            String::from("StaticMesh.KeepMobileMinLODSettingOnDesktop"),
            false,
        );
        match &self.option_overrides {
            Some(o) => {
                for (key, value) in o {
                    self.options.insert(key.clone(), *value);
                }
            }
            None => return,
        }
    }
    fn override_use_audio_streaming(&self) -> bool {
        self.game != EGame::UE4_28
            && self.game != EGame::GTATheTrilogyDefinitiveEdition
            && self.game != EGame::ReadyOrNot
            && self.game != EGame::BladeAndSoul
            && self.game != EGame::Stray
    }
    fn init_map_struct_types(&mut self) {
        self.mapstruct_types.clear();

        self.mapstruct_types.insert(
            String::from("BindingIdToReferences"),
            (String::from("Guid"), None),
        );
        self.mapstruct_types.insert(
            String::from("UserParameterRedirects"),
            (
                String::from("NiagaraVariable"),
                Some(String::from("NiagaraVariable")),
            ),
        );
        self.mapstruct_types.insert(
            String::from("Tracks"),
            (String::from("MovieSceneTrackIdentifier"), None),
        );
        self.mapstruct_types.insert(
            String::from("SubSequences"),
            (String::from("MovieSceneSequenceID"), None),
        );
        self.mapstruct_types.insert(
            String::from("Hierarchy"),
            (String::from("MovieSceneSequenceID"), None),
        );
        self.mapstruct_types.insert(
            String::from("TrackSignatureToTrackIdentifier"),
            (
                String::from("Guid"),
                Some(String::from("MovieSceneTrackIdentifier")),
            ),
        );
        self.mapstruct_types.insert(
            String::from("UserParameterRedirects"),
            (
                String::from("NiagaraVariable"),
                Some(String::from("NiagaraVariable")),
            ),
        );
        match &self.mapstruct_overrides {
            Some(o) => {
                for (k, v) in o {
                    self.mapstruct_types.insert(k.clone(), v.clone());
                }
            }
            None => return,
        }
    }
}
