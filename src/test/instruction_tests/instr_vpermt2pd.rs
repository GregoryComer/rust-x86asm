use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpermt2pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 245, 138, 127, 228], OperandSize::Dword)
}

#[test]
fn vpermt2pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Eight, 1904636948, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 197, 139, 127, 188, 202, 20, 116, 134, 113], OperandSize::Dword)
}

#[test]
fn vpermt2pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 155, 127, 60, 182], OperandSize::Dword)
}

#[test]
fn vpermt2pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 229, 131, 127, 235], OperandSize::Qword)
}

#[test]
fn vpermt2pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 173, 142, 127, 51], OperandSize::Qword)
}

#[test]
fn vpermt2pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RAX, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 141, 158, 127, 20, 72], OperandSize::Qword)
}

#[test]
fn vpermt2pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 245, 169, 127, 250], OperandSize::Dword)
}

#[test]
fn vpermt2pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(ESI, Four, 2132073933, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 170, 127, 12, 181, 205, 221, 20, 127], OperandSize::Dword)
}

#[test]
fn vpermt2pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EDX, 962691672, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 127, 162, 88, 130, 97, 57], OperandSize::Dword)
}

#[test]
fn vpermt2pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 253, 172, 127, 201], OperandSize::Qword)
}

#[test]
fn vpermt2pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RSI, Eight, 1055299111, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 173, 163, 127, 180, 243, 39, 150, 230, 62], OperandSize::Qword)
}

#[test]
fn vpermt2pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(YMM28)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(RSI, 503818276, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 205, 189, 127, 166, 36, 168, 7, 30], OperandSize::Qword)
}

#[test]
fn vpermt2pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 197, 204, 127, 222], OperandSize::Dword)
}

#[test]
fn vpermt2pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 245, 206, 127, 47], OperandSize::Dword)
}

#[test]
fn vpermt2pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EAX, EBX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 218, 127, 52, 88], OperandSize::Dword)
}

#[test]
fn vpermt2pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM27)), operand3: Some(Direct(ZMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 165, 199, 127, 226], OperandSize::Qword)
}

#[test]
fn vpermt2pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexed(RCX, RBX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 205, 199, 127, 4, 89], OperandSize::Qword)
}

#[test]
fn vpermt2pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMT2PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM16)), operand3: Some(Indirect(RDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 209, 127, 26], OperandSize::Qword)
}

