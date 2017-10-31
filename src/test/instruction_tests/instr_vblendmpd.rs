use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vblendmpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 101, 240], OperandSize::Dword)
}

#[test]
fn vblendmpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 78563846, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 142, 101, 140, 206, 6, 202, 174, 4], OperandSize::Dword)
}

#[test]
fn vblendmpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Eight, 278189206, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 157, 101, 148, 246, 150, 212, 148, 16], OperandSize::Dword)
}

#[test]
fn vblendmpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 162, 237, 129, 101, 222], OperandSize::Qword)
}

#[test]
fn vblendmpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM17)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1522868028, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 245, 130, 101, 36, 205, 60, 31, 197, 90], OperandSize::Qword)
}

#[test]
fn vblendmpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 957382552, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 197, 151, 101, 180, 80, 152, 127, 16, 57], OperandSize::Qword)
}

#[test]
fn vblendmpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 174, 101, 204], OperandSize::Dword)
}

#[test]
fn vblendmpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 169, 101, 41], OperandSize::Dword)
}

#[test]
fn vblendmpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1119334476, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 191, 101, 60, 205, 76, 176, 183, 66], OperandSize::Dword)
}

#[test]
fn vblendmpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM30)), operand3: Some(Direct(YMM31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 18, 141, 165, 101, 239], OperandSize::Qword)
}

#[test]
fn vblendmpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM26)), operand3: Some(Indirect(RCX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 173, 165, 101, 49], OperandSize::Qword)
}

#[test]
fn vblendmpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1166766004, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 221, 186, 101, 36, 125, 180, 111, 139, 69], OperandSize::Qword)
}

#[test]
fn vblendmpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 204, 101, 221], OperandSize::Dword)
}

#[test]
fn vblendmpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(EAX, 1511324223, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 201, 101, 144, 63, 250, 20, 90], OperandSize::Dword)
}

#[test]
fn vblendmpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 205, 221, 101, 36, 218], OperandSize::Dword)
}

#[test]
fn vblendmpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM30)), operand3: Some(Direct(ZMM9)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 82, 141, 198, 101, 209], OperandSize::Qword)
}

#[test]
fn vblendmpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Two, 636975172, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 133, 203, 101, 156, 113, 68, 120, 247, 37], OperandSize::Qword)
}

#[test]
fn vblendmpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VBLENDMPD, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledIndexed(RBX, RBX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 149, 215, 101, 20, 219], OperandSize::Qword)
}

