use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermi2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 119, 198], OperandSize::Dword)
}

#[test]
fn vpermi2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 509674414, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 229, 143, 119, 169, 174, 3, 97, 30], OperandSize::Dword)
}

#[test]
fn vpermi2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 1553220246, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 156, 119, 12, 205, 150, 66, 148, 92], OperandSize::Dword)
}

#[test]
fn vpermi2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM23)), operand3: Some(Direct(XMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 34, 197, 134, 119, 196], OperandSize::Qword)
}

#[test]
fn vpermi2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM21)), operand2: Some(Direct(XMM11)), operand3: Some(IndirectScaledIndexed(RAX, RSI, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 165, 140, 119, 44, 112], OperandSize::Qword)
}

#[test]
fn vpermi2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 315578531, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 213, 150, 119, 28, 157, 163, 88, 207, 18], OperandSize::Qword)
}

#[test]
fn vpermi2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 119, 202], OperandSize::Dword)
}

#[test]
fn vpermi2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 119, 7], OperandSize::Dword)
}

#[test]
fn vpermi2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 222099750, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 189, 119, 52, 85, 38, 249, 60, 13], OperandSize::Dword)
}

#[test]
fn vpermi2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 245, 169, 119, 238], OperandSize::Qword)
}

#[test]
fn vpermi2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 695054641, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 157, 165, 119, 20, 125, 49, 177, 109, 41], OperandSize::Qword)
}

#[test]
fn vpermi2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 182, 119, 52, 202], OperandSize::Qword)
}

#[test]
fn vpermi2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 119, 241], OperandSize::Dword)
}

#[test]
fn vpermi2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 205, 202, 119, 23], OperandSize::Dword)
}

#[test]
fn vpermi2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 219, 119, 26], OperandSize::Dword)
}

#[test]
fn vpermi2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM19)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 178, 245, 196, 119, 243], OperandSize::Qword)
}

#[test]
fn vpermi2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM13)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 2060941641, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 149, 207, 119, 4, 117, 73, 121, 215, 122], OperandSize::Qword)
}

#[test]
fn vpermi2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMI2PD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectDisplaced(RCX, 878328081, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 133, 212, 119, 145, 17, 57, 90, 52], OperandSize::Qword)
}

