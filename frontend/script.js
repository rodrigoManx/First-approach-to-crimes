// This example requires the Visualization library. Include the libraries=visualization
// parameter when you first load the API. For example:
// <script src="https://maps.googleapis.com/maps/api/js?key=YOUR_API_KEY&libraries=visualization">

var map, infowindow, heatmap, gradient;

function initMap() {
    map = new google.maps.Map(document.getElementById('map'), {
        zoom: 12,
        center: { lat: 33.754504, lng: -84.396582 },
        mapTypeId: 'roadmap'
    });

    var script = document.createElement('script');
    script.src = './heatmap.js';
    document.getElementsByTagName('head')[0].appendChild(script);

    gradient = [
        'rgba(0, 255, 255, 0)',
        'rgba(0, 255, 255, 1)',
        'rgba(0, 191, 255, 1)',
        'rgba(0, 127, 255, 1)',
        'rgba(0, 63, 255, 1)',
        'rgba(0, 0, 255, 1)',
        'rgba(0, 0, 223, 1)',
        'rgba(0, 0, 191, 1)',
        'rgba(0, 0, 159, 1)',
        'rgba(0, 0, 127, 1)',
        'rgba(63, 0, 91, 1)',
        'rgba(127, 0, 63, 1)',
        'rgba(191, 0, 31, 1)',
        'rgba(255, 0, 0, 1)'
    ]

    map.data.setStyle({
        fillColor: 'green',
        strokeWeight: 1,
        icon: 'https://chart.googleapis.com/chart?chst=d_simple_text_icon_left&chld=|12|F00|glyphish_location|12|000|FFF',
        pixelOffset: new google.maps.Size(6, 6)
    });

    infowindow = new google.maps.InfoWindow();

    map.data.addListener('click', function (event) {
        var myHTML = event.feature.getProperty("description");
        infowindow.setContent("<div style='width:190px; text-align: center;'>" + myHTML + "</div>");
        infowindow.setPosition(event.feature.getGeometry().get());
        infowindow.setOptions({ pixelOffset: new google.maps.Size(0, -1) });
        infowindow.open(map);
    });

    map.data.addListener('mouseover', function (event) {
        document.getElementById('info-box').textContent =
            event.feature.getProperty('crime_type');
    });

    // map.addListener('zoom_changed', function () {
    //     if (map.zoom < 12) {
    //         map.data.setStyle({
    //             visibile: false
    //         });
    //     } else {
    //         map.data.setStyle({
    //             visibile: true
    //         });
    //     }
    // });

    map.data.loadGeoJson("./geo.json");

}

function toggleHeatmap() {
    heatmap.setMap(heatmap.getMap() ? null : map);
}

function changeRadius() {
    heatmap.set('radius', heatmap.get('radius') ? null : 17);
}

function changeOpacity() {
    heatmap.set('opacity', heatmap.get('opacity') ? null : 0.2);
}