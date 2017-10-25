use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn xor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 217], OperandSize::Word)
}

#[test]
fn xor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(SI, 30492, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 156, 28, 119], OperandSize::Word)
}

#[test]
fn xor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 202], OperandSize::Dword)
}

#[test]
fn xor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 28, 127], OperandSize::Dword)
}

#[test]
fn xor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 210], OperandSize::Qword)
}

#[test]
fn xor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 11], OperandSize::Qword)
}

#[test]
fn xor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 202], OperandSize::Qword)
}

#[test]
fn xor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RBX, Two, 1106973500, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 12, 93, 60, 19, 251, 65], OperandSize::Qword)
}

#[test]
fn xor_9() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 238], OperandSize::Word)
}

#[test]
fn xor_10() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 13329, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 163, 17, 52], OperandSize::Word)
}

#[test]
fn xor_11() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 222], OperandSize::Dword)
}

#[test]
fn xor_12() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 2035416369, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 52, 205, 49, 253, 81, 121], OperandSize::Dword)
}

#[test]
fn xor_13() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 239], OperandSize::Qword)
}

#[test]
fn xor_14() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 931084297, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 36, 125, 9, 56, 127, 55], OperandSize::Qword)
}

#[test]
fn xor_15() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 213], OperandSize::Word)
}

#[test]
fn xor_16() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 25607, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 177, 7, 100], OperandSize::Word)
}

#[test]
fn xor_17() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 238], OperandSize::Dword)
}

#[test]
fn xor_18() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(ESI, Two, 187923838, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 44, 117, 126, 125, 51, 11], OperandSize::Dword)
}

#[test]
fn xor_19() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 230], OperandSize::Qword)
}

#[test]
fn xor_20() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RSI, 449768149, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 166, 213, 234, 206, 26], OperandSize::Qword)
}

#[test]
fn xor_21() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 250], OperandSize::Qword)
}

#[test]
fn xor_22() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RDX, Two, 1974042280, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 52, 85, 168, 126, 169, 117], OperandSize::Qword)
}

#[test]
fn xor_23() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 209], OperandSize::Word)
}

#[test]
fn xor_24() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(BP, 4486, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 150, 134, 17], OperandSize::Word)
}

#[test]
fn xor_25() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 201], OperandSize::Dword)
}

#[test]
fn xor_26() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(EDI, 71109795, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 143, 163, 12, 61, 4], OperandSize::Dword)
}

#[test]
fn xor_27() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 218], OperandSize::Qword)
}

#[test]
fn xor_28() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(RAX, 2080146081, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 136, 161, 130, 252, 123], OperandSize::Qword)
}

#[test]
fn xor_29() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[48, 201], OperandSize::Qword)
}

#[test]
fn xor_30() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledDisplaced(RBX, Two, 778176445, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[50, 12, 93, 189, 7, 98, 46], OperandSize::Qword)
}

#[test]
fn xor_31() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 252], OperandSize::Word)
}

#[test]
fn xor_32() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 53], OperandSize::Word)
}

#[test]
fn xor_33() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 239], OperandSize::Dword)
}

#[test]
fn xor_34() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledDisplaced(EDI, Four, 763135992, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 44, 189, 248, 135, 124, 45], OperandSize::Dword)
}

#[test]
fn xor_35() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 252], OperandSize::Qword)
}

#[test]
fn xor_36() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BP)), operand2: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 41], OperandSize::Qword)
}

#[test]
fn xor_37() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 49, 205], OperandSize::Word)
}

#[test]
fn xor_38() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 9154, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 51, 169, 194, 35], OperandSize::Word)
}

#[test]
fn xor_39() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 210], OperandSize::Dword)
}

#[test]
fn xor_40() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 1319653473, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 140, 255, 97, 80, 168, 78], OperandSize::Dword)
}

#[test]
fn xor_41() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[49, 234], OperandSize::Qword)
}

#[test]
fn xor_42() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Two, 1803843280, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[51, 172, 75, 208, 118, 132, 107], OperandSize::Qword)
}

#[test]
fn xor_43() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RCX)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 49, 209], OperandSize::Qword)
}

#[test]
fn xor_44() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RBP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 1546141757, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 51, 44, 141, 61, 64, 40, 92], OperandSize::Qword)
}

#[test]
fn xor_45() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 0], OperandSize::Word)
}

#[test]
fn xor_46() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 6], OperandSize::Dword)
}

#[test]
fn xor_47() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AL)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[52, 92], OperandSize::Qword)
}

#[test]
fn xor_48() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(1036)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 12, 4], OperandSize::Word)
}

#[test]
fn xor_49() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(26658)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 34, 104], OperandSize::Dword)
}

#[test]
fn xor_50() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(AX)), operand2: Some(Literal16(29690)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 250, 115], OperandSize::Qword)
}

#[test]
fn xor_51() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(220092539)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 53, 123, 88, 30, 13], OperandSize::Word)
}

#[test]
fn xor_52() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(276919878)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 70, 118, 129, 16], OperandSize::Dword)
}

#[test]
fn xor_53() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EAX)), operand2: Some(Literal32(987938253)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[53, 205, 189, 226, 58], OperandSize::Qword)
}

#[test]
fn xor_54() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1608947212)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 53, 12, 150, 230, 95], OperandSize::Qword)
}

#[test]
fn xor_55() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CL)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 241, 107], OperandSize::Word)
}

#[test]
fn xor_56() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 19438, Some(OperandSize::Byte), None)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 177, 238, 75, 49], OperandSize::Word)
}

#[test]
fn xor_57() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 243, 93], OperandSize::Dword)
}

#[test]
fn xor_58() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(EAX, ESI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 52, 240, 16], OperandSize::Dword)
}

#[test]
fn xor_59() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BL)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 243, 28], OperandSize::Qword)
}

#[test]
fn xor_60() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 51, 73], OperandSize::Qword)
}

#[test]
fn xor_61() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DL)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 242, 90], OperandSize::Qword)
}

#[test]
fn xor_62() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RDI, 734754415, Some(OperandSize::Byte), None)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 183, 111, 118, 203, 43, 19], OperandSize::Qword)
}

#[test]
fn xor_63() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Literal16(21645)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 246, 141, 84], OperandSize::Word)
}

#[test]
fn xor_64() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 80, Some(OperandSize::Word), None)), operand2: Some(Literal16(4590)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 115, 80, 238, 17], OperandSize::Word)
}

#[test]
fn xor_65() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(BX)), operand2: Some(Literal16(29264)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 243, 80, 114], OperandSize::Dword)
}

#[test]
fn xor_66() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(EAX, EDX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal16(16932)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 52, 80, 36, 66], OperandSize::Dword)
}

#[test]
fn xor_67() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SI)), operand2: Some(Literal16(25424)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 246, 80, 99], OperandSize::Qword)
}

#[test]
fn xor_68() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 262339114, Some(OperandSize::Word), None)), operand2: Some(Literal16(5588)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 52, 205, 42, 250, 162, 15, 212, 21], OperandSize::Qword)
}

#[test]
fn xor_69() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(Literal32(1659629823)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 241, 255, 240, 235, 98], OperandSize::Word)
}

#[test]
fn xor_70() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(BP, 24716, Some(OperandSize::Dword), None)), operand2: Some(Literal32(620510968)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 182, 140, 96, 248, 62, 252, 36], OperandSize::Word)
}

#[test]
fn xor_71() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESP)), operand2: Some(Literal32(318692109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 244, 13, 219, 254, 18], OperandSize::Dword)
}

#[test]
fn xor_72() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Dword), None)), operand2: Some(Literal32(612348647)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 52, 120, 231, 178, 127, 36], OperandSize::Dword)
}

#[test]
fn xor_73() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ECX)), operand2: Some(Literal32(118799866)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 241, 250, 189, 20, 7], OperandSize::Qword)
}

#[test]
fn xor_74() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 1706719558, Some(OperandSize::Dword), None)), operand2: Some(Literal32(2005754482)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 180, 192, 70, 121, 186, 101, 114, 98, 141, 119], OperandSize::Qword)
}

#[test]
fn xor_75() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RSP)), operand2: Some(Literal32(522254096)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 244, 16, 247, 32, 31], OperandSize::Qword)
}

#[test]
fn xor_76() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand2: Some(Literal32(1049274228)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 50, 116, 167, 138, 62], OperandSize::Qword)
}

#[test]
fn xor_77() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(DI)), operand2: Some(Literal8(60)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 247, 60], OperandSize::Word)
}

#[test]
fn xor_78() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 52, 57], OperandSize::Word)
}

#[test]
fn xor_79() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(CX)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 241, 87], OperandSize::Dword)
}

#[test]
fn xor_80() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Eight, 186788713, Some(OperandSize::Word), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 180, 255, 105, 43, 34, 11, 125], OperandSize::Dword)
}

#[test]
fn xor_81() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(SP)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 244, 0], OperandSize::Qword)
}

#[test]
fn xor_82() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectDisplaced(RDI, 1427589842, Some(OperandSize::Word), None)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 183, 210, 74, 23, 85, 22], OperandSize::Qword)
}

#[test]
fn xor_83() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 246, 9], OperandSize::Word)
}

#[test]
fn xor_84() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 50, 118], OperandSize::Word)
}

#[test]
fn xor_85() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 245, 53], OperandSize::Dword)
}

#[test]
fn xor_86() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexed(ECX, EDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 52, 209, 99], OperandSize::Dword)
}

#[test]
fn xor_87() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 243, 93], OperandSize::Qword)
}

#[test]
fn xor_88() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledDisplaced(RAX, Four, 104948535, Some(OperandSize::Dword), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 52, 133, 55, 99, 65, 6, 81], OperandSize::Qword)
}

#[test]
fn xor_89() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(Direct(RDI)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 247, 109], OperandSize::Qword)
}

#[test]
fn xor_90() {
    run_test(&Instruction { mnemonic: Mnemonic::XOR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 391230205, Some(OperandSize::Qword), None)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 180, 80, 253, 178, 81, 23, 126], OperandSize::Qword)
}

