# SwingAway
Summary: 
	I will be creating a simulation of Pete Alonso’s (my favorite player) season if every pitch thrown to him, he had to swing at. After the simulation is done I will compare the numbers of the players actual season and my simulated season. Looking at stats like batting average, OBS, OPS, HRs, slugging percentage and WAR. Obviously his walks will be zero, so it will be interesting to see how this will affect these stats. 
	I will be taking the data of all of his at bats from last season and run them through my program that makes him swing at every pitch. To keep this simulation semi-real the pitchers will not know that Pete has to swing at every pitch, they will still attack him with the same pitches they would normally throw. 

Technologies:
	Using Rust I will take the data for sites like PITCHf/x and Retrosheet to simulate the season. Retrosheet will be giving me the data on all the games and the pitches thrown. PITCHf/x has the data for Pete’s zone profile, hit tendencies and spray charts. Using the data collected from these two sites my Rust program will simulate the season and get his stats. 

Development Plan:
	I have never used Rust before and I will try to get a good foundation of that language before I start anything. Pete’s data comes next, collecting all of it and getting his percentages to hit certain pitches and their pitch location. Then expanding on that data with where he is likely to hit each pitch and how hard/far. Putting all that data in my program and then feeding it all the pitches he was thrown in the 2020 season. Using baseball reference to then get all the formulas for his stats and calculating them in my program. Also using baseball reference to compare his real stats and simulated stats, along with comparing his season with other players. 
PITCHf/x: http://www.brooksbaseball.net/h_landing.php?player=624413
Retrosheet: https://www.retrosheet.org/boxesetc/A/Palonp001.htm
Baseball Reference: https://www.baseball-reference.com/players/a/alonspe01.shtml

Challenges:
	I don’t know how deep I want to dive into the data, PITCHf/x allows me to see how often Pete is to hit a ground ball/ fly ball/ line drive, the launch angle of all his hits and the velocity of the hit. I will have to use all these to simulate if he will hit the ball first off and then were. Not only that, but I will have to set certain parameters of what a single, double, triple and home run is, these will also have to change for each stadium he played at in the 2020 season (especially accounting for the Green Monster in Boston). I need what his hits were to calculate stats like OPS, slugging percentage and WAR, as they take into account extra base hits.
	Never having used Rust before will be its own set of problems. With learning a new programming language I will have to solve my own errors and look up appropriate ways of utilizing the language to its fullest. 
