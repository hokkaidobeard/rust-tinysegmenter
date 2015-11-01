const BIAS: i32 = -332;

lazy_static! {
  static ref B1: char = unsafe { char::from_u32_unchecked(0x110001) };
  static ref B2: char = unsafe { char::from_u32_unchecked(0x110002) };
  static ref B3: char = unsafe { char::from_u32_unchecked(0x110003) };
  static ref E1: char = unsafe { char::from_u32_unchecked(0x110004) };
  static ref E2: char = unsafe { char::from_u32_unchecked(0x110005) };
  static ref E3: char = unsafe { char::from_u32_unchecked(0x110006) };
}

lazy_static! {  
  static ref BC1: HashMap<(char, char), i32> = hashmap! { ('H', 'H') => 6, ('I', 'I') => 2461, ('K', 'H') => 406, ('O', 'H') => -1378, };
  static ref BC2: HashMap<(char, char), i32> = hashmap! { ('A', 'A') => -3267, ('A', 'I') => 2744, ('A', 'N') => -878, ('H', 'H') => -4070, ('H', 'M') => -1711, ('H', 'N') => 4012, ('H', 'O') => 3761, ('I', 'A') => 1327, ('I', 'H') => -1184, ('I', 'I') => -1332, ('I', 'K') => 1721, ('I', 'O') => 5492, ('K', 'I') => 3831, ('K', 'K') => -8741, ('M', 'H') => -3132, ('M', 'K') => 3334, ('O', 'O') => -2920, };
  static ref BC3: HashMap<(char, char), i32> = hashmap! { ('H', 'H') => 996, ('H', 'I') => 626, ('H', 'K') => -721, ('H', 'N') => -1307, ('H', 'O') => -836, ('I', 'H') => -301, ('K', 'K') => 2762, ('M', 'K') => 1079, ('M', 'M') => 4034, ('O', 'A') => -1652, ('O', 'H') => 266, };
  static ref BP1: HashMap<(char, char), i32> = hashmap! { ('B', 'B') => 295, ('O', 'B') => 304, ('O', 'O') => -125, ('U', 'B') => 352, };
  static ref BP2: HashMap<(char, char), i32> = hashmap! { ('B', 'O') => 60, ('O', 'O') => -1762, };
  static ref BQ1: HashMap<(char, char, char), i32> = hashmap! { ('B', 'H', 'H') => 1150, ('B', 'H', 'M') => 1521, ('B', 'I', 'I') => -1158, ('B', 'I', 'M') => 886, ('B', 'M', 'H') => 1208, ('B', 'N', 'H') => 449, ('B', 'O', 'H') => -91, ('B', 'O', 'O') => -2597, ('O', 'H', 'I') => 451, ('O', 'I', 'H') => -296, ('O', 'K', 'A') => 1851, ('O', 'K', 'H') => -1020, ('O', 'K', 'K') => 904, ('O', 'O', 'O') => 2965, };
  static ref BQ2: HashMap<(char, char, char), i32> = hashmap! { ('B', 'H', 'H') => 118, ('B', 'H', 'I') => -1159, ('B', 'H', 'M') => 466, ('B', 'I', 'H') => -919, ('B', 'K', 'K') => -1720, ('B', 'K', 'O') => 864, ('O', 'H', 'H') => -1139, ('O', 'H', 'M') => -181, ('O', 'I', 'H') => 153, ('U', 'H', 'I') => -1146, };
  static ref BQ3: HashMap<(char, char, char), i32> = hashmap! { ('B', 'H', 'H') => -792, ('B', 'H', 'I') => 2664, ('B', 'I', 'I') => -299, ('B', 'K', 'I') => 419, ('B', 'M', 'H') => 937, ('B', 'M', 'M') => 8335, ('B', 'N', 'N') => 998, ('B', 'O', 'H') => 775, ('O', 'H', 'H') => 2174, ('O', 'H', 'M') => 439, ('O', 'I', 'I') => 280, ('O', 'K', 'H') => 1798, ('O', 'K', 'I') => -793, ('O', 'K', 'O') => -2242, ('O', 'M', 'H') => -2402, ('O', 'O', 'O') => 11699, };
  static ref BQ4: HashMap<(char, char, char), i32> = hashmap! { ('B', 'H', 'H') => -3895, ('B', 'I', 'H') => 3761, ('B', 'I', 'I') => -4654, ('B', 'I', 'K') => 1348, ('B', 'K', 'K') => -1806, ('B', 'M', 'I') => -3385, ('B', 'O', 'O') => -12396, ('O', 'A', 'H') => 926, ('O', 'H', 'H') => 266, ('O', 'H', 'K') => -2036, ('O', 'N', 'N') => -973, };
  static ref BW1: HashMap<(char, char), i32> = hashmap! { (',', 'と') => 660, (',', '同') => 727, (*B1, 'あ') => 1404, (*B1, '同') => 542, ('、', 'と') => 660, ('、', '同') => 727, ('」', 'と') => 1682, ('あ', 'っ') => 1505, ('い', 'う') => 1743, ('い', 'っ') => -2055, ('い', 'る') => 672, ('う', 'し') => -4817, ('う', 'ん') => 665, ('か', 'ら') => 3472, ('が', 'ら') => 600, ('こ', 'う') => -790, ('こ', 'と') => 2083, ('こ', 'ん') => -1262, ('さ', 'ら') => -4143, ('さ', 'ん') => 4573, ('し', 'た') => 2641, ('し', 'て') => 1104, ('す', 'で') => -3399, ('そ', 'こ') => 1977, ('そ', 'れ') => -871, ('た', 'ち') => 1122, ('た', 'め') => 601, ('っ', 'た') => 3463, ('つ', 'い') => -802, ('て', 'い') => 805, ('て', 'き') => 1249, ('で', 'き') => 1127, ('で', 'す') => 3445, ('で', 'は') => 844, ('と', 'い') => -4915, ('と', 'み') => 1922, ('ど', 'こ') => 3887, ('な', 'い') => 5713, ('な', 'っ') => 3015, ('な', 'ど') => 7379, ('な', 'ん') => -1113, ('に', 'し') => 2468, ('に', 'は') => 1498, ('に', 'も') => 1671, ('に', '対') => -912, ('の', '一') => -501, ('の', '中') => 741, ('ま', 'せ') => 2448, ('ま', 'で') => 1711, ('ま', 'ま') => 2600, ('ま', 'る') => -2155, ('や', 'む') => -1947, ('よ', 'っ') => -2565, ('れ', 'た') => 2369, ('れ', 'で') => -913, ('を', 'し') => 1860, ('を', '見') => 731, ('亡', 'く') => -1886, ('京', '都') => 2558, ('取', 'り') => -2784, ('大', 'き') => -2604, ('大', '阪') => 1497, ('平', '方') => -2314, ('引', 'き') => -1336, ('日', '本') => -195, ('本', '当') => -2423, ('毎', '日') => -2113, ('目', '指') => -724, ('｣', 'と') => 1682, };
  static ref BW2: HashMap<(char, char), i32> = hashmap! { ('.', '.') => -11822, ('1', '1') => -669, ('―', '―') => -5730, ('−', '−') => -13175, ('い', 'う') => -1609, ('う', 'か') => 2490, ('か', 'し') => -1350, ('か', 'も') => -602, ('か', 'ら') => -7194, ('か', 'れ') => 4612, ('が', 'い') => 853, ('が', 'ら') => -3198, ('き', 'た') => 1941, ('く', 'な') => -1597, ('こ', 'と') => -8392, ('こ', 'の') => -4193, ('さ', 'せ') => 4533, ('さ', 'れ') => 13168, ('さ', 'ん') => -3977, ('し', 'い') => -1819, ('し', 'か') => -545, ('し', 'た') => 5078, ('し', 'て') => 972, ('し', 'な') => 939, ('そ', 'の') => -3744, ('た', 'い') => -1253, ('た', 'た') => -662, ('た', 'だ') => -3857, ('た', 'ち') => -786, ('た', 'と') => 1224, ('た', 'は') => -939, ('っ', 'た') => 4589, ('っ', 'て') => 1647, ('っ', 'と') => -2094, ('て', 'い') => 6144, ('て', 'き') => 3640, ('て', 'く') => 2551, ('て', 'は') => -3110, ('て', 'も') => -3065, ('で', 'い') => 2666, ('で', 'き') => -1528, ('で', 'し') => -3828, ('で', 'す') => -4761, ('で', 'も') => -4203, ('と', 'い') => 1890, ('と', 'こ') => -1746, ('と', 'と') => -2279, ('と', 'の') => 720, ('と', 'み') => 5168, ('と', 'も') => -3941, ('な', 'い') => -2488, ('な', 'が') => -1313, ('な', 'ど') => -6509, ('な', 'の') => 2614, ('な', 'ん') => 3099, ('に', 'お') => -1615, ('に', 'し') => 2748, ('に', 'な') => 2454, ('に', 'よ') => -7236, ('に', '対') => -14943, ('に', '従') => -4688, ('に', '関') => -11388, ('の', 'か') => 2093, ('の', 'で') => -7059, ('の', 'に') => -6041, ('の', 'の') => -6125, ('は', 'い') => 1073, ('は', 'が') => -1033, ('は', 'ず') => -2532, ('ば', 'れ') => 1813, ('ま', 'し') => -1316, ('ま', 'で') => -6621, ('ま', 'れ') => 5409, ('め', 'て') => -3153, ('も', 'い') => 2230, ('も', 'の') => -10713, ('ら', 'か') => -944, ('ら', 'し') => -1611, ('ら', 'に') => -1897, ('り', 'し') => 651, ('り', 'ま') => 1620, ('れ', 'た') => 4270, ('れ', 'て') => 849, ('れ', 'ば') => 4114, ('ろ', 'う') => 6067, ('わ', 'れ') => 7901, ('を', '通') => -11877, ('ん', 'だ') => 728, ('ん', 'な') => -4115, ('一', '人') => 602, ('一', '方') => -1375, ('一', '日') => 970, ('一', '部') => -1051, ('上', 'が') => -4479, ('会', '社') => -1116, ('出', 'て') => 2163, ('分', 'の') => -7758, ('同', '党') => 970, ('同', '日') => -913, ('大', '阪') => -2471, ('委', '員') => -1250, ('少', 'な') => -1050, ('年', '度') => -8669, ('年', '間') => -1626, ('府', '県') => -2363, ('手', '権') => -1982, ('新', '聞') => -4066, ('日', '新') => -722, ('日', '本') => -7068, ('日', '米') => 3372, ('曜', '日') => -601, ('朝', '鮮') => -2355, ('本', '人') => -2697, ('東', '京') => -1543, ('然', 'と') => -1384, ('社', '会') => -1276, ('立', 'て') => -990, ('第', 'に') => -1612, ('米', '国') => -4268, ('１', '１') => -669, ('ｸ', 'ﾞ') => 1319,};
  static ref BW3: HashMap<(char, char), i32> = hashmap! { ('あ', 'た') => -2194, ('あ', 'り') => 719, ('あ', 'る') => 3846, ('い', '.') => -1185, ('い', '。') => -1185, ('い', 'い') => 5308, ('い', 'え') => 2079, ('い', 'く') => 3029, ('い', 'た') => 2056, ('い', 'っ') => 1883, ('い', 'る') => 5600, ('い', 'わ') => 1527, ('う', 'ち') => 1117, ('う', 'と') => 4798, ('え', 'と') => 1454, ('か', '.') => 2857, ('か', '。') => 2857, ('か', 'け') => -743, ('か', 'っ') => -4098, ('か', 'に') => -669, ('か', 'ら') => 6520, ('か', 'り') => -2670, ('が', ',') => 1816, ('が', '、') => 1816, ('が', 'き') => -4855, ('が', 'け') => -1127, ('が', 'っ') => -913, ('が', 'ら') => -4977, ('が', 'り') => -2064, ('き', 'た') => 1645, ('け', 'ど') => 1374, ('こ', 'と') => 7397, ('こ', 'の') => 1542, ('こ', 'ろ') => -2757, ('さ', 'い') => -714, ('さ', 'を') => 976, ('し', ',') => 1557, ('し', '、') => 1557, ('し', 'い') => -3714, ('し', 'た') => 3562, ('し', 'て') => 1449, ('し', 'な') => 2608, ('し', 'ま') => 1200, ('す', '.') => -1310, ('す', '。') => -1310, ('す', 'る') => 6521, ('ず', ',') => 3426, ('ず', '、') => 3426, ('ず', 'に') => 841, ('そ', 'う') => 428, ('た', '.') => 8875, ('た', '。') => 8875, ('た', 'い') => -594, ('た', 'の') => 812, ('た', 'り') => -1183, ('た', 'る') => -853, ('だ', '.') => 4098, ('だ', '。') => 4098, ('だ', 'っ') => 1004, ('っ', 'た') => -4748, ('っ', 'て') => 300, ('て', 'い') => 6240, ('て', 'お') => 855, ('て', 'も') => 302, ('で', 'す') => 1437, ('で', 'に') => -1482, ('で', 'は') => 2295, ('と', 'う') => -1387, ('と', 'し') => 2266, ('と', 'の') => 541, ('と', 'も') => -3543, ('ど', 'う') => 4664, ('な', 'い') => 1796, ('な', 'く') => -903, ('な', 'ど') => 2135, ('に', ',') => -1021, ('に', '、') => -1021, ('に', 'し') => 1771, ('に', 'な') => 1906, ('に', 'は') => 2644, ('の', ',') => -724, ('の', '、') => -724, ('の', '子') => -1000, ('は', ',') => 1337, ('は', '、') => 1337, ('べ', 'き') => 2181, ('ま', 'し') => 1113, ('ま', 'す') => 6943, ('ま', 'っ') => -1549, ('ま', 'で') => 6154, ('ま', 'れ') => -793, ('ら', 'し') => 1479, ('ら', 'れ') => 6820, ('る', 'る') => 3818, ('れ', ',') => 854, ('れ', '、') => 854, ('れ', 'た') => 1850, ('れ', 'て') => 1375, ('れ', 'ば') => -3246, ('れ', 'る') => 1091, ('わ', 'れ') => -605, ('ん', 'だ') => 606, ('ん', 'で') => 798, ('カ', '月') => 990, ('会', '議') => 860, ('入', 'り') => 1232, ('大', '会') => 2217, ('始', 'め') => 1681, ('市', ' ') => 965, ('新', '聞') => -5055, ('日', ',') => 974, ('日', '、') => 974, ('社', '会') => 2024, ('ｶ', '月') => 990, };
}

lazy_static! {
  static ref TC1: HashMap<(char, char, char), i32> = hashmap! { ('A', 'A', 'A') => 1093, ('H', 'H', 'H') => 1029, ('H', 'H', 'M') => 580, ('H', 'I', 'I') => 998, ('H', 'O', 'H') => -390, ('H', 'O', 'M') => -331, ('I', 'H', 'I') => 1169, ('I', 'O', 'H') => -142, ('I', 'O', 'I') => -1015, ('I', 'O', 'M') => 467, ('M', 'M', 'H') => 187, ('O', 'O', 'I') => -1832, };
  static ref TC2: HashMap<(char, char, char), i32> = hashmap! { ('H', 'H', 'O') => 2088, ('H', 'I', 'I') => -1023, ('H', 'M', 'M') => -1154, ('I', 'H', 'I') => -1965, ('K', 'K', 'H') => 703, ('O', 'I', 'I') => -2649, };
  static ref TC3: HashMap<(char, char, char), i32> = hashmap! { ('A', 'A', 'A') => -294, ('H', 'H', 'H') => 346, ('H', 'H', 'I') => -341, ('H', 'I', 'I') => -1088, ('H', 'I', 'K') => 731, ('H', 'O', 'H') => -1486, ('I', 'H', 'H') => 128, ('I', 'H', 'I') => -3041, ('I', 'H', 'O') => -1935, ('I', 'I', 'H') => -825, ('I', 'I', 'M') => -1035, ('I', 'O', 'I') => -542, ('K', 'H', 'H') => -1216, ('K', 'K', 'A') => 491, ('K', 'K', 'H') => -1217, ('K', 'O', 'K') => -1009, ('M', 'H', 'H') => -2694, ('M', 'H', 'M') => -457, ('M', 'H', 'O') => 123, ('M', 'M', 'H') => -471, ('N', 'N', 'H') => -1689, ('N', 'N', 'O') => 662, ('O', 'H', 'O') => -3393, };
  static ref TC4: HashMap<(char, char, char), i32> = hashmap! { ('H', 'H', 'H') => -203, ('H', 'H', 'I') => 1344, ('H', 'H', 'K') => 365, ('H', 'H', 'M') => -122, ('H', 'H', 'N') => 182, ('H', 'H', 'O') => 669, ('H', 'I', 'H') => 804, ('H', 'I', 'I') => 679, ('H', 'O', 'H') => 446, ('I', 'H', 'H') => 695, ('I', 'H', 'O') => -2324, ('I', 'I', 'H') => 321, ('I', 'I', 'I') => 1497, ('I', 'I', 'O') => 656, ('I', 'O', 'O') => 54, ('K', 'A', 'K') => 4845, ('K', 'K', 'A') => 3386, ('K', 'K', 'K') => 3065, ('M', 'H', 'H') => -405, ('M', 'H', 'I') => 201, ('M', 'M', 'H') => -241, ('M', 'M', 'M') => 661, ('M', 'O', 'M') => 841, };
  static ref TQ1: HashMap<(char, char, char, char), i32> = hashmap! { ('B', 'H', 'H', 'H') => -227, ('B', 'H', 'H', 'I') => 316, ('B', 'H', 'I', 'H') => -132, ('B', 'I', 'H', 'H') => 60, ('B', 'I', 'I', 'I') => 1595, ('B', 'N', 'H', 'H') => -744, ('B', 'O', 'H', 'H') => 225, ('B', 'O', 'O', 'O') => -908, ('O', 'A', 'K', 'K') => 482, ('O', 'H', 'H', 'H') => 281, ('O', 'H', 'I', 'H') => 249, ('O', 'I', 'H', 'I') => 200, ('O', 'I', 'I', 'H') => -68, };
  static ref TQ2: HashMap<(char, char, char, char), i32> = hashmap! { ('B', 'I', 'H', 'H') => -1401, ('B', 'I', 'I', 'I') => -1033, ('B', 'K', 'A', 'K') => -543, ('B', 'O', 'O', 'O') => -5591, };
  static ref TQ3: HashMap<(char, char, char, char), i32> = hashmap! { ('B', 'H', 'H', 'H') => 478, ('B', 'H', 'H', 'M') => -1073, ('B', 'H', 'I', 'H') => 222, ('B', 'H', 'I', 'I') => -504, ('B', 'I', 'I', 'H') => -116, ('B', 'I', 'I', 'I') => -105, ('B', 'M', 'H', 'I') => -863, ('B', 'M', 'H', 'M') => -464, ('B', 'O', 'M', 'H') => 620, ('O', 'H', 'H', 'H') => 346, ('O', 'H', 'H', 'I') => 1729, ('O', 'H', 'I', 'I') => 997, ('O', 'H', 'M', 'H') => 481, ('O', 'I', 'H', 'H') => 623, ('O', 'I', 'I', 'H') => 1344, ('O', 'K', 'A', 'K') => 2792, ('O', 'K', 'H', 'H') => 587, ('O', 'K', 'K', 'A') => 679, ('O', 'O', 'H', 'H') => 110, ('O', 'O', 'I', 'I') => -685, };
  static ref TQ4: HashMap<(char, char, char, char), i32> = hashmap! { ('B', 'H', 'H', 'H') => -721, ('B', 'H', 'H', 'M') => -3604, ('B', 'H', 'I', 'I') => -966, ('B', 'I', 'I', 'H') => -607, ('B', 'I', 'I', 'I') => -2181, ('O', 'A', 'A', 'A') => -2763, ('O', 'A', 'K', 'K') => 180, ('O', 'H', 'H', 'H') => -294, ('O', 'H', 'H', 'I') => 2446, ('O', 'H', 'H', 'O') => 480, ('O', 'H', 'I', 'H') => -1573, ('O', 'I', 'H', 'H') => 1935, ('O', 'I', 'H', 'I') => -493, ('O', 'I', 'I', 'H') => 626, ('O', 'I', 'I', 'I') => -4007, ('O', 'K', 'A', 'K') => -8156, };
  static ref TW1: HashMap<(char, char, char), i32> = hashmap! { ('に', 'つ', 'い') => -4681, ('東', '京', '都') => 2026, };
  static ref TW2: HashMap<(char, char, char), i32> = hashmap! { ('あ', 'る', '程') => -2049, ('い', 'っ', 'た') => -1256, ('こ', 'ろ', 'が') => -2434, ('し', 'ょ', 'う') => 3873, ('そ', 'の', '後') => -4430, ('だ', 'っ', 'て') => -1049, ('て', 'い', 'た') => 1833, ('と', 'し', 'て') => -4657, ('と', 'も', 'に') => -4517, ('も', 'の', 'で') => 1882, ('一', '気', 'に') => -792, ('初', 'め', 'て') => -1512, ('同', '時', 'に') => -8097, ('大', 'き', 'な') => -1255, ('対', 'し', 'て') => -2721, ('社', '会', '党') => -3216, };
  static ref TW3: HashMap<(char, char, char), i32> = hashmap! { ('い', 'た', 'だ') => -1734, ('し', 'て', 'い') => 1314, ('と', 'し', 'て') => -4314, ('に', 'つ', 'い') => -5483, ('に', 'と', 'っ') => -5989, ('に', '当', 'た') => -6247, ('の', 'で', ',') => -727, ('の', 'で', '、') => -727, ('の', 'も', 'の') => -600, ('れ', 'か', 'ら') => -3752, ('十', '二', '月') => -2287, };
  static ref TW4: HashMap<(char, char, char), i32> = hashmap! { ('い', 'う', '.') => 8576, ('い', 'う', '。') => 8576, ('か', 'ら', 'な') => -2348, ('し', 'て', 'い') => 2958, ('た', 'が', ',') => 1516, ('た', 'が', '、') => 1516, ('て', 'い', 'る') => 1538, ('と', 'い', 'う') => 1349, ('ま', 'し', 'た') => 5543, ('ま', 'せ', 'ん') => 1097, ('よ', 'う', 'と') => -4258, ('よ', 'る', 'と') => 5865, };
}

lazy_static! {
  static ref UC1: HashMap<char, i32> = hashmap! { 'A' => 484, 'K' => 93, 'M' => 645, 'O' => -505, };
  static ref UC2: HashMap<char, i32> = hashmap! { 'A' => 819, 'H' => 1059, 'I' => 409, 'M' => 3987, 'N' => 5775, 'O' => 646, };
  static ref UC3: HashMap<char, i32> = hashmap! { 'A' => -1370, 'I' => 2311, };
  static ref UC4: HashMap<char, i32> = hashmap! { 'A' => -2643, 'H' => 1809, 'I' => -1032, 'K' => -3450, 'M' => 3565, 'N' => 3876, 'O' => 6646, };
  static ref UC5: HashMap<char, i32> = hashmap! { 'H' => 313, 'I' => -1238, 'K' => -799, 'M' => 539, 'O' => -831, };
  static ref UC6: HashMap<char, i32> = hashmap! { 'H' => -506, 'I' => -253, 'K' => 87, 'M' => 247, 'O' => -387, };
  static ref UP1: HashMap<char, i32> = hashmap! { 'O' => -214, };
  static ref UP2: HashMap<char, i32> = hashmap! { 'B' => 69, 'O' => 935, };
  static ref UP3: HashMap<char, i32> = hashmap! { 'B' => 189, };
  static ref UQ1: HashMap<(char, char), i32> = hashmap! { ('B', 'H') => 21, ('B', 'I') => -12, ('B', 'K') => -99, ('B', 'N') => 142, ('B', 'O') => -56, ('O', 'H') => -95, ('O', 'I') => 477, ('O', 'K') => 410, ('O', 'O') => -2422, };
  static ref UQ2: HashMap<(char, char), i32> = hashmap! { ('B', 'H') => 216, ('B', 'I') => 113, ('O', 'K') => 1759, };
  static ref UQ3: HashMap<(char, char), i32> = hashmap! { ('B', 'A') => -479, ('B', 'H') => 42, ('B', 'I') => 1913, ('B', 'K') => -7198, ('B', 'M') => 3160, ('B', 'N') => 6427, ('B', 'O') => 14761, ('O', 'I') => -827, ('O', 'N') => -3212, };
  static ref UW1: HashMap<char, i32> = hashmap! { ',' => 156, '、' => 156, '「' => -463, 'あ' => -941, 'う' => -127, 'が' => -553, 'き' => 121, 'こ' => 505, 'で' => -201, 'と' => -547, 'ど' => -123, 'に' => -789, 'の' => -185, 'は' => -847, 'も' => -466, 'や' => -470, 'よ' => 182, 'ら' => -292, 'り' => 208, 'れ' => 169, 'を' => -446, 'ん' => -137, '・' => -135, '主' => -402, '京' => -268, '区' => -912, '午' => 871, '国' => -460, '大' => 561, '委' => 729, '市' => -411, '日' => -141, '理' => 361, '生' => -408, '県' => -386, '都' => -718, '｢' => -463, '･' => -135, };
  static ref UW2: HashMap<char, i32> = hashmap! { ',' => -829, '、' => -829, '〇' => 892, '「' => -645, '」' => 3145, 'あ' => -538, 'い' => 505, 'う' => 134, 'お' => -502, 'か' => 1454, 'が' => -856, 'く' => -412, 'こ' => 1141, 'さ' => 878, 'ざ' => 540, 'し' => 1529, 'す' => -675, 'せ' => 300, 'そ' => -1011, 'た' => 188, 'だ' => 1837, 'つ' => -949, 'て' => -291, 'で' => -268, 'と' => -981, 'ど' => 1273, 'な' => 1063, 'に' => -1764, 'の' => 130, 'は' => -409, 'ひ' => -1273, 'べ' => 1261, 'ま' => 600, 'も' => -1263, 'や' => -402, 'よ' => 1639, 'り' => -579, 'る' => -694, 'れ' => 571, 'を' => -2516, 'ん' => 2095, 'ア' => -587, 'カ' => 306, 'キ' => 568, 'ッ' => 831, '三' => -758, '不' => -2150, '世' => -302, '中' => -968, '主' => -861, '事' => 492, '人' => -123, '会' => 978, '保' => 362, '入' => 548, '初' => -3025, '副' => -1566, '北' => -3414, '区' => -422, '大' => -1769, '天' => -865, '太' => -483, '子' => -1519, '学' => 760, '実' => 1023, '小' => -2009, '市' => -813, '年' => -1060, '強' => 1067, '手' => -1519, '揺' => -1033, '政' => 1522, '文' => -1355, '新' => -1682, '日' => -1815, '明' => -1462, '最' => -630, '朝' => -1843, '本' => -1650, '東' => -931, '果' => -665, '次' => -2378, '民' => -180, '気' => -1740, '理' => 752, '発' => 529, '目' => -1584, '相' => -242, '県' => -1165, '立' => -763, '第' => 810, '米' => 509, '自' => -1353, '行' => 838, '西' => -744, '見' => -3874, '調' => 1010, '議' => 1198, '込' => 3041, '開' => 1758, '間' => -1257, '｢' => -645, '｣' => 3145, 'ｯ' => 831, 'ｱ' => -587, 'ｶ' => 306, 'ｷ' => 568, };
  static ref UW3: HashMap<char, i32> = hashmap! { ',' => 4889, '1' => -800, '−' => -1723, '、' => 4889, '々' => -2311, '〇' => 5827, '」' => 2670, '〓' => -3573, 'あ' => -2696, 'い' => 1006, 'う' => 2342, 'え' => 1983, 'お' => -4864, 'か' => -1163, 'が' => 3271, 'く' => 1004, 'け' => 388, 'げ' => 401, 'こ' => -3552, 'ご' => -3116, 'さ' => -1058, 'し' => -395, 'す' => 584, 'せ' => 3685, 'そ' => -5228, 'た' => 842, 'ち' => -521, 'っ' => -1444, 'つ' => -1081, 'て' => 6167, 'で' => 2318, 'と' => 1691, 'ど' => -899, 'な' => -2788, 'に' => 2745, 'の' => 4056, 'は' => 4555, 'ひ' => -2171, 'ふ' => -1798, 'へ' => 1199, 'ほ' => -5516, 'ま' => -4384, 'み' => -120, 'め' => 1205, 'も' => 2323, 'や' => -788, 'よ' => -202, 'ら' => 727, 'り' => 649, 'る' => 5905, 'れ' => 2773, 'わ' => -1207, 'を' => 6620, 'ん' => -518, 'ア' => 551, 'グ' => 1319, 'ス' => 874, 'ッ' => -1350, 'ト' => 521, 'ム' => 1109, 'ル' => 1591, 'ロ' => 2201, 'ン' => 278, '・' => -3794, '一' => -1619, '下' => -1759, '世' => -2087, '両' => 3815, '中' => 653, '主' => -758, '予' => -1193, '二' => 974, '人' => 2742, '今' => 792, '他' => 1889, '以' => -1368, '低' => 811, '何' => 4265, '作' => -361, '保' => -2439, '元' => 4858, '党' => 3593, '全' => 1574, '公' => -3030, '六' => 755, '共' => -1880, '円' => 5807, '再' => 3095, '分' => 457, '初' => 2475, '別' => 1129, '前' => 2286, '副' => 4437, '力' => 365, '動' => -949, '務' => -1872, '化' => 1327, '北' => -1038, '区' => 4646, '千' => -2309, '午' => -783, '協' => -1006, '口' => 483, '右' => 1233, '各' => 3588, '合' => -241, '同' => 3906, '和' => -837, '員' => 4513, '国' => 642, '型' => 1389, '場' => 1219, '外' => -241, '妻' => 2016, '学' => -1356, '安' => -423, '実' => -1008, '家' => 1078, '小' => -513, '少' => -3102, '州' => 1155, '市' => 3197, '平' => -1804, '年' => 2416, '広' => -1030, '府' => 1605, '度' => 1452, '建' => -2352, '当' => -3885, '得' => 1905, '思' => -1291, '性' => 1822, '戸' => -488, '指' => -3973, '政' => -2013, '教' => -1479, '数' => 3222, '文' => -1489, '新' => 1764, '日' => 2099, '旧' => 5792, '昨' => -661, '時' => -1248, '曜' => -951, '最' => -937, '月' => 4125, '期' => 360, '李' => 3094, '村' => 364, '東' => -805, '核' => 5156, '森' => 2438, '業' => 484, '氏' => 2613, '民' => -1694, '決' => -1073, '法' => 1868, '海' => -495, '無' => 979, '物' => 461, '特' => -3850, '生' => -273, '用' => 914, '町' => 1215, '的' => 7313, '直' => -1835, '省' => 792, '県' => 6293, '知' => -1528, '私' => 4231, '税' => 401, '立' => -960, '第' => 1201, '米' => 7767, '系' => 3066, '約' => 3663, '級' => 1384, '統' => -4229, '総' => 1163, '線' => 1255, '者' => 6457, '能' => 725, '自' => -2869, '英' => 785, '見' => 1044, '調' => -562, '財' => -733, '費' => 1777, '車' => 1835, '軍' => 1375, '込' => -1504, '通' => -1136, '選' => -681, '郎' => 1026, '郡' => 4404, '部' => 1200, '金' => 2163, '長' => 421, '開' => -1432, '間' => 1302, '関' => -1282, '雨' => 2009, '電' => -1045, '非' => 2066, '駅' => 1620, '１' => -800, '｣' => 2670, '･' => -3794, 'ｯ' => -1350, 'ｱ' => 551, 'ｽ' => 874, 'ﾄ' => 521, 'ﾑ' => 1109, 'ﾙ' => 1591, 'ﾛ' => 2201, 'ﾝ' => 278, };
  static ref UW4: HashMap<char, i32> = hashmap! { ',' => 3930, '.' => 3508, '―' => -4841, '、' => 3930, '。' => 3508, '〇' => 4999, '「' => 1895, '」' => 3798, '〓' => -5156, 'あ' => 4752, 'い' => -3435, 'う' => -640, 'え' => -2514, 'お' => 2405, 'か' => 530, 'が' => 6006, 'き' => -4482, 'ぎ' => -3821, 'く' => -3788, 'け' => -4376, 'げ' => -4734, 'こ' => 2255, 'ご' => 1979, 'さ' => 2864, 'し' => -843, 'じ' => -2506, 'す' => -731, 'ず' => 1251, 'せ' => 181, 'そ' => 4091, 'た' => 5034, 'だ' => 5408, 'ち' => -3654, 'っ' => -5882, 'つ' => -1659, 'て' => 3994, 'で' => 7410, 'と' => 4547, 'な' => 5433, 'に' => 6499, 'ぬ' => 1853, 'ね' => 1413, 'の' => 7396, 'は' => 8578, 'ば' => 1940, 'ひ' => 4249, 'び' => -4134, 'ふ' => 1345, 'へ' => 6665, 'べ' => -744, 'ほ' => 1464, 'ま' => 1051, 'み' => -2082, 'む' => -882, 'め' => -5046, 'も' => 4169, 'ゃ' => -2666, 'や' => 2795, 'ょ' => -1544, 'よ' => 3351, 'ら' => -2922, 'り' => -9726, 'る' => -14896, 'れ' => -2613, 'ろ' => -4570, 'わ' => -1783, 'を' => 13150, 'ん' => -2352, 'カ' => 2145, 'コ' => 1789, 'セ' => 1287, 'ッ' => -724, 'ト' => -403, 'メ' => -1635, 'ラ' => -881, 'リ' => -541, 'ル' => -856, 'ン' => -3637, '・' => -4371, 'ー' => -11870, '一' => -2069, '中' => 2210, '予' => 782, '事' => -190, '井' => -1768, '人' => 1036, '以' => 544, '会' => 950, '体' => -1286, '作' => 530, '側' => 4292, '先' => 601, '党' => -2006, '共' => -1212, '内' => 584, '円' => 788, '初' => 1347, '前' => 1623, '副' => 3879, '力' => -302, '動' => -740, '務' => -2715, '化' => 776, '区' => 4517, '協' => 1013, '参' => 1555, '合' => -1834, '和' => -681, '員' => -910, '器' => -851, '回' => 1500, '国' => -619, '園' => -1200, '地' => 866, '場' => -1410, '塁' => -2094, '士' => -1413, '多' => 1067, '大' => 571, '子' => -4802, '学' => -1397, '定' => -1057, '寺' => -809, '小' => 1910, '屋' => -1328, '山' => -1500, '島' => -2056, '川' => -2667, '市' => 2771, '年' => 374, '庁' => -4556, '後' => 456, '性' => 553, '感' => 916, '所' => -1566, '支' => 856, '改' => 787, '政' => 2182, '教' => 704, '文' => 522, '方' => -856, '日' => 1798, '時' => 1829, '最' => 845, '月' => -9066, '木' => -485, '来' => -442, '校' => -360, '業' => -1043, '氏' => 5388, '民' => -2716, '気' => -910, '沢' => -939, '済' => -543, '物' => -735, '率' => 672, '球' => -1267, '生' => -1286, '産' => -1101, '田' => -2900, '町' => 1826, '的' => 2586, '目' => 922, '省' => -3485, '県' => 2997, '空' => -867, '立' => -2112, '第' => 788, '米' => 2937, '系' => 786, '約' => 2171, '経' => 1146, '統' => -1169, '総' => 940, '線' => -994, '署' => 749, '者' => 2145, '能' => -730, '般' => -852, '行' => -792, '規' => 792, '警' => -1184, '議' => -244, '谷' => -1000, '賞' => 730, '車' => -1481, '軍' => 1158, '輪' => -1433, '込' => -3370, '近' => 929, '道' => -1291, '選' => 2596, '郎' => -4866, '都' => 1192, '野' => -1100, '銀' => -2213, '長' => 357, '間' => -2344, '院' => -2297, '際' => -2604, '電' => -878, '領' => -1659, '題' => -792, '館' => -1984, '首' => 1749, '高' => 2120, '｢' => 1895, '｣' => 3798, '･' => -4371, 'ｯ' => -724, 'ｰ' => -11870, 'ｶ' => 2145, 'ｺ' => 1789, 'ｾ' => 1287, 'ﾄ' => -403, 'ﾒ' => -1635, 'ﾗ' => -881, 'ﾘ' => -541, 'ﾙ' => -856, 'ﾝ' => -3637, };
  static ref UW5: HashMap<char, i32> = hashmap! { ',' => 465, '.' => -299, '1' => -514, *E2 => -32768, ']' => -2762, '、' => 465, '。' => -299, '「' => 363, 'あ' => 1655, 'い' => 331, 'う' => -503, 'え' => 1199, 'お' => 527, 'か' => 647, 'が' => -421, 'き' => 1624, 'ぎ' => 1971, 'く' => 312, 'げ' => -983, 'さ' => -1537, 'し' => -1371, 'す' => -852, 'だ' => -1186, 'ち' => 1093, 'っ' => 52, 'つ' => 921, 'て' => -18, 'で' => -850, 'と' => -127, 'ど' => 1682, 'な' => -787, 'に' => -1224, 'の' => -635, 'は' => -578, 'べ' => 1001, 'み' => 502, 'め' => 865, 'ゃ' => 3350, 'ょ' => 854, 'り' => -208, 'る' => 429, 'れ' => 504, 'わ' => 419, 'を' => -1264, 'ん' => 327, 'イ' => 241, 'ル' => 451, 'ン' => -343, '中' => -871, '京' => 722, '会' => -1153, '党' => -654, '務' => 3519, '区' => -901, '告' => 848, '員' => 2104, '大' => -1296, '学' => -548, '定' => 1785, '嵐' => -1304, '市' => -2991, '席' => 921, '年' => 1763, '思' => 872, '所' => -814, '挙' => 1618, '新' => -1682, '日' => 218, '月' => -4353, '査' => 932, '格' => 1356, '機' => -1508, '氏' => -1347, '田' => 240, '町' => -3912, '的' => -3149, '相' => 1319, '省' => -1052, '県' => -4003, '研' => -997, '社' => -278, '空' => -813, '統' => 1955, '者' => -2233, '表' => 663, '語' => -1073, '議' => 1219, '選' => -1018, '郎' => -368, '長' => 786, '間' => 1191, '題' => 2368, '館' => -689, '１' => -514, '｢' => 363, 'ｲ' => 241, 'ﾙ' => 451, 'ﾝ' => -343, };
  static ref UW6: HashMap<char, i32> = hashmap! { ',' => 227, '.' => 808, '1' => -270, *E1 => 306, '、' => 227, '。' => 808, 'あ' => -307, 'う' => 189, 'か' => 241, 'が' => -73, 'く' => -121, 'こ' => -200, 'じ' => 1782, 'す' => 383, 'た' => -428, 'っ' => 573, 'て' => -1014, 'で' => 101, 'と' => -105, 'な' => -253, 'に' => -149, 'の' => -417, 'は' => -236, 'も' => -206, 'り' => 187, 'る' => -135, 'を' => 195, 'ル' => -673, 'ン' => -496, '一' => -277, '中' => 201, '件' => -800, '会' => 624, '前' => 302, '区' => 1792, '員' => -1212, '委' => 798, '学' => -960, '市' => 887, '広' => -695, '後' => 535, '業' => -697, '相' => 753, '社' => -507, '福' => 974, '空' => -822, '者' => 1811, '連' => 463, '郎' => 1082, '１' => -270, 'ﾙ' => -673, 'ﾝ' => -496, };
}