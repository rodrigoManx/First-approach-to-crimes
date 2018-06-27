var reH = /\d\d:\d\d:\d\d/;
var reD = /Mon\,|Tue\,|Wed\,|Thu\,|Fri\,|Sat\,|Sun\,|Unk\,/;

d = {"Unk":0, "Sun": 1, "Mon": 2, "Tue": 3, "Wed": 4,"Thu": 5, "Fri": 6,"Sat": 7}

d3.json("geo.json", function(error, data) 
{
  if (error) throw error;

  var frequenciesH = d3.nest()
      .key(function(d) { return getHour(d.properties.description); })
      .rollup(function(t) { return t.length; })
      .entries(data.features);

  var frequenciesD = d3.nest()
      .key(function(d) { return getDay(d.properties.description); })
      .rollup(function(t) { return t.length; })
      .entries(data.features);

  frequenciesH.sort(function(a, b) { return (a.key - b.key)*1; });
  frequenciesD.sort(function(a, b) { return (d[a.key] - d[b.key])*1; });
  console.log(frequenciesD);

  drawBars("Hours", frequenciesH);
  drawBars("Days", frequenciesD);
  drawBars("Weeks", frequenciesH);

});

function drawBars(div, frequencies) {
  var width = document.getElementById(div).offsetWidth;
  var height = document.getElementById(div).offsetHeight;
  var y = d3.scaleLinear().range([height*0.9, height*0.2]);
  var x = d3.scaleBand().range([width*0.1, width*0.95]);

  y.domain([0, d3.max(frequencies, function(d) { return d.value; })]);
  x.domain(frequencies.map(function(d) { return d.key; })).padding(0.1);

  var svg = d3.select("#"+div)
    .append("svg")
    .attr("class", "TypesBars");

  svg.append("g")
    .attr("class", "x axis")
    .attr("transform", "translate(" + width * 0.00 + "," + height * 0.9 + ")")
    .call(d3.axisBottom(x));

  svg.selectAll("line")
    .style("display", "none");

  svg.append("g")
    .attr("class", "y axis")
    .attr("transform", "translate(" + width * 0.1 + "," + height * 0 + ")")
    .call(d3.axisLeft(y).ticks(10));//.tickSizeInner([-width*0.85]));

  svg.selectAll("path")
    .style("display", "none");

  svg.selectAll(".bar")
    .data(frequencies)
  .enter().append("rect")
    .attr("class", "bar")
    .attr("transform", "translate(" + width * 0.0 + "," + height * 0 + ")")
    .attr("x", function(d) { return x(d.key); })
    .attr("y", function(d) { return height*0.9 - (height*0.9 - y(d.value));})
    .attr("height", function(d) { return (height*0.9 - y(d.value)); })//function(d) { return y(d.value)*0.7; })
    .attr("width", x.bandwidth());//function(d) { return x(d.value)*0.45; });
}


function getHour(string) {
    var res = reH.exec(string);
    return res[0].substring(0,2);
}

function getDay(string) {
    var res = reD.exec(string);
    return res[0].substring(0,3);
}