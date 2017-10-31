use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vscalefps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 117, 138, 44, 236], OperandSize::Dword)
}

#[test]
fn vscalefps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 69, 142, 44, 44, 192], OperandSize::Dword)
}

#[test]
fn vscalefps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 77, 155, 44, 2], OperandSize::Dword)
}

#[test]
fn vscalefps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 162, 45, 139, 44, 198], OperandSize::Qword)
}

#[test]
fn vscalefps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM30)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 85, 134, 44, 52, 206], OperandSize::Qword)
}

#[test]
fn vscalefps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectDisplaced(RSI, 1306018745, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 29, 147, 44, 158, 185, 67, 216, 77], OperandSize::Qword)
}

#[test]
fn vscalefps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 172, 44, 230], OperandSize::Dword)
}

#[test]
fn vscalefps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 44, 63], OperandSize::Dword)
}

#[test]
fn vscalefps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Eight, 895097413, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 188, 44, 148, 243, 69, 26, 90, 53], OperandSize::Dword)
}

#[test]
fn vscalefps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 210, 5, 170, 44, 246], OperandSize::Qword)
}

#[test]
fn vscalefps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM22)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1316093683, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 77, 167, 44, 36, 125, 243, 254, 113, 78], OperandSize::Qword)
}

#[test]
fn vscalefps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RAX, 884791640, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 125, 183, 44, 184, 88, 217, 188, 52], OperandSize::Qword)
}

#[test]
fn vscalefps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 109, 251, 44, 248], OperandSize::Dword)
}

#[test]
fn vscalefps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 688542436, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 69, 205, 44, 36, 221, 228, 82, 10, 41], OperandSize::Dword)
}

#[test]
fn vscalefps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 220, 44, 43], OperandSize::Dword)
}

#[test]
fn vscalefps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM21)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 194, 85, 182, 44, 221], OperandSize::Qword)
}

#[test]
fn vscalefps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Eight, 1288001397, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 226, 21, 207, 44, 180, 206, 117, 87, 197, 76], OperandSize::Qword)
}

#[test]
fn vscalefps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSCALEFPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RDI, RDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 45, 210, 44, 20, 255], OperandSize::Qword)
}

