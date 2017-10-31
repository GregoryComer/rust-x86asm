use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn imul_1() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: Some(Literal16(31002)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 250, 26, 121], OperandSize::Word)
}

#[test]
fn imul_2() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 5311, Some(OperandSize::Word), None)), operand3: Some(Literal16(11908)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 139, 191, 20, 132, 46], OperandSize::Word)
}

#[test]
fn imul_3() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(Direct(SI)), operand3: Some(Literal16(29281)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 254, 97, 114], OperandSize::Dword)
}

#[test]
fn imul_4() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: Some(Literal16(13884)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 48, 60, 54], OperandSize::Dword)
}

#[test]
fn imul_5() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: Some(Literal16(16224)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 233, 96, 63], OperandSize::Qword)
}

#[test]
fn imul_6() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(IndirectDisplaced(RDX, 152053889, Some(OperandSize::Word), None)), operand3: Some(Literal16(23076)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 146, 129, 40, 16, 9, 36, 90], OperandSize::Qword)
}

#[test]
fn imul_7() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: Some(Literal32(95564176)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 225, 144, 49, 178, 5], OperandSize::Word)
}

#[test]
fn imul_8() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1994116771)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 105, 8, 163, 206, 219, 118], OperandSize::Word)
}

#[test]
fn imul_9() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: Some(Literal32(1010068240)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 207, 16, 107, 52, 60], OperandSize::Dword)
}

#[test]
fn imul_10() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: Some(Literal32(2085192243)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 24, 51, 130, 73, 124], OperandSize::Dword)
}

#[test]
fn imul_11() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: Some(Literal32(2139254198)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 219, 182, 109, 130, 127], OperandSize::Qword)
}

#[test]
fn imul_12() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand3: Some(Literal32(1273229760)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[105, 11, 192, 241, 227, 75], OperandSize::Qword)
}

#[test]
fn imul_13() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSP)), operand3: Some(Literal32(1891281772)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 252, 108, 171, 186, 112], OperandSize::Qword)
}

#[test]
fn imul_14() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RCX)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 747430971, Some(OperandSize::Qword), None)), operand3: Some(Literal32(1184087309)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 105, 12, 213, 59, 228, 140, 44, 13, 189, 147, 70], OperandSize::Qword)
}

#[test]
fn imul_15() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: Some(Literal8(53)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 245, 53], OperandSize::Word)
}

#[test]
fn imul_16() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(SI, 174, Some(OperandSize::Word), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 140, 174, 0, 44], OperandSize::Word)
}

#[test]
fn imul_17() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: Some(Literal8(50)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 235, 50], OperandSize::Dword)
}

#[test]
fn imul_18() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(EBX, 1942894269, Some(OperandSize::Word), None)), operand3: Some(Literal8(9)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 139, 189, 54, 206, 115, 9], OperandSize::Dword)
}

#[test]
fn imul_19() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: Some(Literal8(103)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 203, 103], OperandSize::Qword)
}

#[test]
fn imul_20() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Eight, Some(OperandSize::Word), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 60, 249, 27], OperandSize::Qword)
}

#[test]
fn imul_21() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(127)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 246, 127], OperandSize::Word)
}

#[test]
fn imul_22() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 33, Some(OperandSize::Dword), None)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 107, 99, 33, 27], OperandSize::Word)
}

#[test]
fn imul_23() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 234, 110], OperandSize::Dword)
}

#[test]
fn imul_24() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 33, 104], OperandSize::Dword)
}

#[test]
fn imul_25() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 220, 55], OperandSize::Qword)
}

#[test]
fn imul_26() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1735743406, Some(OperandSize::Dword), None)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[107, 172, 113, 174, 87, 117, 103, 77], OperandSize::Qword)
}

#[test]
fn imul_27() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 243, 78], OperandSize::Qword)
}

#[test]
fn imul_28() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 107, 17, 81], OperandSize::Qword)
}

#[test]
fn imul_29() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 207], OperandSize::Word)
}

#[test]
fn imul_30() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SP)), operand2: Some(Indirect(BX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 39], OperandSize::Word)
}

#[test]
fn imul_31() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 223], OperandSize::Dword)
}

#[test]
fn imul_32() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1904621527, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 20, 125, 215, 55, 134, 113], OperandSize::Dword)
}

#[test]
fn imul_33() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 246], OperandSize::Qword)
}

#[test]
fn imul_34() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CX)), operand2: Some(IndirectScaledDisplaced(RSI, Two, 1959932682, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 12, 117, 10, 51, 210, 116], OperandSize::Qword)
}

#[test]
fn imul_35() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 253], OperandSize::Word)
}

#[test]
fn imul_36() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(DI, 188, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 175, 181, 188, 0], OperandSize::Word)
}

#[test]
fn imul_37() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 218], OperandSize::Dword)
}

#[test]
fn imul_38() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 339173455, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 180, 94, 79, 96, 55, 20], OperandSize::Dword)
}

#[test]
fn imul_39() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 215], OperandSize::Qword)
}

#[test]
fn imul_40() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 142340643, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 175, 172, 131, 35, 242, 123, 8], OperandSize::Qword)
}

#[test]
fn imul_41() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBP)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 238], OperandSize::Qword)
}

#[test]
fn imul_42() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 134851273, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 175, 148, 119, 201, 170, 9, 8], OperandSize::Qword)
}

#[test]
fn imul_43() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 235], OperandSize::Word)
}

#[test]
fn imul_44() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 42], OperandSize::Word)
}

#[test]
fn imul_45() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 235], OperandSize::Dword)
}

#[test]
fn imul_46() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 2061661645, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 172, 66, 205, 117, 226, 122], OperandSize::Dword)
}

#[test]
fn imul_47() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 233], OperandSize::Qword)
}

#[test]
fn imul_48() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[246, 46], OperandSize::Qword)
}

#[test]
fn imul_49() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 234], OperandSize::Word)
}

#[test]
fn imul_50() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 42], OperandSize::Word)
}

#[test]
fn imul_51() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 239], OperandSize::Dword)
}

#[test]
fn imul_52() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(EAX, 523482516, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 168, 148, 181, 51, 31], OperandSize::Dword)
}

#[test]
fn imul_53() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(DI)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 239], OperandSize::Qword)
}

#[test]
fn imul_54() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1526356204, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 44, 213, 236, 88, 250, 90], OperandSize::Qword)
}

#[test]
fn imul_55() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 237], OperandSize::Word)
}

#[test]
fn imul_56() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(BP, 86, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 247, 110, 86], OperandSize::Word)
}

#[test]
fn imul_57() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(EBX)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 235], OperandSize::Dword)
}

#[test]
fn imul_58() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(ECX, 45299187, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 169, 243, 53, 179, 2], OperandSize::Dword)
}

#[test]
fn imul_59() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(ESP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 236], OperandSize::Qword)
}

#[test]
fn imul_60() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectDisplaced(RBX, 1474316490, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[247, 171, 202, 72, 224, 87], OperandSize::Qword)
}

#[test]
fn imul_61() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(Direct(RBP)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 237], OperandSize::Qword)
}

#[test]
fn imul_62() {
    run_test(&Instruction { mnemonic: Mnemonic::IMUL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Four, 910989207, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 247, 172, 144, 151, 151, 76, 54], OperandSize::Qword)
}

