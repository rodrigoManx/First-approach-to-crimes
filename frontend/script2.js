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

    infowindow = new google.maps.InfoWindow();

    // map.data.addListener('click', function (event) {
    //     var myHTML = event.feature.getProperty("description");
    //     infowindow.setContent("<div style='width:190px; text-align: center;'>" + myHTML + "</div>");
    //     infowindow.setPosition(event.feature.getGeometry().get());
    //     infowindow.setOptions({ pixelOffset: new google.maps.Size(0, -1) });
    //     infowindow.open(map);var script = document.createElement('script');
    // script.src = './heatmap.js';
    // document.getElementsByTagName('head')[0].appendChild(script);
    // });

    map.data.addListener('click', function (event) {
        var myHTML = event.feature.getProperty("description");
        infowindow.setContent("<div style='width:190px; text-align: center;'>" + myHTML + "</div>");
        infowindow.setPosition(event.feature.getGeometry().get());
        infowindow.setOptions({ pixelOffset: new google.maps.Size(0, -10) });
        infowindow.open(map);
    });

    map.data.addListener('mouseover', function (event) {
        document.getElementById('info-box').textContent =
            event.feature.getProperty('crime_type');
    });

    map.data.setStyle(function(feature) {
        var group = feature.getProperty('cluster_group');
        return {
        fillColor: 'green',
        strokeWeight: 1,
        icon: 'icons/' + group + '.png',
        pixelOffset: new google.maps.Size(6, 6)
        }
    });

    map.data.loadGeoJson("cluster.json");
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