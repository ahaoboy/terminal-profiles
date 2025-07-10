curl -L "https://aka.ms/terminal-profiles-schema" -o profiles.json

quicktype \
--lang rust \
--derive serde_derive \
--serde-references \
--visibility public \
--src profiles.json \
-o src/profiles.rs


quicktype --src profiles.json