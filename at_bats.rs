pub fn run() -> Vec<Vec<i32>> {
    let f1 = 442;
    let f2 = 1045;
    let f3 = 3091;
    let f4 = 2000;
    let f5 = 000;
    let f6 = 758;
    let f7 = 2063;
    let f8 = 2394;
    let f9 = 3684;
    let f10 = 1176;
    let f11 = 278;
    let f12 = 1158;
    let f13 = 2462;
    let f14 = 2708;
    let f15 = 526;
    let f16 = 000;
    let f17 = 909;
    let f18 = 862;
    let f19 = 465;
    let f20 = 385;
    let f21 = 000;
    let f22 = 000;
    let f23 = 400;
    let f24 = 571;
    let f25 = 263;

    //Sinkers
    let si1 = 455;
    let si2 = 000;
    let si3 = 000;
    let si4 = 1250;
    let si5 = 2000;
    let si6 = 909;
    let si7 = 2222;
    let si8 = 1667;
    let si9 = 000;
    let si10 = 000;
    let si11 = 260;
    let si12 = 000;
    let si13 = 1111;
    let si14 = 526;
    let si15 = 952;
    let si16 = 278;
    let si17 = 185;
    let si18 = 000;
    let si19 = 370;
    let si20 = 476;
    let si21 = 606;
    let si22 = 1333;
    let si23 = 254;
    let si24 = 000;
    let si25 = 000;
    
    //Cutter
    let ct1 = 000;
    let ct2 = 000;
    let ct3 = 2500;
    let ct5 = 000;
    let ct6 = 000;
    let ct7 = 000;
    let ct8 = 4286;
    let ct9 = 2857;
    let ct10 = 2857;
    let ct11 = 000;
    let ct12 = 833;
    let ct13 = 2000;
    let ct14 = 3000;
    let ct15 = 1765;
    let ct16 = 2727;
    let ct17 = 000;
    let ct18 = 000;
    let ct19 = 909;
    let ct20 = 588;
    let ct21 = 556;
    let ct22 = 4000;
    let ct23 = 833;
    let ct24 = 3750;
    let ct25 = 1333;

    //Slider
    let sd1 = 000;
    let sd4 = 000;
    let sd6 = 000;
    let sd8 = 1667;
    let sd9 = 2500;
    let sd10 = 1429;
    let sd11 = 000;
    let sd12 = 000;
    let sd13 = 541;
    let sd14 = 1600;
    let sd15 = 3636;
    let sd16 = 2857;
    let sd17 = 1154;
    let sd18 = 1750;
    let sd19 = 1538;
    let sd20 = 2917;
    let sd21 = 2200;
    let sd22 = 2273;
    let sd23 = 2927;
    let sd24 = 2676;
    let sd25 = 2595;

    //Curve
    let c2 = 000;
    let c3 = 000;
    let c4 = 000;
    let c5 = 000;
    let c6 = 000;
    let c7 = 000;
    let c8 = 000;
    let c9 = 909;
    let c10 = 000;
    let c11 = 000;
    let c12 = 000;
    let c13 = 000;
    let c14 = 1111;
    let c15 = 000;
    let c16 = 000;
    let c17 = 476;
    let c18 = 1000;
    let c19 = 870;
    let c20 = 1818;
    let c21 = 1111;
    let c22 = 1818;
    let c23 = 1277;
    let c24 = 1860;
    let c25 = 2088;

    //Changeup
    let ch5 = 000;
    let ch6 = 000;
    let ch9 = 000;
    let ch10 = 2222;
    let ch11 = 769;
    let ch12 = 000;
    let ch13 = 000;
    let ch14 = 000;
    let ch15 = 3636;
    let ch16 = 1765;
    let ch17 = 714;
    let ch18 = 2857;
    let ch19 = 5000;
    let ch20 = 3333;
    let ch21 = 2258;
    let ch22 = 1707;
    let ch23 = 2364;
    let ch24 = 2500;
    let ch25 = 3548;

    //Splitter
    let sp11 = 000;
    let sp12 = 000;
    let sp17 = 000;
    let sp24 = 4444;
  
    let ab1: Vec<i32> = vec![si6, sd13, sd25, f2, si14, f15];
    let ab2: Vec<i32> = vec![sd13, f9, si21, f12, f6, f2, sd25];
    let ab3: Vec<i32> = vec![f11, sd12, si12, c17];
    let ab4: Vec<i32> = vec![si4, f5, f1,f12];
    let ab5: Vec<i32> = vec![ct15, ct25, ch17, c19, f11, c19];
    let ab6: Vec<i32> = vec![sd12, f2, f3];
    let ab7: Vec<i32> = vec![f19, f15, f12, sd23, sd22, sd19];
    let ab8: Vec<i32> = vec![f2, f11, f4, f9, ch18, sd16];
    let ab9: Vec<i32> = vec![c8, f5, ch24, f5, f8];
    let ab10: Vec<i32> = vec![si22, si16, si24, si16, si13];
    let ab11: Vec<i32> = vec![ch16, c12, si16, c23, c17, c19, c22, c13, c17];
    let ab12: Vec<i32> = vec![si1, si6, ch21, ch13];
    let ab13: Vec<i32> = vec![f19, f14, f9, sd25];
    let ab14: Vec<i32> = vec![ct25, ct23, c25, ct15];
    let ab15: Vec<i32> = vec![sd20, si9];
    let ab16: Vec<i32> = vec![f20, f2, f9];
    let ab17: Vec<i32> = vec![ct16, ch19, f5, c18];
    let ab18: Vec<i32> = vec![c25, f14];
    let ab19: Vec<i32> = vec![ct3, c25, sd15, c18];
    let ab20: Vec<i32> = vec![sd25, f3, sd24, f3, f23, sd25];
    let ab21: Vec<i32> = vec![ct20, f8, ct20, ct1];
    let ab22: Vec<i32> = vec![ct25, c24, ct14, ct20];
    let ab23: Vec<i32> = vec![f4, ct21, f7, f4, f15, f3, ct18];
    let ab24: Vec<i32> = vec![f5, ct9, ct10, c6];
    let ab25: Vec<i32> = vec![c14, ct16, f16, ct1, ch24, si12];
    let ab26: Vec<i32> = vec![ct16, ct11, ct16, ct16];
    let ab27: Vec<i32> = vec![c7, c23, c12, f24,f1];
    let ab28: Vec<i32> = vec![ct9, ct15, c21, ct8, f4];
    let ab29: Vec<i32> = vec![c4, c21, f8, f9, c13];
    let ab30: Vec<i32> = vec![c22, c5, sd21, f22, c21];
    let ab31: Vec<i32> = vec![si11, si16, si16];
    let ab32: Vec<i32> = vec![c22, c23, f2, f3];
    let ab33: Vec<i32> = vec![c7, c12, sp12, f1, c18, f20, sp11];
    let ab34: Vec<i32> = vec![f8, f9, sd25, sd22, f1, f2, sd18, sd25];
    let ab35: Vec<i32> = vec![c7, c21, f7, ct25, ct22, ct23];
    let ab36: Vec<i32> = vec![sd11, sd18];
    let ab37: Vec<i32> = vec![c25, si22, si6, si6];
    let ab38: Vec<i32> = vec![f13, si11, sd10];
    let ab39: Vec<i32> = vec![c10, sd17, f4, sd22];
    let ab40: Vec<i32> = vec![si11, ct20, si12, sd6, si19, si19];
    let ab41: Vec<i32> = vec![c8, f21, f22, c25, ct25];
    let ab42: Vec<i32> = vec![si6, si17, si6, si16, sd10, f13, f6];
    let ab43: Vec<i32> = vec![si23, si6, sd20, si12, si19];
    let ab44: Vec<i32> = vec![ct12, ct21, c21, sd23, si23, si11, sd23];
    let ab45: Vec<i32> = vec![si21, si17, ct21, si16, si15];
    let ab46: Vec<i32> = vec![f10, f9, c17];
    let ab47: Vec<i32> = vec![si19, sd21, sd23];
    let ab48: Vec<i32> = vec![f3, si14, sd22, si20];
    let ab49: Vec<i32> = vec![sd22, sd23, f13, f3, sd21, f8, si15];
    let ab50: Vec<i32> = vec![f8, sd25, sd24, f20, sd24, f18];
    let ab51: Vec<i32> = vec![f2, f1, f25, f1];
    let ab52: Vec<i32> = vec![sd12, f12, sd25, f2, f5];
    let ab53: Vec<i32> = vec![si21, si16, ct25, si7, si16, si21];
    let ab54: Vec<i32> = vec![f21, f21, f13];
    let ab55: Vec<i32> = vec![f3, f9];
    let ab56: Vec<i32> = vec![sd25, sd9, f17, f12];
    let ab57: Vec<i32> = vec![f13, c24, sd19];
    let ab58: Vec<i32> = vec![sd25, f1, sd16, sd25, sd18];
    let ab59: Vec<i32> = vec![si17, c17, c16];
    let ab60: Vec<i32> = vec![f6, si25, si18, f2, f8, sd9, f12, si22];
    let ab61: Vec<i32> = vec![si18, sd25, si23, sd20, sd25];
    let ab62: Vec<i32> = vec![c3, sd6, si16, sd19, c25];
    let ab63: Vec<i32> = vec![f11, si12, si22, ch12];
    let ab64: Vec<i32> = vec![f13, ch24, ch21, si16, f25, f22, ch22];
    let ab65: Vec<i32> = vec![c18, f13];
    let ab66: Vec<i32> = vec![f13, f3, sd18, sd21, sd24];
    let ab67: Vec<i32> = vec![si25, sd21, si20, sd22, f4, si19];
    let ab68: Vec<i32> = vec![ct6, ch19];
    let ab69: Vec<i32> = vec![f3, f3, f8, f8];
    let ab70: Vec<i32> = vec![f15, si15, f21, si18];
    let ab71: Vec<i32> = vec![f14, si15, f23, f7];
    let ab72: Vec<i32> = vec![ct14, f21, f24, sp24, ct19, ct24];
    let ab73: Vec<i32> = vec![si20, si13, si19, si2];
    let ab74: Vec<i32> = vec![sd6, sd13];
    let ab75: Vec<i32> = vec![si16, si21, si21, si17, si16, si22, f23, c12];
    let ab76: Vec<i32> = vec![f18, c20, f2, f16, c15, f15];
    let ab77: Vec<i32> = vec![f12, f8, c25, f15, f22, f23];
    let ab78: Vec<i32> = vec![f5, f15];
    let ab79: Vec<i32> = vec![ct9, ct25, ct15];
    let ab80: Vec<i32> = vec![c2, sd15, ch25, sd15, f23, ch17];
    let ab81: Vec<i32> = vec![si15, ch25, f10];
    let ab82: Vec<i32> = vec![f6, si23, sp17];
    let ab83: Vec<i32> = vec![si14, ch11, ch18, si18, c20];
    let ab84: Vec<i32> = vec![si2, ch11, c12, ch6, si11, si6];
    let ab85: Vec<i32> = vec![sd11, sd19, f6, sd25, f20];
    let ab86: Vec<i32> = vec![f19, f1, sd1, sd25, ct25];
    let ab87: Vec<i32> = vec![c25, si19, si16, si18];
    let ab88: Vec<i32> = vec![sd25, f15, sd13];
    let ab89: Vec<i32> = vec![c25, c22, si11, si21, sd25];
    let ab90: Vec<i32> = vec![c25, sd13, f1, f13];
    let ab91: Vec<i32> = vec![sd13, f8, c20];
    let ab92: Vec<i32> = vec![f4, f3, sd13, sd13, si4, sd23];
    let ab93: Vec<i32> = vec![f6, si11, si6, f9, ch23, ch23, si12];
    let ab94: Vec<i32> = vec![f15, ch16, ch23, si16, ct17, f14];
    let ab95: Vec<i32> = vec![sd13, f11, f6, ch20, f8, f3, ch23, ch23];
    let ab96: Vec<i32> = vec![f1, f7, f6, ch10, f7];
    let ab97: Vec<i32> = vec![ch5, ch14, f1, ch23, f11];
    let ab98: Vec<i32> = vec![ch24, f6, ch21, f7, ch9, f3];
    let ab99: Vec<i32> = vec![f17, ch18];
    let ab100: Vec<i32> = vec![si11, sd24, c17, c19];
    let ab101: Vec<i32> = vec![sd14, f11, sd14];
    let ab102: Vec<i32> = vec![f12, f16, sd25, sd25];
    let ab103: Vec<i32> = vec![ct12, ct9, f8];
    let ab104: Vec<i32> = vec![f3, c14, ch20, ch25];
    let ab105: Vec<i32> = vec![ct11, ct2, f2, f8];
    let ab106: Vec<i32> = vec![f15, f13, f8, c25, f8, c25, f8];
    let ab107: Vec<i32> = vec![f12, f12, ct8];
    let ab108: Vec<i32> = vec![f9, si21, si9, si16, si7, c25, f8, ch16];
    let ab109: Vec<i32> = vec![ct15, sd14, si9, si4];
    let ab110: Vec<i32> = vec![sd14, si3, sd14, sd25];
    let ab111: Vec<i32> = vec![si8, f4, f7];
    let ab112: Vec<i32> = vec![f5, ch18];
    let ab113: Vec<i32> = vec![f2, f7, f12];
    let ab114: Vec<i32> = vec![si17, si17];
    let ab115: Vec<i32> = vec![sd8, sd20, f25, ch24];
    let ab116: Vec<i32> = vec![c24, c19, f6];
    let ab117: Vec<i32> = vec![f23, f7];
    let ab118: Vec<i32> = vec![c25, c6, f6];
    let ab119: Vec<i32> = vec![f11, f8, f4];
    let ab120: Vec<i32> = vec![f5, f17, f17, f3];
    let ab121: Vec<i32> = vec![f13, f12, f16, f7];
    let ab122: Vec<i32> = vec![f7, ch17, ch23];
    let ab123: Vec<i32> = vec![ct11, si5, ct11, si9, si17, ct22];
    let ab124: Vec<i32> = vec![c19, f2, f13, c18, f16, c18];
    let ab125: Vec<i32> = vec![c25, f14];
    let ab126: Vec<i32> = vec![sd13, f11, sd24, f16, sd24];
    let ab127: Vec<i32> = vec![f2, f7];
    let ab128: Vec<i32> = vec![f1, ch15, ch15, ch24];
    let ab129: Vec<i32> = vec![f3, f10, ch13];
    let ab130: Vec<i32> = vec![f4, f19, ch22, f25, f19, f25];
    let ab131: Vec<i32> = vec![f12, f7, f6, sd21, f18, f15];
    let ab132: Vec<i32> = vec![f11, f8, f4];
    let ab133: Vec<i32> = vec![f19, f12];
    let ab134: Vec<i32> = vec![si14, si25, si18];
    let ab135: Vec<i32> = vec![c23, si25, si7];
    let ab136: Vec<i32> = vec![si23, ch17];
    let ab137: Vec<i32> = vec![c25, c25, c25, si23, si23, c17];
    let ab138: Vec<i32> = vec![ch12, ch24, ch17, si20, c25];
    let ab139: Vec<i32> = vec![ch21, ch21, si13, si13, c24];
    let ab140: Vec<i32> = vec![sd25, si15, sd4, sd25, sd20];
    let ab141: Vec<i32> = vec![sd21, si16, si18];
    let ab142: Vec<i32> = vec![ct5, ct20, c7, si21, ct15];
    let ab143: Vec<i32> = vec![ct9, f8, c23, c17, ct19];
    let ab144: Vec<i32> = vec![c8, c23];
    let ab145: Vec<i32> = vec![f19, f19, f15, c22];
    let ab146: Vec<i32> = vec![f2, ct11, ct24, f14];
    let ab147: Vec<i32> = vec![c4, c19];
    let ab148: Vec<i32> = vec![si9, c19];
    let ab149: Vec<i32> = vec![c19, si17, si14];
    let ab150: Vec<i32> = vec![sd15, sd21, sd23, sd18];
    let ab151: Vec<i32> = vec![f13, c18];
    let ab152: Vec<i32> = vec![f11, ch24, f12, c21, f11, c21];
    let ab153: Vec<i32> = vec![sd17, ch25, si11, si16, ch20];
    let ab154: Vec<i32> = vec![c2, c12, c11, ch16];
    let ab155: Vec<i32> = vec![f11, f12, sd16, sd13];
    let ab156: Vec<i32> = vec![f16, si18, f3, f6, f6, sd16];
    let ab157: Vec<i32> = vec![c11, f6];
    let ab158: Vec<i32> = vec![sd18, sd25, si23, sd25];
    let ab159: Vec<i32> = vec![f4, ct14, f4, ct15, f12];
    let ab160: Vec<i32> = vec![f6, ch14, ch10, ch20];
    let ab161: Vec<i32> = vec![f6, ch19, ct6];
    let ab162: Vec<i32> = vec![sd20, sd25];
    let ab163: Vec<i32> = vec![si20, si10, ch15];
    let ab164: Vec<i32> = vec![si7, ch25, sd9];
    let ab165: Vec<i32> = vec![si14, sd14];
    let ab166: Vec<i32> = vec![si24, si10, ct14, si16, ct20];
    let ab167: Vec<i32> = vec![f12, sd21, sd13, sd19, ch20, sd12, si20];
    let ab168: Vec<i32> = vec![c14, c25, c18];
    let ab169: Vec<i32> = vec![c18, f13];
    let ab170: Vec<i32> = vec![ch24, c11, ch17, si8, c18, c22];
    let ab171: Vec<i32> = vec![ct12, si16, c15, si11, ct24, c25];
    let ab172: Vec<i32> = vec![c9, si3, ch25, si3];
    let ab173: Vec<i32> = vec![f17, c18, ct21];
    let ab174: Vec<i32> = vec![si18, sd20, si25, ct13];
    let ab175: Vec<i32> = vec![f1, si11, si16, sd10];
    let ab176: Vec<i32> = vec![c19, si21, si11, si16, si16, sd15];
    let ab177: Vec<i32> = vec![sd1, si17, si11];
    let ab178: Vec<i32> = vec![si17, sd15, sd20, f11, sd20];
    let ab179: Vec<i32> = vec![si15, ch20];
    let ab180: Vec<i32> = vec![c21, f5, f2, f4, f8, f8];
    let ab181: Vec<i32> = vec![f7, sd21, sd21, c24, ch10, f3];
    let ab182: Vec<i32> = vec![sd20, sd14];
    let ab183: Vec<i32> = vec![sd19, sd25, f12, sd14];
    let ab184: Vec<i32> = vec![c14, c20, f25, f13, c23];
    let ab185: Vec<i32> = vec![c2, f10, f9, f20, c25];
    let ab186: Vec<i32> = vec![c20, c9, c25, c24];
    let ab187: Vec<i32> = vec![f7, f18];
    let ab188: Vec<i32> = vec![si12, sd22, sd23, sd21, sd22, sd22, si12, sd16];
    let ab189: Vec<i32> = vec![sd22, f7, si25, f2, sd17, si14];
    let ab190: Vec<i32> = vec![sd23, ch19];
    let ab191: Vec<i32> = vec![f7, ch22, f12];
    let ab192: Vec<i32> = vec![ct6, ct20, ct6, ct13];
    let ab193: Vec<i32> = vec![sd20, f6, ch17];
    let ab194: Vec<i32> = vec![c11, f20, sd25, f20, sd25, sd20, f17];
    let ab195: Vec<i32> = vec![f25, f11, f12, sd25, sd25];
    let ab196: Vec<i32> = vec![f13, si21, f19, sp17];
    let ab197: Vec<i32> = vec![ct6, ct1, ct15, ct8, ct3, ct7, c21, ct3];
    let ab198: Vec<i32> = vec![c7, f12, f4, c25, f8];
    let ab199: Vec<i32> = vec![f14, f9, f5, ct20, c25];
    let ab200: Vec<i32> = vec![ct15, f25, f14, c25, f19, f12];
    let ab201: Vec<i32> = vec![f10, f9, c17];
    let ab202: Vec<i32> = vec![sd12, f3, f6, sd22];
    let ab203: Vec<i32> = vec![f2, f1, f22, f20];
    let ab204: Vec<i32> = vec![sd24, sd9, f16, f12];
    let ab205: Vec<i32> = vec![c2, sd14, ch25, f15, f22, f23];
    let ab206: Vec<i32> = vec![ct9, ct25, ct15];
    let ab207: Vec<i32> = vec![si14, ch11, ch18, si15, c20];
    
    
    let abs = vec![ab1, ab2, ab3, ab4, ab5, ab6, ab7, ab8, ab9, ab10, ab11, ab12, ab13, ab14, ab15, ab16, ab17, ab18, ab19, ab20, ab21, ab22, ab23, ab24, ab25, ab26, ab27, ab28, ab29, ab30, ab31, ab32, ab33, ab34, ab35, ab36, ab37, ab38, ab39, ab40, ab41, ab42, ab43, ab44, ab45, ab46, ab47, ab48, ab49, ab50, ab51, ab52, ab53, ab54, ab55, ab56, ab57, ab58, ab59, ab60, ab61, ab62, ab63, ab64, ab65, ab66, ab67, ab68, ab69, ab70, ab71, ab72, ab73, ab74, ab75, ab76, ab77, ab78, ab79, ab80, ab81, ab82, ab83, ab84, ab85, ab86, ab87, ab88, ab89, ab90, ab91, ab92, ab93, ab94, ab95, ab96, ab97, ab98, ab99, ab100, 
    ab101, ab102, ab103, ab104, ab105, ab106, ab107, ab108, ab109, ab110, ab111, ab112, ab113, ab114, ab115, ab116, ab117, ab118, ab119, ab120, ab121, ab122, ab123, ab124, ab125, ab126, ab127, ab128, ab129, ab130, ab131, ab132, ab133, ab134, ab135, ab136, ab137, ab138, ab139, ab140, ab141, ab142, ab143, ab144, ab145, ab146, ab147, ab148, ab149, ab150, ab151, ab152, ab153, ab154, ab155, ab156, ab157, ab158, ab159, ab160, ab161, ab162, ab163, ab164, ab165, ab166, ab167, ab168, ab169, ab170, ab171, ab172, ab173, ab174, ab175, ab176, ab177, ab178, ab179, ab180, ab181, ab182, ab183, ab184, ab185, ab186, ab187, ab188, ab189, ab190, ab191, ab192, ab193, ab194, ab195, ab196, ab197, ab198, ab199, ab200, 
    ab201, ab202, ab203, ab204, ab205, ab206, ab207,];
    
    {
        abs
    }

}

