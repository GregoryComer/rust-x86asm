use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 100, 230], OperandSize::Dword)
}

#[test]
fn vpblendmq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexed(EDI, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 237, 141, 100, 28, 135], OperandSize::Dword)
}

#[test]
fn vpblendmq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Eight, 355297586, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 154, 100, 164, 206, 50, 105, 45, 21], OperandSize::Dword)
}

#[test]
fn vpblendmq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 237, 141, 100, 237], OperandSize::Qword)
}

#[test]
fn vpblendmq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1417657024, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 157, 130, 100, 28, 117, 192, 186, 127, 84], OperandSize::Qword)
}

#[test]
fn vpblendmq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 221, 157, 100, 44, 207], OperandSize::Qword)
}

#[test]
fn vpblendmq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 169, 100, 212], OperandSize::Dword)
}

#[test]
fn vpblendmq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(EAX, 1522907944, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 100, 176, 40, 187, 197, 90], OperandSize::Dword)
}

#[test]
fn vpblendmq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(ECX, 902908102, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 197, 186, 100, 161, 198, 72, 209, 53], OperandSize::Dword)
}

#[test]
fn vpblendmq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 164, 100, 240], OperandSize::Qword)
}

#[test]
fn vpblendmq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM11)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 165, 173, 100, 52, 78], OperandSize::Qword)
}

#[test]
fn vpblendmq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM29)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 149, 177, 100, 52, 67], OperandSize::Qword)
}

#[test]
fn vpblendmq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 213, 202, 100, 201], OperandSize::Dword)
}

#[test]
fn vpblendmq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Two, 1975636423, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 204, 100, 132, 115, 199, 209, 193, 117], OperandSize::Dword)
}

#[test]
fn vpblendmq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Four, 1259826670, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 222, 100, 4, 157, 238, 109, 23, 75], OperandSize::Dword)
}

#[test]
fn vpblendmq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM24)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 189, 197, 100, 230], OperandSize::Qword)
}

#[test]
fn vpblendmq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM28)), operand3: Some(IndirectDisplaced(RSI, 1870158351, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 157, 199, 100, 158, 15, 90, 120, 111], OperandSize::Qword)
}

#[test]
fn vpblendmq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Eight, 1600288935, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 219, 100, 188, 215, 167, 120, 98, 95], OperandSize::Qword)
}

