use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn rol_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(47)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 194, 47], OperandSize::Word)
}

#[test]
fn rol_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 254, Some(OperandSize::Byte), None)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 129, 254, 0, 28], OperandSize::Word)
}

#[test]
fn rol_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 193, 79], OperandSize::Dword)
}

#[test]
fn rol_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(EAX, 327312456, Some(OperandSize::Byte), None)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 128, 72, 100, 130, 19, 3], OperandSize::Dword)
}

#[test]
fn rol_5() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 194, 115], OperandSize::Qword)
}

#[test]
fn rol_6() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 839894566, Some(OperandSize::Byte), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 132, 151, 38, 198, 15, 50, 38], OperandSize::Qword)
}

#[test]
fn rol_7() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 193, 50], OperandSize::Qword)
}

#[test]
fn rol_8() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RBX, 1073779092, Some(OperandSize::Byte), None)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 131, 148, 145, 0, 64, 12], OperandSize::Qword)
}

#[test]
fn rol_9() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DX)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 194, 56], OperandSize::Word)
}

#[test]
fn rol_10() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(DI, 233, Some(OperandSize::Word), None)), operand2: Some(Literal8(57)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 133, 233, 0, 57], OperandSize::Word)
}

#[test]
fn rol_11() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BX)), operand2: Some(Literal8(20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 195, 20], OperandSize::Dword)
}

#[test]
fn rol_12() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EBX, Four, 1258139280, Some(OperandSize::Word), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 132, 152, 144, 174, 253, 74, 115], OperandSize::Dword)
}

#[test]
fn rol_13() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BX)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 195, 107], OperandSize::Qword)
}

#[test]
fn rol_14() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 438998223, Some(OperandSize::Word), None)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 132, 217, 207, 148, 42, 26, 0], OperandSize::Qword)
}

#[test]
fn rol_15() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDX)), operand2: Some(Literal8(0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 194, 0], OperandSize::Word)
}

#[test]
fn rol_16() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(27)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 3, 27], OperandSize::Word)
}

#[test]
fn rol_17() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(74)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 199, 74], OperandSize::Dword)
}

#[test]
fn rol_18() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 3, 75], OperandSize::Dword)
}

#[test]
fn rol_19() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESI)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 198, 123], OperandSize::Qword)
}

#[test]
fn rol_20() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RSI, 1893949094, Some(OperandSize::Dword), None)), operand2: Some(Literal8(100)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 134, 166, 94, 227, 112, 100], OperandSize::Qword)
}

#[test]
fn rol_21() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RSP)), operand2: Some(Literal8(56)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 196, 56], OperandSize::Qword)
}

#[test]
fn rol_22() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(RCX, Four, 204717170, Some(OperandSize::Qword), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 4, 141, 114, 188, 51, 12, 92], OperandSize::Qword)
}

#[test]
fn rol_23() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 193], OperandSize::Word)
}

#[test]
fn rol_24() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 126, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 67, 126], OperandSize::Word)
}

#[test]
fn rol_25() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 193], OperandSize::Dword)
}

#[test]
fn rol_26() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 200205904, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 132, 128, 80, 230, 238, 11], OperandSize::Dword)
}

#[test]
fn rol_27() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 193], OperandSize::Qword)
}

#[test]
fn rol_28() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 2], OperandSize::Qword)
}

#[test]
fn rol_29() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 193], OperandSize::Qword)
}

#[test]
fn rol_30() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 4, 186], OperandSize::Qword)
}

#[test]
fn rol_31() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 193], OperandSize::Word)
}

#[test]
fn rol_32() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 5], OperandSize::Word)
}

#[test]
fn rol_33() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 195], OperandSize::Dword)
}

#[test]
fn rol_34() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 3], OperandSize::Dword)
}

#[test]
fn rol_35() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 199], OperandSize::Qword)
}

#[test]
fn rol_36() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RDI, 1528224640, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 135, 128, 219, 22, 91], OperandSize::Qword)
}

#[test]
fn rol_37() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 195], OperandSize::Word)
}

#[test]
fn rol_38() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 1], OperandSize::Word)
}

#[test]
fn rol_39() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 199], OperandSize::Dword)
}

#[test]
fn rol_40() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(ECX, 2128586535, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 129, 39, 167, 223, 126], OperandSize::Dword)
}

#[test]
fn rol_41() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 195], OperandSize::Qword)
}

#[test]
fn rol_42() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(RDI, Four, 292166003, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 4, 189, 115, 25, 106, 17], OperandSize::Qword)
}

#[test]
fn rol_43() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RDI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 199], OperandSize::Qword)
}

#[test]
fn rol_44() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 542816590, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 132, 147, 78, 185, 90, 32], OperandSize::Qword)
}

#[test]
fn rol_45() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Word)
}

#[test]
fn rol_46() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(DI, 7611, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 133, 187, 29], OperandSize::Word)
}

#[test]
fn rol_47() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Dword)
}

#[test]
fn rol_48() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(EDX, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 2], OperandSize::Dword)
}

#[test]
fn rol_49() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Qword)
}

#[test]
fn rol_50() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexed(RDI, RDI, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 4, 127], OperandSize::Qword)
}

#[test]
fn rol_51() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 194], OperandSize::Qword)
}

#[test]
fn rol_52() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1346080839, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 132, 114, 71, 144, 59, 80], OperandSize::Qword)
}

#[test]
fn rol_53() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 194], OperandSize::Word)
}

#[test]
fn rol_54() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 1, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 64, 1], OperandSize::Word)
}

#[test]
fn rol_55() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 193], OperandSize::Dword)
}

#[test]
fn rol_56() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 1], OperandSize::Dword)
}

#[test]
fn rol_57() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 194], OperandSize::Qword)
}

#[test]
fn rol_58() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(RCX, Eight, 2127047376, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 4, 205, 208, 42, 200, 126], OperandSize::Qword)
}

#[test]
fn rol_59() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 196], OperandSize::Word)
}

#[test]
fn rol_60() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 29519, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 130, 79, 115], OperandSize::Word)
}

#[test]
fn rol_61() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(EDI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 199], OperandSize::Dword)
}

#[test]
fn rol_62() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 7], OperandSize::Dword)
}

#[test]
fn rol_63() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(ECX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 193], OperandSize::Qword)
}

#[test]
fn rol_64() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 100910660, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 4, 221, 68, 198, 3, 6], OperandSize::Qword)
}

#[test]
fn rol_65() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(Direct(RSI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 198], OperandSize::Qword)
}

#[test]
fn rol_66() {
    run_test(&Instruction { mnemonic: Mnemonic::ROL, operand1: Some(IndirectDisplaced(RCX, 807910270, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 129, 126, 187, 39, 48], OperandSize::Qword)
}

