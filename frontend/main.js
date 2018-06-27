function toggleHeatmap() {
    heatmap.setMap(heatmap.getMap() ? null : map);
}

function changeRadius() {
    heatmap.set('radius', heatmap.get('radius') ? null : 17);
}

function changeOpacity() {
    heatmap.set('opacity', heatmap.get('opacity') ? null : 0.2);
}