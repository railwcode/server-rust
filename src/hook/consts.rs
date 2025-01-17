//! The port of `hook.js`. Commit `3b94b5f13468d01ee4cbb413eb6ddb0f10dcb45a`.

pub const HOOK_TARGET_HOST: [&str; 5] = [
    "music.163.com",
    "interface.music.163.com",
    "interface3.music.163.com",
    "apm.music.163.com",
    "apm3.music.163.com",
];

pub const HOOK_TARGET_PATH: [&str; 34] = [
    "/api/v3/playlist/detail",
    "/api/v3/song/detail",
    "/api/v6/playlist/detail",
    "/api/album/play",
    "/api/artist/privilege",
    "/api/album/privilege",
    "/api/v1/artist",
    "/api/v1/artist/songs",
    "/api/artist/top/song",
    "/api/v1/album",
    "/api/album/v3/detail",
    "/api/playlist/privilege",
    "/api/song/enhance/player/url",
    "/api/song/enhance/player/url/v1",
    "/api/song/enhance/download/url",
    "/api/song/enhance/privilege",
    "/batch",
    "/api/batch",
    "/api/v1/search/get",
    "/api/v1/search/song/get",
    "/api/search/complex/get",
    "/api/cloudsearch/pc",
    "/api/v1/playlist/manipulate/tracks",
    "/api/song/like",
    "/api/v1/play/record",
    "/api/playlist/v4/detail",
    "/api/v1/radio/get",
    "/api/v1/discovery/recommend/songs",
    "/api/v1/discovery/recommend/songs",
    "/api/usertool/sound/mobile/promote",
    "/api/usertool/sound/mobile/theme",
    "/api/usertool/sound/mobile/animationList",
    "/api/usertool/sound/mobile/all",
    "/api/usertool/sound/mobile/detail",
];

pub const HOOK_DOMAIN_LIST: [&str; 5] = [
    "music.163.com",
    "music.126.net",
    "iplay.163.com",
    "look.163.com",
    "y.163.com",
];
