use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sbb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 209], OperandSize::Word)
}

#[test]
fn sbb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 29], OperandSize::Word)
}

#[test]
fn sbb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(BL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 219], OperandSize::Dword)
}

#[test]
fn sbb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(ECX, 1769625402, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 145, 58, 87, 122, 105], OperandSize::Dword)
}

#[test]
fn sbb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 211], OperandSize::Qword)
}

#[test]
fn sbb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RDI, RCX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 20, 79], OperandSize::Qword)
}

#[test]
fn sbb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 201], OperandSize::Qword)
}

#[test]
fn sbb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 1478236917, Some(OperandSize::Byte), None)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 148, 200, 245, 26, 28, 88], OperandSize::Qword)
}

#[test]
fn sbb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 231], OperandSize::Word)
}

#[test]
fn sbb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 42], OperandSize::Word)
}

#[test]
fn sbb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 236], OperandSize::Dword)
}

#[test]
fn sbb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 58], OperandSize::Dword)
}

#[test]
fn sbb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 251], OperandSize::Qword)
}

#[test]
fn sbb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RCX, Eight, 411606913, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 164, 200, 129, 159, 136, 24], OperandSize::Qword)
}

#[test]
fn sbb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 203], OperandSize::Word)
}

#[test]
fn sbb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 32], OperandSize::Word)
}

#[test]
fn sbb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 238], OperandSize::Dword)
}

#[test]
fn sbb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(ECX, 893384525, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 145, 77, 247, 63, 53], OperandSize::Dword)
}

#[test]
fn sbb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 234], OperandSize::Qword)
}

#[test]
fn sbb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1746957658, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 20, 77, 90, 117, 32, 104], OperandSize::Qword)
}

#[test]
fn sbb_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 255], OperandSize::Qword)
}

#[test]
fn sbb_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 44, 186], OperandSize::Qword)
}

#[test]
fn sbb_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Direct(DL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 210], OperandSize::Word)
}

#[test]
fn sbb_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(IndirectDisplaced(SI, 219, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 140, 219, 0], OperandSize::Word)
}

#[test]
fn sbb_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 203], OperandSize::Dword)
}

#[test]
fn sbb_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(IndirectDisplaced(ESI, 1430189206, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 150, 150, 244, 62, 85], OperandSize::Dword)
}

#[test]
fn sbb_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 203], OperandSize::Qword)
}

#[test]
fn sbb_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Eight, 276993355, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 148, 254, 75, 149, 130, 16], OperandSize::Qword)
}

#[test]
fn sbb_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[24, 201], OperandSize::Qword)
}

#[test]
fn sbb_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[26, 12, 176], OperandSize::Qword)
}

#[test]
fn sbb_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 231], OperandSize::Word)
}

#[test]
fn sbb_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CX)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 13], OperandSize::Word)
}

#[test]
fn sbb_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 244], OperandSize::Dword)
}

#[test]
fn sbb_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Indirect(EAX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 48], OperandSize::Dword)
}

#[test]
fn sbb_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 233], OperandSize::Qword)
}

#[test]
fn sbb_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(RBX, 1425091210, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 187, 138, 42, 241, 84], OperandSize::Qword)
}

#[test]
fn sbb_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 25, 238], OperandSize::Word)
}

#[test]
fn sbb_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 8909, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 27, 147, 205, 34], OperandSize::Word)
}

#[test]
fn sbb_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 206], OperandSize::Dword)
}

#[test]
fn sbb_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1489924836, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 12, 125, 228, 114, 206, 88], OperandSize::Dword)
}

#[test]
fn sbb_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[25, 246], OperandSize::Qword)
}

#[test]
fn sbb_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledDisplaced(RBX, Four, 622501133, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[27, 44, 157, 13, 157, 26, 37], OperandSize::Qword)
}

#[test]
fn sbb_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RBX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 25, 243], OperandSize::Qword)
}

#[test]
fn sbb_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RSI)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RCX, Four, 825775994, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 27, 180, 137, 122, 87, 56, 49], OperandSize::Qword)
}

#[test]
fn sbb_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 46], OperandSize::Word)
}

#[test]
fn sbb_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 4], OperandSize::Dword)
}

#[test]
fn sbb_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AL)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[28, 29], OperandSize::Qword)
}

#[test]
fn sbb_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(23212)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 172, 90], OperandSize::Word)
}

#[test]
fn sbb_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(28713)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 41, 112], OperandSize::Dword)
}

#[test]
fn sbb_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(AX)), operand2: Some(Literal16(19164)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 220, 74], OperandSize::Qword)
}

#[test]
fn sbb_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(65354613)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 29, 117, 59, 229, 3], OperandSize::Word)
}

#[test]
fn sbb_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(413186019)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 227, 183, 160, 24], OperandSize::Dword)
}

#[test]
fn sbb_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EAX)), operand2: Some(Literal32(1539277582)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[29, 14, 131, 191, 91], OperandSize::Qword)
}

#[test]
fn sbb_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RAX)), operand2: Some(Literal32(1474158232)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 29, 152, 222, 221, 87], OperandSize::Qword)
}

#[test]
fn sbb_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 68], OperandSize::Word)
}

#[test]
fn sbb_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 28, 45], OperandSize::Word)
}

#[test]
fn sbb_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 218, 114], OperandSize::Dword)
}

#[test]
fn sbb_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(EBX, Two, 141592952, Some(OperandSize::Byte), None)), operand2: Some(Literal8(44)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 28, 93, 120, 137, 112, 8, 44], OperandSize::Dword)
}

#[test]
fn sbb_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DL)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 218, 93], OperandSize::Qword)
}

#[test]
fn sbb_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RCX, 504468101, Some(OperandSize::Byte), None)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 153, 133, 146, 17, 30, 2], OperandSize::Qword)
}

#[test]
fn sbb_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(CL)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 217, 125], OperandSize::Qword)
}

#[test]
fn sbb_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RAX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[128, 24, 21], OperandSize::Qword)
}

#[test]
fn sbb_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(BX)), operand2: Some(Literal16(17087)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 219, 191, 66], OperandSize::Word)
}

#[test]
fn sbb_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(BP, 58, Some(OperandSize::Word), None)), operand2: Some(Literal16(21398)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 94, 58, 150, 83], OperandSize::Word)
}

#[test]
fn sbb_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Literal16(27836)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 218, 188, 108], OperandSize::Dword)
}

#[test]
fn sbb_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(ECX, 1459664944, Some(OperandSize::Word), None)), operand2: Some(Literal16(6189)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 153, 48, 184, 0, 87, 45, 24], OperandSize::Dword)
}

#[test]
fn sbb_67() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SI)), operand2: Some(Literal16(20026)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 222, 58, 78], OperandSize::Qword)
}

#[test]
fn sbb_68() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(RBX, RDX, Four, Some(OperandSize::Word), None)), operand2: Some(Literal16(32311)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 28, 147, 55, 126], OperandSize::Qword)
}

#[test]
fn sbb_69() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ECX)), operand2: Some(Literal32(1823619486)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 217, 158, 57, 178, 108], OperandSize::Word)
}

#[test]
fn sbb_70() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1354151058)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 129, 29, 146, 180, 182, 80], OperandSize::Word)
}

#[test]
fn sbb_71() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Literal32(1257989176)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 219, 56, 100, 251, 74], OperandSize::Dword)
}

#[test]
fn sbb_72() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(EDX, 111516816, Some(OperandSize::Dword), None)), operand2: Some(Literal32(335986489)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 154, 144, 156, 165, 6, 57, 191, 6, 20], OperandSize::Dword)
}

#[test]
fn sbb_73() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBX)), operand2: Some(Literal32(927933687)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 219, 247, 36, 79, 55], OperandSize::Qword)
}

#[test]
fn sbb_74() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: Some(Literal32(1644873515)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[129, 26, 43, 199, 10, 98], OperandSize::Qword)
}

#[test]
fn sbb_75() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RSP)), operand2: Some(Literal32(556721503)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 220, 95, 229, 46, 33], OperandSize::Qword)
}

#[test]
fn sbb_76() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RDX, 1456979340, Some(OperandSize::Qword), None)), operand2: Some(Literal32(2024438900)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 129, 154, 140, 189, 215, 86, 116, 124, 170, 120], OperandSize::Qword)
}

#[test]
fn sbb_77() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(SP)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 220, 35], OperandSize::Word)
}

#[test]
fn sbb_78() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 26, 76], OperandSize::Word)
}

#[test]
fn sbb_79() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 218, 90], OperandSize::Dword)
}

#[test]
fn sbb_80() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Four, 1953148273, Some(OperandSize::Word), None)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 156, 182, 113, 173, 106, 116, 65], OperandSize::Dword)
}

#[test]
fn sbb_81() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(DX)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 218, 107], OperandSize::Qword)
}

#[test]
fn sbb_82() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectDisplaced(RCX, 218997489, Some(OperandSize::Word), None)), operand2: Some(Literal8(107)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 153, 241, 162, 13, 13, 107], OperandSize::Qword)
}

#[test]
fn sbb_83() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EDX)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 218, 39], OperandSize::Word)
}

#[test]
fn sbb_84() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 110, Some(OperandSize::Dword), None)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 131, 91, 110, 99], OperandSize::Word)
}

#[test]
fn sbb_85() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(ECX)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 217, 120], OperandSize::Dword)
}

#[test]
fn sbb_86() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexed(EBX, ECX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(11)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 28, 203, 11], OperandSize::Dword)
}

#[test]
fn sbb_87() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(EBP)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 221, 2], OperandSize::Qword)
}

#[test]
fn sbb_88() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledDisplaced(RCX, Two, 477185867, Some(OperandSize::Dword), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[131, 28, 77, 75, 71, 113, 28, 92], OperandSize::Qword)
}

#[test]
fn sbb_89() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(Direct(RDX)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 218, 113], OperandSize::Qword)
}

#[test]
fn sbb_90() {
    run_test(&Instruction { mnemonic: Mnemonic::SBB, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 733730896, Some(OperandSize::Qword), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 131, 156, 246, 80, 216, 187, 43, 52], OperandSize::Qword)
}

