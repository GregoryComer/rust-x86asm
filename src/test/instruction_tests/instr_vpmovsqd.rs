use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpmovsqd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 126, 140, 37, 244], OperandSize::Dword)
}

#[test]
fn vpmovsqd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Indirect(ESI, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 37, 30], OperandSize::Dword)
}

#[test]
fn vpmovsqd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 130, 126, 138, 37, 203], OperandSize::Qword)
}

#[test]
fn vpmovsqd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledDisplaced(RDI, Two, 630034736, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 8, 37, 20, 125, 48, 145, 141, 37], OperandSize::Qword)
}

#[test]
fn vpmovsqd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 126, 175, 37, 217], OperandSize::Dword)
}

#[test]
fn vpmovsqd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(ESI, 1930083895, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 37, 174, 55, 190, 10, 115], OperandSize::Dword)
}

#[test]
fn vpmovsqd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(YMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 126, 171, 37, 200], OperandSize::Qword)
}

#[test]
fn vpmovsqd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(RBX, 557749851, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 40, 37, 171, 91, 150, 62, 33], OperandSize::Qword)
}

#[test]
fn vpmovsqd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 126, 201, 37, 208], OperandSize::Dword)
}

#[test]
fn vpmovsqd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectDisplaced(ESI, 650249294, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 126, 72, 37, 182, 78, 4, 194, 38], OperandSize::Dword)
}

#[test]
fn vpmovsqd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(ZMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 126, 207, 37, 253], OperandSize::Qword)
}

#[test]
fn vpmovsqd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMOVSQD, operand1: Some(IndirectScaledIndexed(RBX, RDX, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(ZMM26)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 98, 126, 72, 37, 20, 83], OperandSize::Qword)
}

