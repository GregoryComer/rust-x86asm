use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcompressq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 139, 193], OperandSize::Dword)
}

#[test]
fn vpcompressq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 8, 139, 46], OperandSize::Dword)
}

#[test]
fn vpcompressq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(XMM14)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 253, 142, 139, 254], OperandSize::Qword)
}

#[test]
fn vpcompressq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledDisplaced(RAX, Eight, 1711998754, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 253, 8, 139, 20, 197, 34, 7, 11, 102], OperandSize::Qword)
}

#[test]
fn vpcompressq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 139, 198], OperandSize::Dword)
}

#[test]
fn vpcompressq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 40, 139, 0], OperandSize::Dword)
}

#[test]
fn vpcompressq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM15)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 82, 253, 175, 139, 253], OperandSize::Qword)
}

#[test]
fn vpcompressq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 253, 40, 139, 3], OperandSize::Qword)
}

#[test]
fn vpcompressq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 206, 139, 254], OperandSize::Dword)
}

#[test]
fn vpcompressq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(IndirectScaledDisplaced(EAX, Eight, 1614424901, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 242, 253, 72, 139, 36, 197, 69, 43, 58, 96], OperandSize::Dword)
}

#[test]
fn vpcompressq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 253, 205, 139, 223], OperandSize::Qword)
}

#[test]
fn vpcompressq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCOMPRESSQ, operand1: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 226, 253, 72, 139, 39], OperandSize::Qword)
}

