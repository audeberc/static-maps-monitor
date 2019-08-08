pub fn get_yandex_url(
    layer: String,
    lon0: f64,
    lat0: f64,
    lon1: f64,
    lat1: f64,
    _api_key: String,
) -> String {
    format!(
        "https://static-maps.yandex.ru/1.x/?lang=en_US&l={}&bbox={},{}~{},{}",
        layer, lon0, lat0, lon1, lat1
    )
}

pub fn get_google_url(
    layer: String,
    lon0: f64,
    lat0: f64,
    lon1: f64,
    lat1: f64,
    api_key: String,
) -> String {
    let width = lon1 - lon0; // width in deg
    let height = lat1 - lat0;
    let zoom_level = (360.0 / width).log2().round();
    let clat = lat0 + (height / 2.0);
    let clon = lon0 + (width / 2.0);
    format!("https://maps.googleapis.com/maps/api/staticmap?center={},{}&zoom={}&maptype={}&key={}&format=jpg&size=640x640&scale=2", clat, clon, zoom_level, layer, api_key)
}
/* NOT TESTED
pub fn get_baidu_url(layer: String, lon0: f64, lat0: f64, lon1: f64, lat1: f64, api_key: String) -> String {
    let width = lon1 - lon0; // width in deg
    let height = lat1 - lat0;
    let zoom_level = (360.0 / width).log2().round();
    let clat = lat0 + (height/2.0);
    let clon = lon0 + (width/2.0);
    format!("http://api.map.baidu.com/staticimage/v2?ak={}&mcode=666666&center={},{}&width=512&height=512&zoom=11  ", clon, clat, zoom_level, layer, api_key)
}
*/
pub fn get_bing_url(
    layer: String,
    lon0: f64,
    lat0: f64,
    lon1: f64,
    lat1: f64,
    api_key: String,
) -> String {
    // Layer: "Aerial"
    let width = lon1 - lon0; // width in deg
    let height = lat1 - lat0;
    let zoom_level = (360.0 / width).log2().round();
    let clat = lat0 + (height / 2.0);
    let clon = lon0 + (width / 2.0);
    format!("https://dev.virtualearth.net/REST/v1/Imagery/Map/{}/{},{}/{}?format=jpeg&mapSize=1500,1500&key={}", layer, clat, clon, zoom_level, api_key)
}

