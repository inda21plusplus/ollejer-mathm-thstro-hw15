Gör en ny output-mapp för nya pizzaprogram så vi kan jämföra med tidigare
resultat eller nåt sånt.

```fish
for f in (echo -e "a\nb\nc\nd\ne")
    python3 scorer.py input_data/$f* < outputs/5/$f
end
```
