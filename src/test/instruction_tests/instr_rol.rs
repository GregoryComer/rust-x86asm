use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rol_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(122)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 194, 122], OperandSize::Word)
}

#[test]
fn rol_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(SI, 11345, Some(OperandSize::Byte), None)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 132, 81, 44, 23], OperandSize::Word)
}

#[test]
fn rol_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 194, 103], OperandSize::Dword)
}

#[test]
fn rol_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(ECX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 1, 44], OperandSize::Dword)
}

#[test]
fn rol_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 195, 2], OperandSize::Qword)
}

#[test]
fn rol_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RSI, 806232836, Some(OperandSize::Byte), None)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 134, 4, 35, 14, 48, 19], OperandSize::Qword)
}

#[test]
fn rol_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 193, 57], OperandSize::Qword)
}

#[test]
fn rol_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RDX, 554621371, Some(OperandSize::Byte), None)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 130, 187, 217, 14, 33, 48], OperandSize::Qword)
}

#[test]
fn rol_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DI)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 199, 74], OperandSize::Word)
}

#[test]
fn rol_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 1], OperandSize::Word)
}

#[test]
fn rol_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CX)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 193, 86], OperandSize::Dword)
}

#[test]
fn rol_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 7, 0], OperandSize::Dword)
}

#[test]
fn rol_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BP)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 197, 11], OperandSize::Qword)
}

#[test]
fn rol_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1031026401, Some(OperandSize::Word), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 132, 186, 225, 54, 116, 61, 14], OperandSize::Qword)
}

#[test]
fn rol_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 197, 54], OperandSize::Word)
}

#[test]
fn rol_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 109, Some(OperandSize::Dword), None)), operand2: Some(Literal8(83)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 66, 109, 83], OperandSize::Word)
}

#[test]
fn rol_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESP)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 196, 57], OperandSize::Dword)
}

#[test]
fn rol_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 6, 6], OperandSize::Dword)
}

#[test]
fn rol_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 197, 81], OperandSize::Qword)
}

#[test]
fn rol_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 2749855, Some(OperandSize::Dword), None)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 132, 206, 159, 245, 41, 0, 121], OperandSize::Qword)
}

#[test]
fn rol_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RDI)), operand2: Some(Literal8(72)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 199, 72], OperandSize::Qword)
}

#[test]
fn rol_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RCX, 806533361, Some(OperandSize::Qword), None)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 129, 241, 184, 18, 48, 28], OperandSize::Qword)
}

#[test]
fn rol_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 194], OperandSize::Word)
}

#[test]
fn rol_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(BX, 16, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 71, 16], OperandSize::Word)
}

#[test]
fn rol_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 193], OperandSize::Dword)
}

#[test]
fn rol_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1418031355, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 4, 141, 251, 112, 133, 84], OperandSize::Dword)
}

#[test]
fn rol_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 195], OperandSize::Qword)
}

#[test]
fn rol_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Two, 1076803977, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 132, 88, 137, 185, 46, 64], OperandSize::Qword)
}

#[test]
fn rol_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 194], OperandSize::Qword)
}

#[test]
fn rol_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 4, 80], OperandSize::Qword)
}

#[test]
fn rol_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 195], OperandSize::Word)
}

#[test]
fn rol_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 4, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 67, 4], OperandSize::Word)
}

#[test]
fn rol_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 197], OperandSize::Dword)
}

#[test]
fn rol_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ECX, Two, 2099015609, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 132, 72, 185, 111, 28, 125], OperandSize::Dword)
}

#[test]
fn rol_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 199], OperandSize::Qword)
}

#[test]
fn rol_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 3], OperandSize::Qword)
}

#[test]
fn rol_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 197], OperandSize::Word)
}

#[test]
fn rol_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Memory(4536, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 6, 184, 17], OperandSize::Word)
}

#[test]
fn rol_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 198], OperandSize::Dword)
}

#[test]
fn rol_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(EBX, Four, 621949669, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 4, 157, 229, 50, 18, 37], OperandSize::Dword)
}

#[test]
fn rol_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 194], OperandSize::Qword)
}

#[test]
fn rol_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RAX, Eight, 820382679, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 132, 192, 215, 11, 230, 48], OperandSize::Qword)
}

#[test]
fn rol_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RCX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 193], OperandSize::Qword)
}

#[test]
fn rol_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RCX, 16120383, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 129, 63, 250, 245, 0], OperandSize::Qword)
}

#[test]
fn rol_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Word)
}

#[test]
fn rol_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 1, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 67, 1], OperandSize::Word)
}

#[test]
fn rol_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Dword)
}

#[test]
fn rol_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(ECX, Eight, 1172628517, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 4, 205, 37, 228, 228, 69], OperandSize::Dword)
}

#[test]
fn rol_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 195], OperandSize::Qword)
}

#[test]
fn rol_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(RBX, Four, 2091947488, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 4, 157, 224, 149, 176, 124], OperandSize::Qword)
}

#[test]
fn rol_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Qword)
}

#[test]
fn rol_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 4, 82], OperandSize::Qword)
}

#[test]
fn rol_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 197], OperandSize::Word)
}

#[test]
fn rol_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(BX, 141, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 135, 141, 0], OperandSize::Word)
}

#[test]
fn rol_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 193], OperandSize::Dword)
}

#[test]
fn rol_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1655603762, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 4, 141, 50, 130, 174, 98], OperandSize::Dword)
}

#[test]
fn rol_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 196], OperandSize::Qword)
}

#[test]
fn rol_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RDX, 790667967, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 130, 191, 162, 32, 47], OperandSize::Qword)
}

#[test]
fn rol_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 194], OperandSize::Word)
}

#[test]
fn rol_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 49, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 65, 49], OperandSize::Word)
}

#[test]
fn rol_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EBX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 195], OperandSize::Dword)
}

#[test]
fn rol_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(EDI, 2066747419, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 135, 27, 16, 48, 123], OperandSize::Dword)
}

#[test]
fn rol_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 196], OperandSize::Qword)
}

#[test]
fn rol_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RCX, 173298781, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 129, 93, 84, 84, 10], OperandSize::Qword)
}

#[test]
fn rol_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RSI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 198], OperandSize::Qword)
}

#[test]
fn rol_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RAX, RDX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 4, 144], OperandSize::Qword)
}

