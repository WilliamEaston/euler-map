Just a simple project creating a map projection using [Euler spirals](https://en.wikipedia.org/wiki/Euler_spiral) with the thinnest band possible. I (of course) got the idea from [the Numberphile video](https://www.youtube.com/watch?v=D3tdW9l1690). The Mercator projection map I used as input is from [https://vemaps.com/world/wrld-21](https://vemaps.com/world/wrld-21).

The image files get to be ridiculously large when the band is made smaller because the region with actual perceivable detail is so small relative to the overall spiral. As such, I had to start outputting just zoomed in parts of the image around the poles (see output.png).

I like how it looks when the band is a little thicker so you can zoom out a bit and see the spiral:<img width="1343" height="907" alt="North" src="https://github.com/user-attachments/assets/147c8460-c6a3-4ed5-b577-804d3c9eb1cd" />

Just for fun, here's a screenshot from the first time I got it working and had an output that remotely resembled a map: <img width="1504" height="860" alt="First successful Euler Map - cropped" src="https://github.com/user-attachments/assets/22d5e45a-1bb7-4439-9658-4eb7a7ee83ba" />
