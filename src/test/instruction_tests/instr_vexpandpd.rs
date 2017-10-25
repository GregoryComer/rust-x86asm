use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexpandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 143, 136, 237], OperandSize::Dword)
}

#[test]
fn vexpandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 1880044289, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 136, 172, 86, 1, 51, 15, 112], OperandSize::Dword)
}

#[test]
fn vexpandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 253, 142, 136, 242], OperandSize::Qword)
}

#[test]
fn vexpandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM29)), operand2: Some(IndirectDisplaced(RBX, 1347859998, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 98, 253, 140, 136, 171, 30, 182, 86, 80], OperandSize::Qword)
}

#[test]
fn vexpandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 136, 201], OperandSize::Dword)
}

#[test]
fn vexpandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Eight, 1117276777, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 136, 188, 250, 105, 74, 152, 66], OperandSize::Dword)
}

#[test]
fn vexpandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM10)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 66, 253, 173, 136, 242], OperandSize::Qword)
}

#[test]
fn vexpandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM13)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 253, 169, 136, 40], OperandSize::Qword)
}

#[test]
fn vexpandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 136, 253], OperandSize::Dword)
}

#[test]
fn vexpandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM0)), operand2: Some(IndirectScaledIndexedDisplaced(ECX, ECX, Four, 720707228, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 136, 132, 137, 156, 30, 245, 42], OperandSize::Dword)
}

#[test]
fn vexpandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 253, 201, 136, 210], OperandSize::Qword)
}

#[test]
fn vexpandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 253, 201, 136, 48], OperandSize::Qword)
}

