import re

data = {}

class Color:
    def __init__(self, r,g,b,name):
        self.color = (r,g,b)
        self.name = name[0].upper() + name[1:]

    def __repr__(self):
        return self.name


with open('tmp') as f:
    expr = re.compile("\s*([0-9]+)\s*([0-9]+)\s*([0-9]+)\s*\t+\s*(.+)")
    for line in f:
        m = expr.match(line)

        if not m:
            continue

        (r,g,b,name) = m.groups()
        key = name.lower().replace(" ","")

        if key in data and key.lower() != key:
            data[key] = Color(r,g,b,name)
        else:
            data[key] = Color(r,g,b,name)


def conv_8bit(n):
    return int((n/255.0)*65535)

def rgb_conv(r,g,b):
    return (conv_8bit(r), conv_8bit(g), conv_8bit(b))

def matchbody(r, g, b):

    (r,g,b) = rgb_conv(r,g,b)
    return """
                let mut color = xlib::XColor {{
                    pixel: 0,
                    red: {r},
                    green: {g},
                    blue: {b},
                    flags: 0,
                    pad: 0,
                }};

                XAllocColor(display, cm, &mut color);

                color
    """.format(r=r,g=g,b=b)

data = list(sorted(data.values(), key=lambda x: x.name))

enum_entry = "    {},"
enum_def = "pub enum NamedColor {{\n{}\n}}"

entries = "\n".join([enum_entry.format(x.name.strip()) for x in data])
print enum_def.format(entries)


impl = "impl NamedColor {{\n{}\n}}"
fn = "    fn color(&self, display, cm: xlib::Colormap) -> xlib::XColor {{\n{}\n    }}"
match = "        match &self {{\n{}\n        }}"
arm = "            NamedColor::{} => {{                {}        }},"

arms = []
for x in data:

    (r,g,b) = x.color
    body = matchbody(int(r),int(g),int(b))
    arms.append(arm.format(x.name, body))


print impl.format(
    fn.format(
        match.format(
            "\n".join(arms)
        )
    )
)

