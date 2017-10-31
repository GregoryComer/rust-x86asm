use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vexpandpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 139, 136, 214], OperandSize::Dword)
}

#[test]
fn vexpandpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM5)), operand2: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 136, 42], OperandSize::Dword)
}

#[test]
fn vexpandpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 139, 136, 208], OperandSize::Qword)
}

#[test]
fn vexpandpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RAX, 459145660, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 141, 136, 128, 188, 1, 94, 27], OperandSize::Qword)
}

#[test]
fn vexpandpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 171, 136, 254], OperandSize::Dword)
}

#[test]
fn vexpandpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1090702290, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 253, 173, 136, 36, 213, 210, 203, 2, 65], OperandSize::Dword)
}

#[test]
fn vexpandpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 253, 173, 136, 232], OperandSize::Qword)
}

#[test]
fn vexpandpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledIndexed(RCX, RDX, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 175, 136, 44, 145], OperandSize::Qword)
}

#[test]
fn vexpandpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 253, 203, 136, 244], OperandSize::Dword)
}

#[test]
fn vexpandpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM3)), operand2: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 136, 24], OperandSize::Dword)
}

#[test]
fn vexpandpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 253, 203, 136, 226], OperandSize::Qword)
}

#[test]
fn vexpandpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VEXPANDPD, operand1: Some(Direct(ZMM0)), operand2: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 202, 136, 1], OperandSize::Qword)
}

