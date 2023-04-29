pub const PALETTES: &'static [&'static [&'static str]] = &[&[], &["#A57C65"], &["#ED8C72", "#688B8A"], &["#C9A66B", "#488A99", "#CE5A57"], &["#C9A66B", "#4D648D", "#CE5A57", "#5EA8A7"], &["#CE5A57", "#5EA8A7", "#E59D5C", "#739F3D", "#4D648D"], &["#4D648D", "#CE5A57", "#739F3D", "#E59D5C", "#5EA8A7", "#F18D9E"], &["#F18D9E", "#5EA8A7", "#E1B16A", "#D55449", "#739F3D", "#4D648D", "#E38B75"], &["#F79B77", "#4D648D", "#F18D9E", "#6FB98F", "#D55449", "#C9A66B", "#66A6AD", "#739F3D"], &["#488A99", "#FA8D62", "#DBAE58", "#6FB98F", "#375E97", "#B38867", "#D55449", "#F18D9E", "#739F3D"], &["#DBAE58", "#66A6AD", "#F18D9E", "#344D90", "#D55449", "#739F3D", "#68829E", "#B38867", "#6FB98F", "#FA8D62"], &["#BF9A77", "#68829E", "#D55449", "#6FB98F", "#E99787", "#66A6AD", "#F78BD2", "#F69454", "#344D90", "#DBAE58", "#739F3D"], &["#337BAE", "#DE7A22", "#739F3D", "#E7472E", "#5EA8A7", "#F18D9E", "#ED5752", "#EDB83D", "#C9A66B", "#6FB98F", "#ED8C72", "#9D331F"], &["#EDB83D", "#217CA3", "#E05858", "#739F3D", "#E7472E", "#E38B75", "#E1B16A", "#6FB98F", "#5EA8A7", "#9D331F", "#DE7A22", "#F18D9E", "#375E97"], &["#EDB83D", "#9D331F", "#426E86", "#F79B77", "#E7472E", "#739F3D", "#E05858", "#C9A66B", "#5EA8A7", "#344D90", "#DE7A22", "#6FB98F", "#337BAE", "#F18D9E"], &["#217CA3", "#EE693F", "#7CAA2D", "#5EA8A7", "#EDB83D", "#E05858", "#C9A66B", "#E7472E", "#ED8C72", "#6FB98F", "#9D331F", "#344D90", "#5C821A", "#DE7A22", "#F18D9E"], &["#F9BA32", "#217CA3", "#ED5752", "#6FB98F", "#739F3D", "#C9A66B", "#9D331F", "#5EA8A7", "#E7472E", "#DE7A22", "#F69454", "#EC96A4", "#344D90", "#E38B75", "#F78BD2", "#EDB83D"], &["#F69454", "#52958B", "#9D331F", "#217CA3", "#598234", "#D25C00", "#F18D9E", "#DBAE58", "#E7472E", "#68A225", "#F9BA32", "#ED8C72", "#E05858", "#344D90", "#66A6AD", "#BF9A77", "#6FB98F"], &["#E38B75", "#5EA8A7", "#F69454", "#9D331F", "#68A225", "#EDB83D", "#FD3C3C", "#426E86", "#6FB98F", "#E05858", "#F78BD2", "#E1B16A", "#DE7A22", "#598234", "#E7552C", "#B38867", "#2988BC", "#344D90"], &["#F9BA32", "#337BAE", "#962715", "#598234", "#E05858", "#5EA8A7", "#C9A66B", "#ED8C72", "#F18D9E", "#F34A4A", "#344D90", "#E7472E", "#DE7A22", "#BA5536", "#6FB98F", "#F69454", "#68A225", "#DDB74C", "#257982"], &["#E38B75", "#7E7B15", "#337BAE", "#E7472E", "#07575B", "#A43820", "#EDB83D", "#66A6AD", "#7D5642", "#344D90", "#DE7A22", "#68A225", "#F18D9E", "#486824", "#F69454", "#E05858", "#6AB187", "#1A405F", "#662225", "#C9A66B"], &["#E38B75", "#DE7A22", "#2C7873", "#7CAA2D", "#EDB83D", "#E7472E", "#2988BC", "#9D331F", "#5EA8A7", "#344D90", "#7D5642", "#6FB98F", "#F69454", "#486824", "#E05858", "#1A405F", "#C9A66B", "#F18D9E", "#662225", "#426E86", "#004445"], &["#E7472E", "#1E656D", "#EDB83D", "#537027", "#763626", "#F78BD2", "#D55449", "#5EA8A7", "#68A225", "#375E97", "#F69454", "#A43820", "#337BAE", "#C29545", "#1E434C", "#E99787", "#1A405F", "#785A46", "#BF9A77", "#DE7A22", "#6FB98F", "#F52549"], &["#2988BC", "#DE7A22", "#E05858", "#3F681C", "#7D5E3C", "#6FB98F", "#344D90", "#962715", "#E1B16A", "#34675C", "#F9BA32", "#E7472E", "#E99787", "#68A225", "#66A6AD", "#6C2D2C", "#7E7B15", "#F78BD2", "#1A405F", "#FA8D62", "#1E434C", "#AF4425", "#426E86"], &["#F18D9E", "#52958B", "#DE7A22", "#D72C16", "#1A405F", "#68A225", "#BA5536", "#EDB83D", "#3F6C45", "#7D5642", "#662225", "#66A6AD", "#426E86", "#D55448", "#962715", "#B38867", "#6FB98F", "#2988BC", "#537027", "#07575B", "#F79B77", "#C29545", "#F52549", "#375E97"], &["#66A6AD", "#BA5536", "#DE7A22", "#68A225", "#337BAE", "#F34A4A", "#DBAE58", "#07575B", "#3F681C", "#F18D9E", "#785A46", "#E05858", "#F9BA32", "#1A405F", "#ED8C72", "#D72C16", "#962715", "#BF9A77", "#6FB98F", "#F69454", "#662225", "#7E7B15", "#138D90", "#344D90", "#EB5E30"], &["#E7472E", "#138D90", "#D9B44A", "#1E434C", "#A57C65", "#6FB98F", "#337BAE", "#68A225", "#3F6C45", "#9D331F", "#7D5642", "#F18D9E", "#F69454", "#E05858", "#3F681C", "#D25C00", "#F9BA32", "#C9A66B", "#1A405F", "#662225", "#66A6AD", "#375E97", "#EE693F", "#2B616D", "#F79B77", "#7E7B15"], &["#F79B77", "#2C7873", "#763626", "#FA4032", "#7E7B15", "#2988BC", "#1A405F", "#EDAE01", "#B38867", "#E7552C", "#F78BD2", "#DE7A22", "#3F681C", "#E1B16A", "#68A225", "#66A6AD", "#6FB98F", "#7D5642", "#A43820", "#E29930", "#1E434C", "#EDB83D", "#3F6C45", "#EC96A4", "#426E86", "#E05858", "#375E97"], &["#FF4447", "#486824", "#344D90", "#DBAE58", "#2C7873", "#8C0004", "#E05858", "#1A405F", "#F0810F", "#763626", "#ED8C72", "#F78BD2", "#68A225", "#E8A735", "#426E86", "#66A6AD", "#7D5642", "#EE693F", "#6FB98F", "#D72C16", "#EB8A3E", "#B38867", "#EC96A4", "#1E434C", "#2988BC", "#AF4425", "#F5BE41", "#7E7B15"], &["#763626", "#8593AE", "#DE7A22", "#6FB98F", "#8C0004", "#D1B280", "#2B616D", "#3F681C", "#A57C65", "#EDB83D", "#EE693F", "#66A6AD", "#375E97", "#F78BD2", "#68A225", "#D72C16", "#7D5642", "#D8412F", "#E59D5C", "#1A405F", "#AF4425", "#138D90", "#7E7B15", "#E05858", "#2988BC", "#EC96A4", "#F79B77", "#F9BA32", "#3F6C45"], &["#EB8A3E", "#426E86", "#B2473E", "#68A225", "#7D5642", "#6FB98F", "#F9BA32", "#EC96A4", "#962715", "#486824", "#488A99", "#ED5752", "#E1B16A", "#EB5E30", "#1E434C", "#662225", "#1A405F", "#66A6AD", "#DE7A22", "#F79B77", "#EDB83D", "#B38867", "#BE7970", "#375E97", "#2C7873", "#7E7B15", "#2988BC", "#F78BD2", "#D72C16", "#9B4F0F"], &["#A43820", "#3F6C45", "#375E97", "#EDAE01", "#F79B77", "#68A225", "#E05858", "#DE7A22", "#66A6AD", "#662225", "#B38867", "#1E434C", "#EC96A4", "#E1B16A", "#3F681C", "#6FB98F", "#D72C16", "#128277", "#2988BC", "#4C3F54", "#7E7B15", "#8593AE", "#1A405F", "#EDB83D", "#EE693F", "#F78BD2", "#8C0004", "#7D5642", "#006C84", "#EB8A3E", "#FF4447"], &["#F79B77", "#4C3F54", "#486824", "#A10115", "#52958B", "#B38867", "#F9BA32", "#D72C16", "#EC96A4", "#AA4B41", "#68A225", "#1995AD", "#2988BC", "#DE7A22", "#004445", "#6C2D2C", "#FF4447", "#C9A66B", "#785A46", "#D9B44A", "#EE693F", "#1A405F", "#344D90", "#6FB98F", "#5B7065", "#F8A055", "#A43820", "#66A6AD", "#E05858", "#F78BD2", "#68829E", "#7E7B15"], &["#68A225", "#785A46", "#F69454", "#2988BC", "#D72C16", "#5F968E", "#AF4425", "#EDAE01", "#283655", "#3F681C", "#E05858", "#F18D9E", "#335252", "#763626", "#E1B16A", "#CB6318", "#6FB98F", "#ED8C72", "#344D90", "#FF4447", "#8C0004", "#1E434C", "#505160", "#D8412F", "#FA812F", "#7E7B15", "#EDB83D", "#31A9B8", "#138D90", "#B38867", "#4B7447", "#E8A735", "#426E86"], &["#1E434C", "#ED8C72", "#68A225", "#4B7447", "#D72C16", "#FAAF08", "#785A46", "#66A6AD", "#BA5536", "#2988BC", "#EDB83D", "#F0810F", "#283655", "#E4535E", "#763626", "#C9A66B", "#F78BD2", "#6FB98F", "#375E97", "#1E656D", "#EC96A4", "#52958B", "#962715", "#F69454", "#CB6318", "#7E7B15", "#D55448", "#3F681C", "#882426", "#68829E", "#505160", "#5B7065", "#EB5E30", "#FF4447"], &["#D72C16", "#1E434C", "#DE7A22", "#8EBA43", "#962715", "#52958B", "#B38867", "#DBAE58", "#F78BD2", "#4B7447", "#4D648D", "#EE693F", "#785A46", "#763626", "#F9BA32", "#6FB98F", "#F79B77", "#E05858", "#2988BC", "#4F6457", "#66A6AD", "#1A405F", "#F8A055", "#344D90", "#68A225", "#7E7B15", "#FF4447", "#2B616D", "#8593AE", "#3F681C", "#C9A66B", "#882426", "#EC96A4", "#BA5536", "#4C3F54"], &["#7D5642", "#D72C16", "#006C84", "#D9B44A", "#F79B77", "#31A2AC", "#AF4425", "#3F681C", "#4C3F54", "#EC96A4", "#337BAE", "#BF9A77", "#F0810F", "#662225", "#E05858", "#EB8A3E", "#E8A735", "#CB6318", "#6FB98F", "#962715", "#68A225", "#66A6AD", "#F78BD2", "#344D90", "#9F4636", "#4B7447", "#755248", "#1A405F", "#C29545", "#F34A4A", "#EE693F", "#34675C", "#1E434C", "#7E7B15", "#F9BA32", "#E38B75"], &["#D72C16", "#688B8A", "#B38867", "#EDB83D", "#4C3F54", "#468B00", "#F78BD2", "#AA4B41", "#EB5E30", "#6FB98F", "#486824", "#1E434C", "#F8A055", "#785A46", "#FF4447", "#962715", "#344D90", "#F9BA32", "#D25C00", "#337BAE", "#4F6457", "#EC96A4", "#763626", "#1A405F", "#D55448", "#52958B", "#7CAA2D", "#E4535E", "#31A9B8", "#F79B77", "#7E7B15", "#006C84", "#258039", "#C05805", "#882426", "#E1B16A", "#DE7A22"], &["#4B7447", "#A10115", "#E8A735", "#4C3F54", "#66A6AD", "#FA8D62", "#763626", "#3F681C", "#ED5752", "#8EBA43", "#E38B75", "#52958B", "#2988BC", "#DE7A22", "#D72C16", "#1E434C", "#EDB83D", "#EC96A4", "#BF9A77", "#7E7B15", "#785A46", "#34675C", "#E4535E", "#EDAE01", "#FF4447", "#F78BD2", "#1A405F", "#006C84", "#6FB98F", "#AA4B41", "#A43820", "#EB8A3E", "#4D648D", "#EE693F", "#8593AE", "#C29545", "#344D90", "#68A225"], &["#5F968E", "#CB6318", "#E05858", "#C29545", "#505160", "#468B00", "#7D5E3C", "#2988BC", "#EDAE01", "#3F681C", "#1E434C", "#8C0004", "#F78BD2", "#E73F0B", "#662225", "#283655", "#E99787", "#8593AE", "#F52549", "#31A9B8", "#6FB98F", "#F79B77", "#EDB83D", "#006C84", "#BF9A77", "#EE693F", "#F0810F", "#E8A735", "#4B7447", "#344D90", "#7E7B15", "#9D331F", "#7CAA2D", "#D13525", "#128277", "#EB8A3E", "#755248", "#4D648D", "#4F6457"], &["#EB8A3E", "#2988BC", "#BF9A77", "#FF4447", "#68A225", "#5B7065", "#D8412F", "#1A405F", "#EDB83D", "#763626", "#E4535E", "#962715", "#52958B", "#68829E", "#258039", "#F78BD2", "#07575B", "#344D90", "#EE693F", "#D55448", "#785A46", "#C29545", "#537027", "#66A6AD", "#F79B77", "#EC96A4", "#4C3F54", "#FAAF08", "#6FB98F", "#CB6318", "#7E7B15", "#882426", "#E99787", "#5A5F37", "#F0810F", "#AA4B41", "#1E434C", "#4B7447", "#488A99", "#D72C16"]];

pub fn get_palette(n: usize) -> &'static [&'static str] {
    PALETTES.get(n).unwrap()
}

pub fn get_color(i: usize, n: usize) -> String {
    get_palette(n).get(i).unwrap_or(&"#FFFFFF").to_string()
} 