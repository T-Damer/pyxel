use crate::Pyxel;

pub struct Resource {}

impl Resource {
    pub fn new() -> Resource {
        Resource {}
    }

    /*
    void ClearImage(int32_t image_index);
    void ClearTilemap(int32_t tilemap_index);
    void ClearSound(int32_t sound_index);
    void ClearMusic(int32_t music_index);

    std::string DumpImage(int32_t image_index) const;
    std::string DumpTilemap(int32_t tilemap_index) const;
    std::string DumpSound(int32_t sound_index) const;
    std::string DumpMusic(int32_t music_index) const;

    void ParseImage(int32_t image_index, const std::string& str);
    void ParseTilemap(int32_t tilemap_index, const std::string& str);
    void ParseSound(int32_t sound_index, const std::string& str);
    void ParseMusic(int32_t music_index, const std::string& str);

    static std::string GetVersionName();
    static std::string GetImageName(int32_t image_index);
    static std::string GetTilemapName(int32_t tilemap_index);
    static std::string GetSoundName(int32_t sound_index);
    static std::string GetMusicName(int32_t music_index);
    */
}

impl Pyxel {
    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        //
    }

    pub fn save(&mut self, filename: &str) {
        //
    }
}
