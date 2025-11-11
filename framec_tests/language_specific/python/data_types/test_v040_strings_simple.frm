@target python

# Python-specific override for simplified v0.40 string features using native bodies

system PyStringSimpleV040 {
    interface:
        run()

    machine:
        $Start {
            run() {
                name = "Frame"
                version = 0.40

                # F-strings
                fstr1 = f"Hello {name}!"
                fstr2 = f"Version {version}"
                print(fstr1)
                print(fstr2)

                # Raw strings
                raw1 = r"C:\\path\\to\\file"
                raw2 = r"Line 1\nLine 2"
                print(raw1)
                print(raw2)

                # Byte strings
                bytes1 = b"Binary data"
                print(bytes1)

                # Triple-quoted strings
                multi = """This is
a multi-line
string"""
                print(multi)

                # Raw triple-quoted
                raw_multi = r"""Raw
with \n literal"""
                print(raw_multi)

                # Percent formatting
                pct1 = "Hello %s" % name
                pct2 = "Version %.1f" % version
                pct3 = "%s v%.1f" % (name, version)
                print(pct1)
                print(pct2)
                print(pct3)

                return
            }
        }
}

fn main() {
    t = PyStringSimpleV040()
    t.run()
}

