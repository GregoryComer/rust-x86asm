use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sub_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 219], OperandSize::Word)
}

#[test]
fn sub_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(DI, 208, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 149, 208, 0], OperandSize::Word)
}

#[test]
fn sub_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 217], OperandSize::Dword)
}

#[test]
fn sub_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 1628662931, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 140, 218, 147, 108, 19, 97], OperandSize::Dword)
}

#[test]
fn sub_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 201], OperandSize::Qword)
}

#[test]
fn sub_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RCX, 78131108, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 137, 164, 47, 168, 4], OperandSize::Qword)
}

#[test]
fn sub_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 203], OperandSize::Qword)
}

#[test]
fn sub_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 20, 129], OperandSize::Qword)
}

#[test]
fn sub_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 254], OperandSize::Word)
}

#[test]
fn sub_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 237, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 152, 237, 0], OperandSize::Word)
}

#[test]
fn sub_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 238], OperandSize::Dword)
}

#[test]
fn sub_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(ESI, 1275060268, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 182, 44, 224, 255, 75], OperandSize::Dword)
}

#[test]
fn sub_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 245], OperandSize::Qword)
}

#[test]
fn sub_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 57], OperandSize::Qword)
}

#[test]
fn sub_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 246], OperandSize::Word)
}

#[test]
fn sub_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 239, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 136, 239, 0], OperandSize::Word)
}

#[test]
fn sub_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 250], OperandSize::Dword)
}

#[test]
fn sub_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(EDI, Two, 1343141473, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 60, 125, 97, 182, 14, 80], OperandSize::Dword)
}

#[test]
fn sub_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 202], OperandSize::Qword)
}

#[test]
fn sub_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 32], OperandSize::Qword)
}

#[test]
fn sub_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RDX)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 250], OperandSize::Qword)
}

#[test]
fn sub_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Four, 75129379, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 180, 183, 35, 98, 122, 4], OperandSize::Qword)
}

#[test]
fn sub_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 219], OperandSize::Word)
}

#[test]
fn sub_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 100, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 73, 100], OperandSize::Word)
}

#[test]
fn sub_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 201], OperandSize::Dword)
}

#[test]
fn sub_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(EBX, 238630581, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 139, 181, 54, 57, 14], OperandSize::Dword)
}

#[test]
fn sub_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 202], OperandSize::Qword)
}

#[test]
fn sub_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 698867637, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 12, 141, 181, 223, 167, 41], OperandSize::Qword)
}

#[test]
fn sub_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[40, 203], OperandSize::Qword)
}

#[test]
fn sub_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[42, 12, 74], OperandSize::Qword)
}

#[test]
fn sub_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 237], OperandSize::Word)
}

#[test]
fn sub_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 9000, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 187, 40, 35], OperandSize::Word)
}

#[test]
fn sub_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 234], OperandSize::Dword)
}

#[test]
fn sub_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledDisplaced(EAX, Two, 1891479691, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 28, 69, 139, 176, 189, 112], OperandSize::Dword)
}

#[test]
fn sub_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 223], OperandSize::Qword)
}

#[test]
fn sub_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BP)), operand2: Some(IndirectDisplaced(RDI, 1473424478, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 175, 94, 172, 210, 87], OperandSize::Qword)
}

#[test]
fn sub_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 41, 211], OperandSize::Word)
}

#[test]
fn sub_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 28227, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 43, 160, 67, 110], OperandSize::Word)
}

#[test]
fn sub_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 233], OperandSize::Dword)
}

#[test]
fn sub_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 785272495, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 180, 185, 175, 78, 206, 46], OperandSize::Dword)
}

#[test]
fn sub_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[41, 220], OperandSize::Qword)
}

#[test]
fn sub_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[43, 62], OperandSize::Qword)
}

#[test]
fn sub_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RSI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 41, 254], OperandSize::Qword)
}

#[test]
fn sub_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RDX)), operand2: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 43, 19], OperandSize::Qword)
}

#[test]
fn sub_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 107], OperandSize::Word)
}

#[test]
fn sub_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(116)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 116], OperandSize::Dword)
}

#[test]
fn sub_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AL)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[44, 19], OperandSize::Qword)
}

#[test]
fn sub_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(9118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 158, 35], OperandSize::Word)
}

#[test]
fn sub_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(23080)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 40, 90], OperandSize::Dword)
}

#[test]
fn sub_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(AX)), operand2: Some(Literal16(21942)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 182, 85], OperandSize::Qword)
}

#[test]
fn sub_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(154604009)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 45, 233, 17, 55, 9], OperandSize::Word)
}

#[test]
fn sub_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1375376401)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 17, 148, 250, 81], OperandSize::Dword)
}

#[test]
fn sub_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(32911606)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[45, 246, 48, 246, 1], OperandSize::Qword)
}

#[test]
fn sub_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(207914279)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 45, 39, 133, 100, 12], OperandSize::Qword)
}

#[test]
fn sub_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 233, 63], OperandSize::Word)
}

#[test]
fn sub_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 69, Some(OperandSize::Byte), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 104, 69, 38], OperandSize::Word)
}

#[test]
fn sub_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 233, 113], OperandSize::Dword)
}

#[test]
fn sub_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(EDI, EAX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 44, 199, 99], OperandSize::Dword)
}

#[test]
fn sub_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CL)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 233, 44], OperandSize::Qword)
}

#[test]
fn sub_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RDX, 198484704, Some(OperandSize::Byte), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 170, 224, 162, 212, 11, 104], OperandSize::Qword)
}

#[test]
fn sub_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DL)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 234, 14], OperandSize::Qword)
}

#[test]
fn sub_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 42, 68], OperandSize::Qword)
}

#[test]
fn sub_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(BX)), operand2: Some(Literal16(24193)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 235, 129, 94], OperandSize::Word)
}

#[test]
fn sub_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(BX, 13000, Some(OperandSize::Word), None)), operand2: Some(Literal16(24766)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 175, 200, 50, 190, 96], OperandSize::Word)
}

#[test]
fn sub_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(CX)), operand2: Some(Literal16(25274)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 233, 186, 98], OperandSize::Dword)
}

#[test]
fn sub_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Literal16(7581)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 42, 157, 29], OperandSize::Dword)
}

#[test]
fn sub_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Literal16(20947)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 234, 211, 81], OperandSize::Qword)
}

#[test]
fn sub_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal16(29200)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 44, 147, 16, 114], OperandSize::Qword)
}

#[test]
fn sub_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EBX)), operand2: Some(Literal32(2022618339)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 235, 227, 180, 142, 120], OperandSize::Word)
}

#[test]
fn sub_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1594937111)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 43, 23, 207, 16, 95], OperandSize::Word)
}

#[test]
fn sub_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Literal32(2045945105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 239, 17, 165, 242, 121], OperandSize::Dword)
}

#[test]
fn sub_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 46055509, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1524204682)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 172, 215, 85, 192, 190, 2, 138, 132, 217, 90], OperandSize::Dword)
}

#[test]
fn sub_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Literal32(1519006515)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 239, 51, 51, 138, 90], OperandSize::Qword)
}

#[test]
fn sub_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 1098891045, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1989473771)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 172, 86, 37, 191, 127, 65, 235, 245, 148, 118], OperandSize::Qword)
}

#[test]
fn sub_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RSI)), operand2: Some(Literal32(429149586)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 238, 146, 77, 148, 25], OperandSize::Qword)
}

#[test]
fn sub_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Indirect(RDI, Some(OperandSize::Qword), None)), operand2: Some(Literal32(383697996)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 47, 76, 196, 222, 22], OperandSize::Qword)
}

#[test]
fn sub_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(SP)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 236, 3], OperandSize::Word)
}

#[test]
fn sub_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(BX, 84, Some(OperandSize::Word), None)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 111, 84, 103], OperandSize::Word)
}

#[test]
fn sub_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 234, 46], OperandSize::Dword)
}

#[test]
fn sub_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 1141346065, Some(OperandSize::Word), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 44, 213, 17, 143, 7, 68, 104], OperandSize::Dword)
}

#[test]
fn sub_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(DX)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 234, 101], OperandSize::Qword)
}

#[test]
fn sub_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(RCX, 1783866875, Some(OperandSize::Word), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 169, 251, 165, 83, 106, 110], OperandSize::Qword)
}

#[test]
fn sub_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESP)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 236, 62], OperandSize::Word)
}

#[test]
fn sub_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectDisplaced(BX, 8, Some(OperandSize::Dword), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 111, 8, 92], OperandSize::Word)
}

#[test]
fn sub_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(ESI)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 238, 88], OperandSize::Dword)
}

#[test]
fn sub_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Four, 2123211828, Some(OperandSize::Dword), None)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 172, 143, 52, 164, 141, 126, 25], OperandSize::Dword)
}

#[test]
fn sub_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(EDI)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 239, 127], OperandSize::Qword)
}

#[test]
fn sub_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 44, 143, 66], OperandSize::Qword)
}

#[test]
fn sub_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(Direct(RCX)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 233, 89], OperandSize::Qword)
}

#[test]
fn sub_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SUB, operand1: Some(IndirectScaledIndexed(RAX, RAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(106)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 44, 64, 106], OperandSize::Qword)
}

