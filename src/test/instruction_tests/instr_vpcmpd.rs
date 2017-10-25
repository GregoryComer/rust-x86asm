use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: Some(Literal8(62)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 243, 85, 9, 31, 239, 62], OperandSize::Dword)
}

#[test]
fn vpcmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(119)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 243, 93, 13, 31, 39, 119], OperandSize::Dword)
}

#[test]
fn vpcmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 871944229, Some(OperandSize::Dword), None)), operand4: Some(Literal8(118)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 77, 30, 31, 20, 93, 37, 208, 248, 51, 118], OperandSize::Dword)
}

#[test]
fn vpcmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM20)), operand4: Some(Literal8(60)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 179, 21, 11, 31, 244, 60], OperandSize::Qword)
}

#[test]
fn vpcmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 275383410, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(96)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 243, 21, 2, 31, 36, 181, 114, 4, 106, 16, 96], OperandSize::Qword)
}

#[test]
fn vpcmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM19)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 101, 23, 31, 23, 77], OperandSize::Qword)
}

#[test]
fn vpcmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: Some(Literal8(1)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 125, 44, 31, 222, 1], OperandSize::Dword)
}

#[test]
fn vpcmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 1419932308, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 243, 109, 44, 31, 12, 117, 148, 114, 162, 84, 77], OperandSize::Dword)
}

#[test]
fn vpcmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K5)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 213146301, Some(OperandSize::Dword), None)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 93, 61, 31, 44, 133, 189, 90, 180, 12, 124], OperandSize::Dword)
}

#[test]
fn vpcmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM14)), operand4: Some(Literal8(78)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 211, 85, 44, 31, 214, 78], OperandSize::Qword)
}

#[test]
fn vpcmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 1302661498, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(109)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 47, 31, 164, 113, 122, 9, 165, 77, 109], OperandSize::Qword)
}

#[test]
fn vpcmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RBX, Two, 310247127, Some(OperandSize::Dword), None)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 13, 50, 31, 164, 95, 215, 254, 125, 18, 114], OperandSize::Qword)
}

#[test]
fn vpcmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K4)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 101, 78, 31, 227, 52], OperandSize::Dword)
}

#[test]
fn vpcmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDI, Four, 1880271646, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 93, 75, 31, 172, 190, 30, 171, 18, 112, 68], OperandSize::Dword)
}

#[test]
fn vpcmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Eight, 733388799, Some(OperandSize::Dword), None)), operand4: Some(Literal8(68)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 69, 95, 31, 148, 203, 255, 159, 182, 43, 68], OperandSize::Dword)
}

#[test]
fn vpcmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM2)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 109, 71, 31, 234, 19], OperandSize::Qword)
}

#[test]
fn vpcmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K5)), operand2: Some(Direct(ZMM29)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(46)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 21, 71, 31, 43, 46], OperandSize::Qword)
}

#[test]
fn vpcmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 243, 101, 94, 31, 19, 97], OperandSize::Qword)
}

