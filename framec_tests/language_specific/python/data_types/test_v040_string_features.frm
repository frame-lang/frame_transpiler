@target python

# Python-specific override for v0.40 string features using native bodies

system PyStringFeaturesV040 {
    interface:
        run()

    machine:
        $Start {
            run() {
                print("Testing v0.40 String Features (Python)")
                print("==============================")

                # 1. F-strings
                name = "Frame"
                version = 0.40
                count = 3
                msg1 = f"Hello {name}!"
                print(msg1)
                msg2 = f"Language: {name}, Version: {version}, Count: {count}"
                print(msg2)
                msg3 = f"Sum: {2 + 3}, Product: {4 * 5}"
                print(msg3)
                msg4 = f"Pi: {3.14159:.2f}"
                print(msg4)

                # 2. Raw strings
                path1 = r"C:\\Users\\Frame\\Documents"
                print(path1)
                raw1 = r"Line 1\nLine 2\tTabbed"
                print(raw1)
                raw2 = r"He said 'Hello'"
                print(raw2)

                # 3. Byte strings
                bytes1 = b"Hello bytes"
                print(bytes1)
                bytes2 = b"\x48\x65\x6c\x6c\x6f"
                print(bytes2)

                # 4. Triple-quoted strings
                multi1 = """This is a
multi-line string
with triple quotes"""
                print(multi1)

                multi2 = """
                First line
                    Indented line
                Last line
                """
                print(multi2)

                multi3 = """This has "quotes" and 'single quotes' inside"""
                print(multi3)

                # 5. Prefixed triple-quoted
                raw_multi = r"""Raw string
with \n not escaped
and \t not a tab"""
                print(raw_multi)
                f_multi = f"""Hello {name}
This is line 2
This is line 3"""
                print(f_multi)

                # 6. Percent formatting
                msgp1 = "Language: %s" % name
                print(msgp1)
                msgp2 = "Name: %s, Version: %.2f, Count: %d" % (name, version, count)
                print(msgp2)
                msgp3 = "%(lang)s v%(ver).1f has %(cnt)d features" % {"lang": name, "ver": version, "cnt": count}
                print(msgp3)
                msgp4 = "Hex: %x, Octal: %o, Float: %f" % (255, 8, 3.14159)
                print(msgp4)

                # 7. Mixed
                result1 = f"Using {name}" + " with " + r"path\\to\\file"
                print(result1)
                template = "Language: {}, Version: {:.1f}"
                result2 = template.format(name, version)
                print(result2)
                upper = f"{name}".upper()
                lower = r"FRAME".lower()
                stripped = """  spaces  """.strip()
                print(upper)
                print(lower)
                print(stripped)

                print("\n==============================")
                return
            }
        }
}

fn main() {
    t = PyStringFeaturesV040()
    t.run()
}

