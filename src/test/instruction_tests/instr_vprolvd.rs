use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vprolvd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 137, 21, 235], OperandSize::Dword)
}

#[test]
fn vprolvd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 963260082, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 101, 141, 21, 164, 90, 178, 46, 106, 57], OperandSize::Dword)
}

#[test]
fn vprolvd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Four, 1566722016, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 109, 153, 21, 148, 184, 224, 71, 98, 93], OperandSize::Dword)
}

#[test]
fn vprolvd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 146, 109, 138, 21, 224], OperandSize::Qword)
}

#[test]
fn vprolvd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 912091024, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 125, 132, 21, 12, 253, 144, 103, 93, 54], OperandSize::Qword)
}

#[test]
fn vprolvd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 37, 153, 21, 60, 89], OperandSize::Qword)
}

#[test]
fn vprolvd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 77, 170, 21, 215], OperandSize::Dword)
}

#[test]
fn vprolvd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 101, 175, 21, 16], OperandSize::Dword)
}

#[test]
fn vprolvd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 125, 187, 21, 12, 250], OperandSize::Dword)
}

#[test]
fn vprolvd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 45, 175, 21, 234], OperandSize::Qword)
}

#[test]
fn vprolvd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM8)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RSI, Two, 633643425, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 61, 169, 21, 148, 119, 161, 161, 196, 37], OperandSize::Qword)
}

#[test]
fn vprolvd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(YMM15)), operand2: Some(Direct(YMM19)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 101, 182, 21, 59], OperandSize::Qword)
}

#[test]
fn vprolvd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 101, 204, 21, 202], OperandSize::Dword)
}

#[test]
fn vprolvd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 205, 21, 31], OperandSize::Dword)
}

#[test]
fn vprolvd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 117, 221, 21, 60, 82], OperandSize::Dword)
}

#[test]
fn vprolvd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 162, 109, 206, 21, 195], OperandSize::Qword)
}

#[test]
fn vprolvd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM15)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 631946343, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 5, 204, 21, 156, 178, 103, 188, 170, 37], OperandSize::Qword)
}

#[test]
fn vprolvd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPROLVD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 351113085, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 101, 222, 21, 140, 184, 125, 143, 237, 20], OperandSize::Qword)
}

