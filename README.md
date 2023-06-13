# What does this do?
Hunt: Showdown displays your current MMR rank as a number of "stars" in game, based on which bracket you fall into. Your actual MMR is a numerical value. That value is written to an XML file by the game, but not displayed in the UI anywhere. This application will read that XML file (and watch it for changes while the app is running), and pull out your MMR.

## Why are there multiple values?
The game tracks your MMR at the end of each match, and sometimes saves 1-3 values as it changes from game to game. The grayed out values are from past games, while the brighter one is the most recent.

# How do I use this?
Download the most recent release, provide a config file, and run it.

There is an example config file called `config.json.example` included with the download. You can rename it to `config.json` (remove the .example at the end), and edit it to configure the application. There are two values stored in the config. The first is the `file_path` for the XML file the application monitors. The example has a best-guess location for Windows, but may not be correct depending on where you installed steam, and the game. 

You can find the path to the file by right-clicking on the game in steam, then selected "Properties", then "Local Files", then "Browse". Take the path it opens and append: "user/profiles/default/attributes.xml"

The second value is `player_names`, which is a list of which players you want to monitor in the following format:
```json
"player_names": [
  "player1",
  "player2",
  "player3"
]
```

Put your steam name (and the steam names of any friends you play with frequently) in the list to tell the application which players to track MMR for. (The game only saves the MMR changes for matches you were in, so it won't monitor any changes to other player's MMRs in games you weren't playing with them.) The names are not case sensitive, though they will appear in the application with whatever casing you use in the config file.
