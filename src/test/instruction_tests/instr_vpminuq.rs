use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpminuq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 237, 137, 59, 238], OperandSize::Dword)
}

#[test]
fn vpminuq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 59, 19], OperandSize::Dword)
}

#[test]
fn vpminuq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 197, 158, 59, 3], OperandSize::Dword)
}

#[test]
fn vpminuq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM14)), operand3: Some(Direct(XMM26)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 141, 138, 59, 242], OperandSize::Qword)
}

#[test]
fn vpminuq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 1068702210, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 213, 133, 59, 148, 217, 2, 26, 179, 63], OperandSize::Qword)
}

#[test]
fn vpminuq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM28)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 157, 149, 59, 48], OperandSize::Qword)
}

#[test]
fn vpminuq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 59, 227], OperandSize::Dword)
}

#[test]
fn vpminuq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EDX, 2101674017, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 169, 59, 146, 33, 0, 69, 125], OperandSize::Dword)
}

#[test]
fn vpminuq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EAX, EDX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 186, 59, 36, 144], OperandSize::Dword)
}

#[test]
fn vpminuq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM10)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 173, 171, 59, 237], OperandSize::Qword)
}

#[test]
fn vpminuq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM24)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1882184046, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 189, 165, 59, 4, 197, 110, 217, 47, 112], OperandSize::Qword)
}

#[test]
fn vpminuq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RDX, RSI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 190, 59, 36, 114], OperandSize::Qword)
}

#[test]
fn vpminuq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 229, 201, 59, 201], OperandSize::Dword)
}

#[test]
fn vpminuq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 59, 52, 127], OperandSize::Dword)
}

#[test]
fn vpminuq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 221, 59, 39], OperandSize::Dword)
}

#[test]
fn vpminuq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 213, 207, 59, 235], OperandSize::Qword)
}

#[test]
fn vpminuq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM10)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 2093814942, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 205, 202, 59, 20, 205, 158, 20, 205, 124], OperandSize::Qword)
}

#[test]
fn vpminuq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMINUQ, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM29)), operand3: Some(IndirectScaledDisplaced(RDX, Four, 977326529, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 149, 213, 59, 20, 149, 193, 209, 64, 58], OperandSize::Qword)
}

