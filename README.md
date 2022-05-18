# Lego Games Patcher
This is a small program which patches TT Lego games to allow newer Lego games to load modified/extracted .DAT files.
I reverse-engineered [AlubJ's tool](https://github.com/AlubJ/TTGamesPatcher) for fun, so I uploaded it here. The original
TTGamesPatcher is closed-source and uses .NET, so the code here is an alternative for those who aren't on Windows, who don't
want .NET, and those who don't want to run random binaries without source code.

# Disclaimer
This tool was written as a simple, fun exercise in reverse-engineering. This program uses the same basic process of writing over 
binary data as the original author's code. As such, I have not tested this code on any Lego Game myself. I instead
wrote a Python tool, "tools/gen_binary.py" which creates dummy files mimicking the pattern AlubJ's tool originally searched for,
with which both programs produce the same output. If it does not work on a specific game, let me know.

Additionally, this program's intention is to allow modders to add custom content to their game. This is not a "crack" related to
piracy, and to my knowledge cannot fulfill that function. Also, it does not remove DRM from Lego Games, it simply allows the games
to read custom, community-made content. However, I am no legal expert, and so if this code violates Lego's EULA 
or any law I will take it down prompty.