use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpexpandd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 125, 138, 137, 238], OperandSize::Dword)
}

#[test]
fn vpexpandd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 142, 137, 3], OperandSize::Dword)
}

#[test]
fn vpexpandd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 125, 138, 137, 246], OperandSize::Qword)
}

#[test]
fn vpexpandd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(XMM13)), operand2: Some(IndirectDisplaced(RBX, 1470903587, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 125, 137, 137, 171, 35, 53, 172, 87], OperandSize::Qword)
}

#[test]
fn vpexpandd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 137, 236], OperandSize::Dword)
}

#[test]
fn vpexpandd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM2)), operand2: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 125, 173, 137, 22], OperandSize::Dword)
}

#[test]
fn vpexpandd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM21)), operand2: Some(Direct(YMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 130, 125, 172, 137, 234], OperandSize::Qword)
}

#[test]
fn vpexpandd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(YMM24)), operand2: Some(IndirectDisplaced(RSI, 21374796, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 125, 171, 137, 134, 76, 39, 70, 1], OperandSize::Qword)
}

#[test]
fn vpexpandd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 125, 201, 137, 217], OperandSize::Dword)
}

#[test]
fn vpexpandd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 125, 206, 137, 28, 90], OperandSize::Dword)
}

#[test]
fn vpexpandd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 204, 137, 253], OperandSize::Qword)
}

#[test]
fn vpexpandd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPEXPANDD, operand1: Some(Direct(ZMM10)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 114, 125, 204, 137, 20, 243], OperandSize::Qword)
}

