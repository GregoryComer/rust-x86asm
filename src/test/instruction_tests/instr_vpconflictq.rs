use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpconflictq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 140, 196, 222], OperandSize::Dword)
}

#[test]
fn vpconflictq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 196, 20, 198], OperandSize::Dword)
}

#[test]
fn vpconflictq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EAX, 1986065515, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 157, 196, 168, 107, 244, 96, 118], OperandSize::Dword)
}

#[test]
fn vpconflictq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 196, 255], OperandSize::Qword)
}

#[test]
fn vpconflictq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 196, 36, 74], OperandSize::Qword)
}

#[test]
fn vpconflictq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Four, 586496978, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 155, 196, 188, 184, 210, 59, 245, 34], OperandSize::Qword)
}

#[test]
fn vpconflictq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 196, 249], OperandSize::Dword)
}

#[test]
fn vpconflictq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EBX, Two, 1744822432, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 172, 196, 140, 95, 160, 224, 255, 103], OperandSize::Dword)
}

#[test]
fn vpconflictq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, EDX, Four, 271279815, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 185, 196, 132, 145, 199, 102, 43, 16], OperandSize::Dword)
}

#[test]
fn vpconflictq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 196, 237], OperandSize::Qword)
}

#[test]
fn vpconflictq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM13)), operand2: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 114, 253, 170, 196, 46], OperandSize::Qword)
}

#[test]
fn vpconflictq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM9)), operand2: Some(IndirectDisplaced(RDX, 1178828072, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 190, 196, 138, 40, 125, 67, 70], OperandSize::Qword)
}

#[test]
fn vpconflictq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 196, 253], OperandSize::Dword)
}

#[test]
fn vpconflictq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(ECX, 448336275, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 205, 196, 145, 147, 17, 185, 26], OperandSize::Dword)
}

#[test]
fn vpconflictq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 196, 3], OperandSize::Dword)
}

#[test]
fn vpconflictq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 253, 203, 196, 250], OperandSize::Qword)
}

#[test]
fn vpconflictq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM24)), operand2: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 253, 202, 196, 4, 183], OperandSize::Qword)
}

#[test]
fn vpconflictq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM23)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 566409595, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 253, 221, 196, 188, 86, 123, 185, 194, 33], OperandSize::Qword)
}

