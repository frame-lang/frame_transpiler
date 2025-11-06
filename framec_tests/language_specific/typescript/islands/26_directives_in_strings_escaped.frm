@target typescript

system StringEscapesTS {
    operations:
    run() {
        const s1 = "escaped -> \"$Next\" inside quotes";
        const s2 = 'push $$\'[+\'] in single quotes';
        const s3 = `forward => \`$^\``;
        const ok = s1.length + s2.length + s3.length;
    }
    machine:
        $Init { run() { } }
}

