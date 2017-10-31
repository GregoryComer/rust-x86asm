use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpblendmb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 101, 137, 102, 197], OperandSize::Dword)
}

#[test]
fn vpblendmb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EAX, 1464047693, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 102, 168, 77, 152, 67, 87], OperandSize::Dword)
}

#[test]
fn vpblendmb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 101, 129, 102, 192], OperandSize::Qword)
}

#[test]
fn vpblendmb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM29)), operand3: Some(IndirectScaledIndexed(RBX, RSI, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 21, 133, 102, 4, 179], OperandSize::Qword)
}

#[test]
fn vpblendmb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 109, 173, 102, 233], OperandSize::Dword)
}

#[test]
fn vpblendmb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1160924647, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 93, 174, 102, 4, 69, 231, 77, 50, 69], OperandSize::Dword)
}

#[test]
fn vpblendmb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 82, 109, 169, 102, 245], OperandSize::Qword)
}

#[test]
fn vpblendmb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(YMM20)), operand2: Some(Direct(YMM30)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Two, 104239132, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 13, 162, 102, 164, 86, 28, 144, 54, 6], OperandSize::Qword)
}

#[test]
fn vpblendmb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 204, 102, 249], OperandSize::Dword)
}

#[test]
fn vpblendmb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM2)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 109, 207, 102, 1], OperandSize::Dword)
}

#[test]
fn vpblendmb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM10)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 45, 207, 102, 219], OperandSize::Qword)
}

#[test]
fn vpblendmb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDMB, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM18)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 943195225, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 109, 193, 102, 156, 178, 89, 4, 56, 56], OperandSize::Qword)
}

