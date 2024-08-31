release:
    source ~/.profile; cargo leptos serve --release

dev:
    source ~/.profile; cargo leptos serve

w:
    source ~/.profile; cargo leptos watch

dist:
    source ~/.profile; cargo leptos build --release;

git:
    -git add *;
    -git add .gitignore;
    -git commit -m "update";
    -git push;