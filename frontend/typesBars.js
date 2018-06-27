d3.json("geo.json", function(error, data) 
{
	if (error) throw error;

	var width = document.getElementById('Type').offsetWidth;
	var height = document.getElementById('Type').offsetHeight;
	var x = d3.scaleLinear().range([width*0.5, width*0.95]);
	var y = d3.scaleBand().range([height*0.2, height*0.9]);

	console.log(data.features);

	var frequencies = d3.nest()
    	.key(function(d) { return d.properties.crime_type; })
    	.rollup(function(t) { return t.length; })
    	.entries(data.features);

	

    x.domain([0, d3.max(frequencies, function(d) { return d.value; })]);
	y.domain(frequencies.map(function(d) { return d.key; })).padding(0.1);


	var svg = d3.select("#Type")
		.append("svg")
		.attr("class", "TypesBars");

  	svg.append("g")
        .attr("class", "y axis")
        .attr("transform", "translate(" + width * 0.5 + "," + height * 0 + ")")
        .call(d3.axisLeft(y));

	svg.selectAll("line")
  		.style("display", "none");
    
	svg.append("g")
        .attr("class", "x axis")
       	.attr("transform", "translate(0," + height * 0.9 + ")")
      	.call(d3.axisBottom(x).ticks(5).tickSizeInner([-5]));

  	svg.selectAll("path")
  		.style("display", "none");

    svg.selectAll(".bar")
        .data(frequencies)
      .enter().append("rect")
        .attr("class", "bar")
        .attr("transform", "translate(" + width * 0.5 + "," + height * 0 + ")")
        .attr("height", y.bandwidth())
        .attr("y", function(d) { return y(d.key); })
        .attr("width", function(d) { return x(d.value)*0.45; });
});
