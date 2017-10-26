use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpconflictq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 138, 196, 255], OperandSize::Dword)
}

#[test]
fn vpconflictq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDI, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 196, 31], OperandSize::Dword)
}

#[test]
fn vpconflictq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 1719598132, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 154, 196, 60, 125, 52, 252, 126, 102], OperandSize::Dword)
}

#[test]
fn vpconflictq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 253, 140, 196, 222], OperandSize::Qword)
}

#[test]
fn vpconflictq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM14)), operand2: Some(IndirectScaledDisplaced(RBX, Eight, 1639653854, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 137, 196, 52, 221, 222, 33, 187, 97], OperandSize::Qword)
}

#[test]
fn vpconflictq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1547251941, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 153, 196, 36, 85, 229, 48, 57, 92], OperandSize::Qword)
}

#[test]
fn vpconflictq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 196, 192], OperandSize::Dword)
}

#[test]
fn vpconflictq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexedDisplaced(EDI, EDX, Eight, 434928084, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 174, 196, 148, 215, 212, 121, 236, 25], OperandSize::Dword)
}

#[test]
fn vpconflictq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 186, 196, 36, 79], OperandSize::Dword)
}

#[test]
fn vpconflictq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 34, 253, 171, 196, 255], OperandSize::Qword)
}

#[test]
fn vpconflictq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM25)), operand2: Some(IndirectDisplaced(RCX, 681971985, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 174, 196, 137, 17, 17, 166, 40], OperandSize::Qword)
}

#[test]
fn vpconflictq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(YMM9)), operand2: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 253, 190, 196, 14], OperandSize::Qword)
}

#[test]
fn vpconflictq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 196, 245], OperandSize::Dword)
}

#[test]
fn vpconflictq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 2012181063, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 196, 4, 181, 71, 114, 239, 119], OperandSize::Dword)
}

#[test]
fn vpconflictq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectDisplaced(EDI, 1913639645, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 220, 196, 151, 221, 210, 15, 114], OperandSize::Dword)
}

#[test]
fn vpconflictq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 253, 205, 196, 199], OperandSize::Qword)
}

#[test]
fn vpconflictq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM26)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 98, 253, 206, 196, 17], OperandSize::Qword)
}

#[test]
fn vpconflictq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCONFLICTQ, operand1: Some(Direct(ZMM8)), operand2: Some(IndirectScaledDisplaced(RDX, Two, 1514390308, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 253, 221, 196, 4, 85, 36, 195, 67, 90], OperandSize::Qword)
}

