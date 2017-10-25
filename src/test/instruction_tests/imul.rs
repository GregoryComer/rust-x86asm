use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn imul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: Some(Literal16(12206)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 210, 174, 47], OperandSize::Word)
}

fn imul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: Some(Literal16(5975)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 26, 87, 23], OperandSize::Word)
}

fn imul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: Some(Literal16(13505)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 210, 193, 52], OperandSize::Dword)
}

fn imul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand3: Some(Literal16(24644)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 26, 68, 96], OperandSize::Dword)
}

fn imul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: Some(Literal16(29151)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 245, 223, 113], OperandSize::Qword)
}

fn imul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 1570107030, Some(OperandSize::Word), None)), operand3: Some(Literal16(26655)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 60, 205, 150, 238, 149, 93, 31, 104], OperandSize::Qword)
}

fn imul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: Some(Literal32(1149559)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 229, 119, 138, 17, 0], OperandSize::Word)
}

fn imul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Memory(8987, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1458415296)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 30, 27, 35, 192, 166, 237, 86], OperandSize::Word)
}

fn imul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: Some(Literal32(1284036806)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 218, 198, 216, 136, 76], OperandSize::Dword)
}

fn imul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1591507079)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 14, 135, 120, 220, 94], OperandSize::Dword)
}

fn imul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: Some(Literal32(155947522)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 242, 2, 146, 75, 9], OperandSize::Qword)
}

fn imul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(RAX, 932389985, Some(OperandSize::Dword), None)), operand3: Some(Literal32(932766812)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 184, 97, 36, 147, 55, 92, 228, 152, 55], OperandSize::Qword)
}

fn imul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: Some(Literal32(1094308088)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 222, 248, 208, 57, 65], OperandSize::Qword)
}

fn imul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 1082929441, Some(OperandSize::Qword), None)), operand3: Some(Literal32(1496317452)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 172, 183, 33, 49, 140, 64, 12, 254, 47, 89], OperandSize::Qword)
}

fn imul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 238, 91], OperandSize::Word)
}

fn imul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Memory(6711, Some(OperandSize::Word), None)), operand3: Some(Literal8(120)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 22, 55, 26, 120], OperandSize::Word)
}

fn imul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 255, 59], OperandSize::Dword)
}

fn imul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 43, 127], OperandSize::Dword)
}

fn imul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 238, 9], OperandSize::Qword)
}

fn imul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(RCX, 719671103, Some(OperandSize::Word), None)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 177, 63, 79, 229, 42, 69], OperandSize::Qword)
}

fn imul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(17)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 213, 17], OperandSize::Word)
}

fn imul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 44, 82], OperandSize::Word)
}

fn imul_23() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 233, 36], OperandSize::Dword)
}

fn imul_24() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 2052848280, Some(OperandSize::Dword), None)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 12, 157, 152, 250, 91, 122, 29], OperandSize::Dword)
}

fn imul_25() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(67)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 226, 67], OperandSize::Qword)
}

fn imul_26() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(58)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 34, 58], OperandSize::Qword)
}

fn imul_27() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 230, 106], OperandSize::Qword)
}

fn imul_28() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSP)), operand2: Some(IndirectDisplaced(RCX, 107797345, Some(OperandSize::Qword), None)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 161, 97, 219, 108, 6, 76], OperandSize::Qword)
}

fn imul_29() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 228], OperandSize::Word)
}

fn imul_30() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 188, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 169, 188, 0], OperandSize::Word)
}

fn imul_31() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 215], OperandSize::Dword)
}

fn imul_32() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1893427259, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 60, 197, 59, 104, 219, 112], OperandSize::Dword)
}

fn imul_33() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 239], OperandSize::Qword)
}

fn imul_34() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledDisplaced(RDX, Four, 1866071545, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 60, 149, 249, 253, 57, 111], OperandSize::Qword)
}

fn imul_35() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 250], OperandSize::Word)
}

fn imul_36() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(IndirectDisplaced(BP, 28, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 102, 28], OperandSize::Word)
}

fn imul_37() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 230], OperandSize::Dword)
}

fn imul_38() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledDisplaced(EBX, Four, 1284828742, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 20, 157, 70, 238, 148, 76], OperandSize::Dword)
}

fn imul_39() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 246], OperandSize::Qword)
}

fn imul_40() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RBX, RAX, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 20, 195], OperandSize::Qword)
}

fn imul_41() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBP)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 237], OperandSize::Qword)
}

fn imul_42() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBX)), operand2: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 24], OperandSize::Qword)
}

fn imul_43() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 233], OperandSize::Word)
}

fn imul_44() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 41], OperandSize::Word)
}

fn imul_45() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 235], OperandSize::Dword)
}

fn imul_46() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(ESI, 8283291, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 174, 155, 100, 126, 0], OperandSize::Dword)
}

fn imul_47() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 234], OperandSize::Qword)
}

fn imul_48() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1750097280, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 44, 77, 128, 93, 80, 104], OperandSize::Qword)
}

fn imul_49() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 233], OperandSize::Word)
}

fn imul_50() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Memory(13408, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 46, 96, 52], OperandSize::Word)
}

fn imul_51() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 234], OperandSize::Dword)
}

fn imul_52() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 1754813568, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 172, 215, 128, 84, 152, 104], OperandSize::Dword)
}

fn imul_53() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 233], OperandSize::Qword)
}

fn imul_54() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(RSI, Eight, 693754079, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 44, 245, 223, 216, 89, 41], OperandSize::Qword)
}

fn imul_55() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 236], OperandSize::Word)
}

fn imul_56() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 101, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 105, 101], OperandSize::Word)
}

fn imul_57() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 237], OperandSize::Dword)
}

fn imul_58() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(ESI, 1814212108, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 174, 12, 174, 34, 108], OperandSize::Dword)
}

fn imul_59() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 239], OperandSize::Qword)
}

fn imul_60() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(RSI, Two, 871193887, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 44, 117, 31, 93, 237, 51], OperandSize::Qword)
}

fn imul_61() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 236], OperandSize::Qword)
}

fn imul_62() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(RDI, Four, 273725028, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 44, 189, 100, 182, 80, 16], OperandSize::Qword)
}

